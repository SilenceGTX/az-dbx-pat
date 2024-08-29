# az-dbx-pat

az-dbx-pat is a command line tool to generate Personal Access Tokens (PAT) for Azure Databricks.

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
