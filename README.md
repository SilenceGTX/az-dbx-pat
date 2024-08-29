# az-dbx-pat

az-dbx-pat is a command line tool to generate Personal Access Tokens (PAT) for Azure Databricks.

## Installation
For Linux system, put the binary file `az-dbx-pat` in `/usr/local/bin` or any other directory in your PATH.
For Windows system, put the binary file `az-dbx-pat.exe` in a directory in your PATH.

## How to use
Please ensure you have performed `az login` or set up relevant environment variables to authenticate with Azure first.  
Then simply run the following command to generate a PAT for Azure Databricks:
```
az-dbx-pat generate -u https://<azure_databricks_url>
```
The Azure Databricks URL should be the URL of the Azure Databricks workspace you want to generate the PAT for, usually in `https://adb-1234567890123456.1.azuredatabricks.net` format.  

## Behind the scenes
The generation of PAT is performed by sending a POST request to the Azure Databricks REST API. The PAT is then returned in the response body.  
In the request hearder, the Azure access token is required, which is obtained from Azure default credential chain.  
