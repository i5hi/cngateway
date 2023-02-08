use crate::e::{ErrorKind, S5Error};
use reqwest::{
    self,
    header::{HeaderMap, AUTHORIZATION},
    Certificate,
};
use serde_derive::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ResponseType {
    Create(CreateBatcherResponse),
    Update(UpdateBatcherResponse),
    Add(BatchInfoResponse),
    Remove(BatchInfoResponse),
    Get(BatchInfoResponse),
    GetDetail(BatchDetailResponse),
    Spend(BatchSpendResponse),
    List(ListBatchersResponse),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BatcherRespoonse {
    pub result: ResponseType,
    pub error: Option<String>,
}
impl BatcherRespoonse {
    pub fn structify(stringified: &str) -> Result<BatcherRespoonse, S5Error> {
        match serde_json::from_str(stringified) {
            Ok(result) => Ok(result),
            Err(_) => Err(S5Error::new(
                ErrorKind::Internal,
                "Error stringifying BatcherRespoonse",
            )),
        }
    }
}
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateBatcherRequest {
    pub batcher_label: String,
    pub conf_target: u64,
}
impl CreateBatcherRequest {
    pub fn new(batcher_label: String, conf_target: u64) -> Self {
        CreateBatcherRequest {
            batcher_label,
            conf_target,
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateBatcherResponse {
    pub batcher_id: u64,
}
impl CreateBatcherResponse {
    pub fn structify(stringified: &str) -> Result<CreateBatcherResponse, S5Error> {
        match serde_json::from_str(stringified) {
            Ok(result) => Ok(result),
            Err(_) => Err(S5Error::new(
                ErrorKind::Internal,
                "Error stringifying CreateBatcherResponse",
            )),
        }
    }
}
///Used to create a batching template, by setting a label and a default confTarget.
pub async fn createbatcher(
    host: String,
    jwt: String,
    cert: Certificate,
    body: CreateBatcherRequest,
) -> Result<CreateBatcherResponse, String> {
    let full_url: String = format!("https://{}/v0/createbatcher", host).to_string();
    let mut headers = HeaderMap::new();
    headers.insert(AUTHORIZATION, format!("Bearer {}", jwt).parse().unwrap());

    let client = reqwest::Client::builder().add_root_certificate(cert);
    let client = match client.default_headers(headers).build() {
        Ok(result) => result,
        Err(e) => return Err(e.to_string()),
    };
    let response = match client.post(&full_url).json(&body).send().await {
        Ok(response) => match response.text().await {
            Ok(text) => {
                println!("{}", text);
                match BatcherRespoonse::structify(&text) {
                    Ok(result) => result,
                    Err(e) => return Err(e.message),
                }
            }
            Err(e) => return Err(e.to_string()),
        },
        Err(e) => return Err(e.to_string()),
    };
    if response.error.is_some() {
        return Err(response.error.unwrap());
    } else {
        match response.result {
            ResponseType::Create(res) => return Ok(res),
            _ => unreachable!("IMPOSSIBRU!"),
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateBatcherRequest {
    pub batcher_label: Option<String>,
    pub batcher_id: Option<String>,
    pub conf_target: u64,
}
impl UpdateBatcherRequest {
    pub fn new(
        batcher_label: Option<String>,
        batcher_id: Option<String>,
        conf_target: u64,
    ) -> Self {
        UpdateBatcherRequest {
            batcher_label,
            batcher_id,
            conf_target,
        }
    }
}
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateBatcherResponse {
    pub batcher_label: String,
    pub batcher_id: String,
    pub conf_target: u64,
}
impl UpdateBatcherResponse {
    pub fn structify(stringified: &str) -> Result<UpdateBatcherResponse, S5Error> {
        match serde_json::from_str(stringified) {
            Ok(result) => Ok(result),
            Err(_) => Err(S5Error::new(
                ErrorKind::Internal,
                "Error stringifying UpdateBatcherResponse",
            )),
        }
    }
}
///Used to change batching template settings.
pub async fn updatebatcher(
    host: String,
    jwt: String,
    cert: Certificate,
    body: UpdateBatcherRequest,
) -> Result<UpdateBatcherResponse, String> {
    let full_url: String = format!("https://{}/v0/updatebatcher", host).to_string();
    let mut headers = HeaderMap::new();
    headers.insert(AUTHORIZATION, format!("Bearer {}", jwt).parse().unwrap());

    let client = reqwest::Client::builder().add_root_certificate(cert);
    let client = match client.default_headers(headers).build() {
        Ok(result) => result,
        Err(e) => return Err(e.to_string()),
    };
    let response = match client.post(&full_url).json(&body).send().await {
        Ok(response) => match response.text().await {
            Ok(text) => {
                println!("{}", text);
                match BatcherRespoonse::structify(&text) {
                    Ok(result) => result,
                    Err(e) => return Err(e.message),
                }
            }
            Err(e) => return Err(e.to_string()),
        },
        Err(e) => return Err(e.to_string()),
    };
    if response.error.is_some() {
        return Err(response.error.unwrap());
    } else {
        match response.result {
            ResponseType::Update(res) => return Ok(res),
            _ => unreachable!("IMPOSSIBRU!"),
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AddToBatchRequest {
    pub address: String,
    pub amount: f64,
    pub batcher_label: Option<String>,
    pub webhook_url: Option<String>,
}
impl AddToBatchRequest {
    pub fn new(
        address: String,
        amount: f64,
        batcher_label: Option<String>,
        webhook_url: Option<String>,
    ) -> Self {
        AddToBatchRequest {
            address,
            amount,
            batcher_label,
            webhook_url,
        }
    }
}
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BatchInfoResponse {
    pub batcher_id: u64,
    pub output_id: u64,
    pub nb_outputs: u64,
    pub oldest: String,
    pub total: f64,
}
impl BatchInfoResponse {
    pub fn structify(stringified: &str) -> Result<BatchInfoResponse, S5Error> {
        match serde_json::from_str(stringified) {
            Ok(result) => Ok(result),
            Err(_) => Err(S5Error::new(
                ErrorKind::Internal,
                "Error stringifying BatchInfoResponse",
            )),
        }
    }
}
///Inserts output information in the DB. Used when batchspend is called later.
pub async fn addtobatch(
    host: String,
    jwt: String,
    cert: Certificate,
    body: AddToBatchRequest,
) -> Result<BatchInfoResponse, String> {
    let full_url: String = format!("https://{}/v0/addtobatch", host).to_string();
    let mut headers = HeaderMap::new();
    headers.insert(AUTHORIZATION, format!("Bearer {}", jwt).parse().unwrap());

    let client = reqwest::Client::builder().add_root_certificate(cert);
    let client = match client.default_headers(headers).build() {
        Ok(result) => result,
        Err(e) => return Err(e.to_string()),
    };
    let response = match client.post(&full_url).json(&body).send().await {
        Ok(response) => match response.text().await {
            Ok(text) => {
                println!("{}", text);
                match BatcherRespoonse::structify(&text) {
                    Ok(result) => result,
                    Err(e) => return Err(e.message),
                }
            }
            Err(e) => return Err(e.to_string()),
        },
        Err(e) => return Err(e.to_string()),
    };
    if response.error.is_some() {
        return Err(response.error.unwrap());
    } else {
        match response.result {
            ResponseType::Add(res) => return Ok(res),
            _ => unreachable!("IMPOSSIBRU!"),
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RemoveFromBatchRequest {
    pub output_id: u64,
}
impl RemoveFromBatchRequest {
    pub fn new(output_id: u64) -> Self {
        RemoveFromBatchRequest { output_id }
    }
}
///Removes a previously added output scheduled for the next batch.
pub async fn removefrombatch(
    host: String,
    jwt: String,
    cert: Certificate,
    body: RemoveFromBatchRequest,
) -> Result<BatchInfoResponse, String> {
    let full_url: String = format!("https://{}/v0/removefrombatch", host).to_string();
    let mut headers = HeaderMap::new();
    headers.insert(AUTHORIZATION, format!("Bearer {}", jwt).parse().unwrap());

    let client = reqwest::Client::builder().add_root_certificate(cert);
    let client = match client.default_headers(headers).build() {
        Ok(result) => result,
        Err(e) => return Err(e.to_string()),
    };
    let response = match client.post(&full_url).json(&body).send().await {
        Ok(response) => match response.text().await {
            Ok(text) => {
                println!("{}", text);
                match BatcherRespoonse::structify(&text) {
                    Ok(result) => result,
                    Err(e) => return Err(e.message),
                }
            }
            Err(e) => return Err(e.to_string()),
        },
        Err(e) => return Err(e.to_string()),
    };
    if response.error.is_some() {
        return Err(response.error.unwrap());
    } else {
        match response.result {
            ResponseType::Remove(res) => return Ok(res),
            _ => unreachable!("IMPOSSIBRU!"),
        }
    }
}


#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetBatcherRequest {
    pub batcher_label: Option<String>,
    pub batcher_id: Option<String>,
}
impl GetBatcherRequest {
    pub fn new(batcher_label: Option<String>, batcher_id: Option<String>) -> Self {
        GetBatcherRequest {
            batcher_label,
            batcher_id,
        }
    }
}
///Will return current state/summary of the requested batching template.
pub async fn getbatcher(
    host: String,
    jwt: String,
    cert: Certificate,
    body: GetBatcherRequest,
) -> Result<BatchInfoResponse, String> {
    let full_url: String = format!("https://{}/v0/getbatcher", host).to_string();
    let mut headers = HeaderMap::new();
    headers.insert(AUTHORIZATION, format!("Bearer {}", jwt).parse().unwrap());

    let client = reqwest::Client::builder().add_root_certificate(cert);
    let client = match client.default_headers(headers).build() {
        Ok(result) => result,
        Err(e) => return Err(e.to_string()),
    };
    let response = match client.post(&full_url).json(&body).send().await {
        Ok(response) => match response.text().await {
            Ok(text) => {
                println!("{}", text);
                match BatcherRespoonse::structify(&text) {
                    Ok(result) => result,
                    Err(e) => return Err(e.message),
                }
            }
            Err(e) => return Err(e.to_string()),
        },
        Err(e) => return Err(e.to_string()),
    };
    if response.error.is_some() {
        return Err(response.error.unwrap());
    } else {
        match response.result {
            ResponseType::Get(res) => return Ok(res),
            _ => unreachable!("IMPOSSIBRU!"),
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BatchSpendRequest {
    pub batcher_label: Option<String>,
    pub batcher_id: Option<String>,
    pub conf_target: Option<u64>,
}
impl BatchSpendRequest {
    pub fn new(
        batcher_label: Option<String>,
        batcher_id: Option<String>,
        conf_target: Option<u64>,
    ) -> Self {
        BatchSpendRequest {
            batcher_label,
            batcher_id,
            conf_target,
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BatchSpendResponse {
    pub status: String,
    pub hash: String,
}
impl BatchSpendResponse {
    /// Used internally to convert api json string to native struct
    pub fn structify(stringified: &str) -> Result<BatchSpendResponse, S5Error> {
        match serde_json::from_str(stringified) {
            Ok(result) => Ok(result),
            Err(_) => Err(S5Error::new(
                ErrorKind::Internal,
                "Error stringifying BatchSpendResponse",
            )),
        }
    }
}

///Calls the sendmany RPC on spending wallet with the unspent "addtobatch" inserted outputs. 
///Will execute default batcher if no batcherId/batcherLabel supplied and default confTarget if no confTarget supplied.
pub async fn batchspend(
    host: String,
    jwt: String,
    cert: Certificate,
    body: BatchSpendRequest,
) -> Result<BatchSpendResponse, String> {
    let full_url: String = format!("https://{}/v0/batchspend", host).to_string();
    let mut headers = HeaderMap::new();
    headers.insert(AUTHORIZATION, format!("Bearer {}", jwt).parse().unwrap());

    let client = reqwest::Client::builder().add_root_certificate(cert);
    let client = match client.default_headers(headers).build() {
        Ok(result) => result,
        Err(e) => return Err(e.to_string()),
    };
    let response = match client.post(&full_url).json(&body).send().await {
        Ok(response) => match response.text().await {
            Ok(text) => {
                println!("{}", text);
                match BatcherRespoonse::structify(&text) {
                    Ok(result) => result,
                    Err(e) => return Err(e.message),
                }
            }
            Err(e) => return Err(e.to_string()),
        },
        Err(e) => return Err(e.to_string()),
    };
    if response.error.is_some() {
        return Err(response.error.unwrap());
    } else {
        match response.result {
            ResponseType::Spend(res) => return Ok(res),
            _ => unreachable!("IMPOSSIBRU!"),
        }
    }
}


///Will return current state/summary of the requested batching template.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetBatchDetailRequest {
    pub batcher_id: u64,
    pub batcher_label: Option<String>,
    pub txid: Option<String>,
}
impl GetBatchDetailRequest {
    pub fn new(batcher_id: u64, batcher_label: Option<String>, txid: Option<String>) -> Self {
        GetBatchDetailRequest {
            batcher_id,
            batcher_label,
            txid,
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Details {
    pub firstseen: i64,
    pub size: i64,
    pub vsize: i64,
    pub replaceable: bool,
    pub fee: f64,
}
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BatchDetailResponse {
    pub batcher_id: i64,
    pub batcher_label: String,
    pub conf_target: i64,
    pub nb_outputs: i64,
    pub oldest: i64,
    pub total: f64,
    pub txid: String,
    pub hash: String,
    pub details: Details,
    pub outputs: HashMap<String, f64>,
}
impl BatchDetailResponse {
    /// Used internally to convert api json string to native struct
    pub fn structify(stringified: &str) -> Result<BatchDetailResponse, S5Error> {
        match serde_json::from_str(stringified) {
            Ok(result) => Ok(result),
            Err(_) => Err(S5Error::new(
                ErrorKind::Internal,
                "Error stringifying BatchDetailResponse",
            )),
        }
    }
}

///Will return current state and details of the requested batch, including all outputs. 
///A batch is the combination of a batcher and an optional txid. 
/// If no txid is supplied, will return current non-yet-executed batch.
pub async fn getbatchdetails(
    host: String,
    jwt: String,
    cert: Certificate,
    body: GetBatchDetailRequest,
) -> Result<BatchDetailResponse, String> {
    let full_url: String = format!("https://{}/v0/getbatchdetails", host).to_string();
    let mut headers = HeaderMap::new();
    headers.insert(AUTHORIZATION, format!("Bearer {}", jwt).parse().unwrap());

    let client = reqwest::Client::builder().add_root_certificate(cert);
    let client = match client.default_headers(headers).build() {
        Ok(result) => result,
        Err(e) => return Err(e.to_string()),
    };
    let response = match client.post(&full_url).json(&body).send().await {
        Ok(response) => match response.text().await {
            Ok(text) => {
                println!("{}", text);
                match BatcherRespoonse::structify(&text) {
                    Ok(result) => result,
                    Err(e) => return Err(e.message),
                }
            }
            Err(e) => return Err(e.to_string()),
        },
        Err(e) => return Err(e.to_string()),
    };
    if response.error.is_some() {
        return Err(response.error.unwrap());
    } else {
        match response.result {
            ResponseType::GetDetail(res) => return Ok(res),
            _ => unreachable!("IMPOSSIBRU!"),
        }
    }
}


#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListBatchersResponse {
    pub batcher_id: i64,
    pub batcher_label: String,
    pub conf_target: i64,
    pub nb_outputs: i64,
    pub oldest: i64,
    pub total: f64,
}
impl ListBatchersResponse {
    /// Used internally to convert api json string to native struct
    pub fn structify(stringified: &str) -> Result<ListBatchersResponse, S5Error> {
        match serde_json::from_str(stringified) {
            Ok(result) => Ok(result),
            Err(_) => Err(S5Error::new(
                ErrorKind::Internal,
                "Error stringifying ListBatchersResponse",
            )),
        }
    }
}
///Will return a list of batch templates. 
///batcherId 1 is a default batcher created at installation time.

pub async fn listbatchers(
    host: String,
    jwt: String,
    cert: Certificate,
) -> Result<ListBatchersResponse, String> {
    let full_url: String = format!("https://{}/v0/listbatchers", host).to_string();
    let mut headers = HeaderMap::new();
    headers.insert(AUTHORIZATION, format!("Bearer {}", jwt).parse().unwrap());

    let client = reqwest::Client::builder().add_root_certificate(cert);
    let client = match client.default_headers(headers).build() {
        Ok(result) => result,
        Err(e) => return Err(e.to_string()),
    };
    let response = match client.get(&full_url).send().await {
        Ok(response) => match response.text().await {
            Ok(text) => {
                println!("{}", text);
                match BatcherRespoonse::structify(&text) {
                    Ok(result) => result,
                    Err(e) => return Err(e.message),
                }
            }
            Err(e) => return Err(e.to_string()),
        },
        Err(e) => return Err(e.to_string()),
    };
    if response.error.is_some() {
        return Err(response.error.unwrap());
    } else {
        match response.result {
            ResponseType::List(res) => return Ok(res),
            _ => unreachable!("IMPOSSIBRU!"),
        }
    }
}
// GET http://cyphernode:8888/batchspend
/*
RESPONSE{
  "status": "accepted",
  "hash": "af867c86000da76df7ddb1054b273ca9e034e8c89d049b5b2795f9f590f67648"
}

*/
