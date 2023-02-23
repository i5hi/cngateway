use crate::e::{ErrorKind, S5Error};
use reqwest::{
    self,
    header::{HeaderMap, AUTHORIZATION},
    Certificate,
};
use serde::{Deserialize, Serialize};


// POST http://cyphernode:8888/getnewaddress
// {"address_type":"bech32","label":"myLabel"}
/*
RESPONSE{
  "balance":1.51911837
}
*/
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum AddressType {
  Bech32,
  P2SH,
  Legacy
}
impl Default for AddressType{
  fn default()->Self{
    AddressType::Bech32
  }
}
impl ToString for AddressType{
  fn to_string(&self)->String{
      match self{
        AddressType::Bech32=>"bech32".to_string(),
        AddressType::P2SH=>"p2sh-segwit".to_string(),
        AddressType::Legacy=>"legacy".to_string(),
      }
  }

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AddressRequest {
    pub address_type: AddressType,
    pub label: String,
}
impl AddressRequest {
    /// Used internally to convert api json string to native struct
    pub fn from_str(stringified: &str) -> Result<AddressRequest, S5Error> {
        match serde_json::from_str(stringified) {
            Ok(result) => Ok(result),
            Err(e) => Err(S5Error::new(ErrorKind::Internal, &e.to_string())),
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Address {
    pub address: String,
}
impl Address {
    /// Used internally to convert api json string to native struct
    pub fn from_str(stringified: &str) -> Result<Address, S5Error> {
        match serde_json::from_str(stringified) {
            Ok(result) => Ok(result),
            Err(e) => Err(S5Error::new(ErrorKind::Internal, &e.to_string())),
        }
    }
}
///Returns the a new address from core wallet
pub async fn getnewaddress(
    host: String,
    jwt: String,
    cert: Certificate,
    body: AddressRequest,
) -> Result<Address, String> {
    let full_url: String = format!("https://{}/v0/getnewaddress", host).to_string();
    let mut headers = HeaderMap::new();
    headers.insert(AUTHORIZATION, format!("Bearer {}", jwt).parse().unwrap());

    let client = reqwest::Client::builder().add_root_certificate(cert);
    let client = match client.default_headers(headers).build() {
        Ok(result) => result,
        Err(e) => return Err(e.to_string()),
    };
    match client.post(&full_url).json(&body).send().await {
        Ok(response) => match response.text().await {
            Ok(text) => {
                println!("{}", text);
                match Address::from_str(&text) {
                    Ok(result) => Ok(result),
                    Err(e) => Err(e.message),
                }
            }
            Err(e) => Err(e.to_string()),
        },
        Err(e) => Err(e.to_string()),
    }
}

// GET http://cyphernode:8888/getmempoolinfo
/*
RESPONSE{
  "size": 25,
  "bytes": 5462,
  "usage": 34736,
  "maxmempool": 64000000,
  "mempoolminfee": 1e-05,
  "minrelaytxfee": 1e-05
}
*/
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MempoolInfo {
    pub size: i64,
    pub bytes: i64,
    pub usage: i64,
    pub maxmempool: i64,
    pub mempoolminfee: f64,
    pub minrelaytxfee: f64,
}
impl MempoolInfo {
    /// Used internally to convert api json string to native struct
    pub fn from_str(stringified: &str) -> Result<MempoolInfo, S5Error> {
        match serde_json::from_str(stringified) {
            Ok(result) => Ok(result),
            Err(e) => Err(S5Error::new(ErrorKind::Internal, &e.to_string())),
        }
    }
}
///Returns the mempool information of the Bitcoin node.
pub async fn getmempoolinfo(
    host: String,
    jwt: String,
    cert: Certificate,
) -> Result<MempoolInfo, String> {
    let full_url: String = format!("https://{}/v0/getmempoolinfo", host).to_string();
    let mut headers = HeaderMap::new();
    headers.insert(AUTHORIZATION, format!("Bearer {}", jwt).parse().unwrap());

    let client = reqwest::Client::builder().add_root_certificate(cert);
    let client = match client.default_headers(headers).build() {
        Ok(result) => result,
        Err(e) => return Err(e.to_string()),
    };
    match client.get(&full_url).send().await {
        Ok(response) => match response.text().await {
            Ok(text) => {
                println!("{}", text);
                match MempoolInfo::from_str(&text) {
                    Ok(result) => Ok(result),
                    Err(e) => Err(e.message),
                }
            }
            Err(e) => Err(e.to_string()),
        },
        Err(e) => Err(e.to_string()),
    }
}

// GET http://cyphernode:8888/getblockchaininfo
/*
RESPONSE{
  "chain": "test",
  "blocks": 1486864,
  "headers": 1486864,
  "bestblockhash": "000000000000002fb99d683e64bbfc2b7ad16f9a425cf7be77b481fb1afa363b",
  "difficulty": 13971064.71015782,
  "mediantime": 1554149114,
  "verificationprogress": 0.9999994536561675,
  "initialblockdownload": false,
  "chainwork": "000000000000000000000000000000000000000000000103ceb57a5896f347ce",
  "size_on_disk": 23647567017,
  "pruned": false,
  "softforks": [
    {
      "id": "bip34",
      "version": 2,
      "reject": {
        "status": true
      }
    },
    {
      "id": "bip66",
      "version": 3,
      "reject": {
        "status": true
      }
    },
    {
      "id": "bip65",
      "version": 4,
      "reject": {
        "status": true
      }
    }
  ],
  "bip9_softforks": {
    "csv": {
      "status": "active",
      "startTime": 1456790400,
      "timeout": 1493596800,
      "since": 770112
    },
    "segwit": {
      "status": "active",
      "startTime": 1462060800,
      "timeout": 1493596800,
      "since": 834624
    }
  },
  "warnings": "Warning: unknown new rules activated (versionbit 28)"
}
*/

// GET http://cyphernode:8888/getbalance
/*
RESPONSE{
  "balance":1.51911837
}
*/
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Balance {
    pub balance: f64,
}
impl Balance {
    /// Used internally to convert api json string to native struct
    pub fn from_str(stringified: &str) -> Result<Balance, S5Error> {
        match serde_json::from_str(stringified) {
            Ok(result) => Ok(result),
            Err(e) => Err(S5Error::new(ErrorKind::Internal, &e.to_string())),
        }
    }
}
///Returns the balance of core wallet
pub async fn getbalance(
    host: String,
    jwt: String,
    cert: Certificate,
) -> Result<Balance, String> {
    let full_url: String = format!("https://{}/v0/getbalance", host).to_string();
    let mut headers = HeaderMap::new();
    headers.insert(AUTHORIZATION, format!("Bearer {}", jwt).parse().unwrap());

    let client = reqwest::Client::builder().add_root_certificate(cert);
    let client = match client.default_headers(headers).build() {
        Ok(result) => result,
        Err(e) => return Err(e.to_string()),
    };
    match client.get(&full_url).send().await {
        Ok(response) => match response.text().await {
            Ok(text) => {
                println!("{}", text);
                match Balance::from_str(&text) {
                    Ok(result) => Ok(result),
                    Err(e) => Err(e.message),
                }
            }
            Err(e) => Err(e.to_string()),
        },
        Err(e) => Err(e.to_string()),
    }
}

// POST http://cyphernode:8888/bitcoin_estimatesmartfee
/*
REQUEST{
    "confTarget":2
}
RESPONSE{
    "result": {
      "feerate": 0.00001000,
      "blocks": 4
    },
    "error": null,
    "id": null
}
*/
