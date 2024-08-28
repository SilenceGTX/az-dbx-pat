use std::error::Error;
use std::process::exit;
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE};
use serde::{Deserialize, Serialize};
use tokio;
use clap::{App, Arg, SubCommand, value_parser};


#[derive(Serialize, Deserialize)]
struct DbxRequestBody {
    comment: String,
    lifetime_seconds: i32,
}

#[derive(Serialize, Deserialize)]
struct DbxTokenInfo {
    token_id: String,
    creation_time: u64,
    expiry_time: u64,
    comment: String,
}

#[derive(Serialize, Deserialize)]
struct DbxTokenResponse {
    token_value: String,
    token_info: DbxTokenInfo,
}


fn get_dbx_token_url(url: &str) -> String {
    // remove trailing slash
    let mut new_url = if url.ends_with("/") {
        url.strip_suffix("/").unwrap().to_string()
    } else {
        url.to_string()
    };

    // ensure it starts with "https://"
    if !new_url.starts_with("https://") {
        new_url = format!("https://{}", new_url);
    }

    // append the token request path
    new_url.push_str("/api/2.0/token/create");

    new_url
}


fn create_header(token : &str) -> Result<HeaderMap, Box<dyn Error>> {
    let mut headers = HeaderMap::new();
    let bearer_token = format!("Bearer {}", token);
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
    headers.insert(AUTHORIZATION, HeaderValue::from_str(&bearer_token)?);
    Ok(headers)
}


async fn get_azure_token() -> Result<String, Box<dyn Error>> {
    let credential = azure_identity::create_credential().unwrap();
    let scopes = vec!["2ff814a6-3304-4ab8-85cb-cd0e6f879c1d/.default"]; // the programmatic ID for Azure Databricks

    match credential.get_token(&scopes).await {
        Ok(token_response) => Ok(token_response.token.secret().to_string()),
        Err(e) => {
            eprintln!("Failed to get Azure token: {}", e);
            Err(e.into())
        }
    }
}


async fn get_pat(url: &str, header: HeaderMap, body: DbxRequestBody) -> Result<DbxTokenResponse, Box<dyn Error>> {
    let client = reqwest::Client::new();
    let resp = client.post(url)
        .headers(header)
        .json(&body)
        .send()
        .await?;

    if resp.status().is_success() {
        let token_response = resp.json().await?;
        Ok(token_response)
    } else {
        let err_msg = format!("Failed to get PAT: HTTP {:?}", resp.status());
        eprintln!("{}", &err_msg);
        Err(err_msg.into())
    }
}


async fn generate_dbx_pat(url: &str, lifetime: i32) -> Result<String, Box<dyn Error>> {
    let dbx_token_url = get_dbx_token_url(url);
    let dbx_body = DbxRequestBody {
        comment: "Generate Azure Databricks PAT".to_string(),
        lifetime_seconds: lifetime,
    };

    let azure_token = get_azure_token().await?;
    let header = create_header(&azure_token)?;

    let dbx_token_response = get_pat(&dbx_token_url, header, dbx_body).await?;
    Ok(dbx_token_response.token_value)
}


#[tokio::main]
async fn main() {
    let app = App::new("az-dbx-pat")
    .about("A CLI tool to generate Azure Databricks Personal Access Token (PAT)")
    .subcommand(SubCommand::with_name("test"))
        .about("This is just a test command")
    .subcommand(SubCommand::with_name("generate")
        .about("Generate Azure Databricks PAT")
        .arg(Arg::with_name("url")
            .short('u') // this should be a char
            .long("url")
            .value_name("URL")
            .help("Azure Databricks workspace URL, e.g. https://adb-12345678.123.azuredatabricks.net")
            .required(true)
            .takes_value(true))
        .arg(Arg::with_name("lifetime")
            .short('l')
            .long("lifetime")
            .value_name("LIFETIME")
            .help("Lifetime of the PAT in seconds")
            .required(false)
            .takes_value(true)
            .default_value("3600")
            .value_parser(value_parser!(i32))));

    let matches = app.get_matches();

    match matches.subcommand() {
        Some(("test", _)) => {
            println!("This is just a test command.");
        },
        
        Some(("generate", sub_m)) => {
            let url = sub_m.value_of("url").unwrap();
            let lifetime = *sub_m.get_one::<i32>("lifetime").expect("If 'lifetime' is not provided, it will use default value (3600s)");
            
            match generate_dbx_pat(url, lifetime).await {
                Ok(pat) => {
                    println!("{}", pat);
                    exit(0);
                },
                Err(e) => {
                    eprintln!("Error generating PAT: {}", e);
                    exit(1);
                }
            }
        },
        _ => {
            println!("Unknown command or no command entered.");
            exit(1);
        }
    }
}
