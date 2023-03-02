use crate::e::{ErrorKind, S5Error};
use reqwest::{
    self,
    header::{HeaderMap, AUTHORIZATION},
    Certificate,
};
use serde_derive::{Deserialize, Serialize};

// POST http://cyphernode/watch
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WatchAddressReq {
    pub address: String,
    #[serde(rename = "unconfirmedCallbackURL")]
    pub unconfirmed_callback_url: String,
    #[serde(rename = "confirmedCallbackURL")]
    pub confirmed_callback_url: String,
    #[serde(rename = "eventMessage")]
    pub event_message: Option<String>,
    pub label: String,
}
impl WatchAddressReq {
    pub fn new(
        address: String,
        unconfirmed_callback_url: String,
        confirmed_callback_url: String,
        event_message: Option<String>,
        label: String,
    ) -> Self {
        WatchAddressReq {
            address,
            unconfirmed_callback_url,
            confirmed_callback_url,
            event_message,
            label,
        }
    }
    /// Used internally to convert to native struct to api json string
    pub fn stringify(&self) -> Result<String, S5Error> {
        match serde_json::to_string(&self.clone()) {
            Ok(result) => Ok(result),
            Err(e) => Err(S5Error::new(
                ErrorKind::Internal,
                &e.to_string(),
            )),
        }
    }
}
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WatchAddress {
    pub address: String,
    #[serde(rename = "unconfirmedCallbackURL")]
    pub unconfirmed_callback_url: String,
    #[serde(rename = "confirmedCallbackURL")]
    pub confirmed_callback_url: String,
    #[serde(rename = "eventMessage")]
    pub event_message: Option<String>,
    pub label: String,
}

impl WatchAddress {
    /// Used internally to convert api json string to native struct
    pub fn from_str(stringified: &str) -> Result<WatchAddress, S5Error> {
        match serde_json::from_str(stringified) {
            Ok(result) => Ok(result),
            Err(e) => Err(S5Error::new(
                ErrorKind::Internal,
                &e.to_string(),
            )),
        }
    }
}

pub async fn watch(
    host: String,
    jwt: String,
    cert: Certificate,
    body: WatchAddressReq,
) -> Result<WatchAddress, String> {
    let full_url: String = format!("https://{}/v0/watch", host).to_string();
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
                match WatchAddress::from_str(&text) {
                    Ok(result) => Ok(result),
                    Err(e) => Err(e.message),
                }
            }
            Err(e) => Err(e.to_string()),
        },
        Err(e) => Err(e.to_string()),
    }
}
// GET http://cyphernode/getactivewatches
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActiveWatches {
    pub watches: Vec<Watch>,
}
impl ActiveWatches {
    /// Used internally to convert api json string to native struct
    pub fn from_str(stringified: &str) -> Result<ActiveWatches, S5Error> {
        match serde_json::from_str(stringified) {
            Ok(result) => Ok(result),
            Err(e) => Err(S5Error::new(
                ErrorKind::Internal,
                &e.to_string(),
            )),
        }
    }
}
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Watch {
    pub id: i64,
    pub address: String,
    pub imported: bool,
    #[serde(rename = "unconfirmedCallbackURL")]
    pub unconfirmed_callback_url: String,
    #[serde(rename = "confirmedCallbackURL")]
    pub confirmed_callback_url: String,
    #[serde(rename = "watching_since")]
    pub watching_since: String,
    #[serde(rename = "eventMessage")]
    pub event_message: Option<String>,
}

pub async fn getactivewatches(
    host: String,
    jwt: String,
    cert: Certificate,
) -> Result<ActiveWatches, String> {
    let full_url: String = format!("https://{}/v0/getactivewatches", host).to_string();
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
                match ActiveWatches::from_str(&text) {
                    Ok(result) => Ok(result),
                    Err(e) => Err(e.message),
                }
            }
            Err(e) => Err(e.to_string()),
        },
        Err(e) => Err(e.to_string()),
    }
}
// GET http://cyphernode/unwatch/2N8DcqzfkYi8CkYzvNNS5amoq3SbAcQNXKp
/*
RESPONSE{
  "event": "unwatch",
  "address": "2N8DcqzfkYi8CkYzvNNS5amoq3SbAcQNXKp",
  "unconfirmedCallbackURL": "192.168.133.233:1111/callback0conf",
  "confirmedCallbackURL": "192.168.133.233:1111/callback1conf"
}
*/
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UnwatchAddress {
    pub event: String,
    pub address: String,
    #[serde(rename = "unconfirmedCallbackURL")]
    pub unconfirmed_callback_url: String,
    #[serde(rename = "confirmedCallbackURL")]
    pub confirmed_callback_url: String,
}
impl UnwatchAddress {
    /// Used internally to convert api json string to native struct
    pub fn structify(stringified: &str) -> Result<UnwatchAddress, S5Error> {
        match serde_json::from_str(stringified) {
            Ok(result) => Ok(result),
            Err(e) => Err(S5Error::new(
                ErrorKind::Internal,
                &e.to_string(),
            )),
        }
    }
}
pub async fn unwatch(
    host: String,
    jwt: String,
    cert: Certificate,
    address: String,
) -> Result<UnwatchAddress, String> {
    let full_url: String = format!("https://{}/v0/unwatch/{}", host, address).to_string();
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
                match UnwatchAddress::structify(&text) {
                    Ok(result) => Ok(result),
                    Err(e) => Err(e.message),
                }
            }
            Err(e) => Err(e.to_string()),
        },
        Err(e) => Err(e.to_string()),
    }
}
// GET http://cyphernode/get_txns_by_watchlabel/Label
/*
RESPONSE{
  "label_txns": [
    {
      "label": "Label",
      "address": "tb3qvsk9em20hgd76d489jyfdpy840vywk5qx9p5sg",
      "txid": "d48171ecc2ea4310ee7a15d9f11d8410d6a658225152b0c27122de1999d87cb2",
      "confirmations": "1",
      "blockheight": "1817509",
      "v_out": "0",
      "amount": "2.545e-05",
      "blockhash": "000000000000015df543042fa9179fe5e0823ef4e9a8cd52c9f26ce96b5935b1",
      "blocktime": "1596496264",
      "timereceived": "1596437271"
    }
  ]
}
*/
// POST http://cyphernode/watchxpub
/*
REQUEST{
    "label":"4421",
    "pub32":"upub57Wa4MvRPNyAhxr578mQUdPr6MHwpg3Su875hj8K75AeUVZLXtFeiP52BrhNqDg93gjALU1MMh5UPRiiQPrwiTiuBBBRHzeyBMgrbwkmmkq",
    "path":"0/1/n",
    "nstart":109,
    "unconfirmedCallbackURL":"192.168.111.233:1111/callback0conf",
    "confirmedCallbackURL":"192.168.111.233:1111/callback1conf"
}
RESPONSE{
  "id":"5",
  "event":"watchxpub",
  "pub32":"upub57Wa4MvRPNyAhxr578mQUdPr6MHwpg3Su875hj8K75AeUVZLXtFeiP52BrhNqDg93gjALU1MMh5UPRiiQPrwiTiuBBBRHzeyBMgrbwkmmkq",
  "label":"2219",
  "path":"0/1/n",
  "nstart":"109",
  "unconfirmedCallbackURL":"192.168.111.233:1111/callback0conf",
  "confirmedCallbackURL":"192.168.111.233:1111/callback1conf"
}
*/
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WatchXpubReq {
    pub label: String,
    pub pub32: String,
    pub path: String,
    pub nstart: i64,
    #[serde(rename = "unconfirmedCallbackURL")]
    pub unconfirmed_callback_url: String,
    #[serde(rename = "confirmedCallbackURL")]
    pub confirmed_callback_url: String,
}
impl WatchXpubReq {
    pub fn new(
        label: String,
        pub32: String,
        path: String,
        nstart: i64,
        unconfirmed_callback_url: String,
        confirmed_callback_url: String,
    ) -> Self {
        WatchXpubReq {
            label,
            pub32,
            path,
            nstart,
            unconfirmed_callback_url,
            confirmed_callback_url,
        }
    }
    /// Used internally to convert to native struct to api json string
    pub fn stringify(&self) -> Result<String, S5Error> {
        match serde_json::to_string(&self.clone()) {
            Ok(result) => Ok(result),
            Err(e) => Err(S5Error::new(
                ErrorKind::Internal,
                &e.to_string(),
            )),
        }
    }
}
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WatchXpub {
    pub id: String,
    pub event: String,
    pub pub32: String,
    pub label: String,
    pub path: String,
    pub nstart: String,
    #[serde(rename = "unconfirmedCallbackURL")]
    pub unconfirmed_callback_url: String,
    #[serde(rename = "confirmedCallbackURL")]
    pub confirmed_callback_url: String,
}
impl WatchXpub {
    /// Used internally to convert api json string to native struct
    pub fn structify(stringified: &str) -> Result<WatchXpub, S5Error> {
        match serde_json::from_str(stringified) {
            Ok(result) => Ok(result),
            Err(e) => Err(S5Error::new(
                ErrorKind::Internal,
                &e.to_string(),
            )),
        }
    }
}

pub async fn watchxpub(
    host: String,
    jwt: String,
    cert: Certificate,
    body: WatchXpubReq,
) -> Result<WatchXpub, String> {
    let full_url: String = format!("https://{}/v0/watch", host).to_string();
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
                match WatchXpub::structify(&text) {
                    Ok(result) => Ok(result),
                    Err(e) => Err(e.message),
                }
            }
            Err(e) => Err(e.to_string()),
        },
        Err(e) => Err(e.to_string()),
    }
}

// GET http://cyphernode/unwatchxpubbyxpub/upub57Wa4MvRPNyAhxr578mQUdPr6MHwpg3Su875hj8K75AeUVZLXtFeiP52BrhNqDg93gjALU1MMh5UPRiiQPrwiTiuBBBRHzeyBMgrbwkmmkq

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UnwatchXpub {
    pub event: String,
    pub pub32: String,
}
impl UnwatchXpub {
    /// Used internally to convert api json string to native struct
    pub fn structify(stringified: &str) -> Result<UnwatchXpub, S5Error> {
        match serde_json::from_str(stringified) {
            Ok(result) => Ok(result),
            Err(e) => Err(S5Error::new(
                ErrorKind::Internal,
                &e.to_string(),
            )),
        }
    }
}
pub async fn unwatchxpubbyxpub(
    host: String,
    jwt: String,
    cert: Certificate,
    xpub: String,
) -> Result<UnwatchXpub, String> {
    let full_url: String = format!("https://{}/v0/unwatchxpubbyxpub/{}", host, xpub).to_string();
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
                match UnwatchXpub::structify(&text) {
                    Ok(result) => Ok(result),
                    Err(e) => Err(e.message),
                }
            }
            Err(e) => Err(e.to_string()),
        },
        Err(e) => Err(e.to_string()),
    }
}

// GET http://cyphernode/getactivexpubwatches
/*
RESPONSE{
  "watches": [
  {
  "id":"291",
  "pub32":"upub57Wa4MvRPNyAhxr578mQUdPr6MHwpg3Su875hj8K75AeUVZLXtFeiP52BrhNqDg93gjALU1MMh5UPRiiQPrwiTiuBBBRHzeyBMgrbwkmmkq",
  "label":"2217",
  "derivation_path":"1/3/n",
  "last_imported_n":"121",
  "unconfirmedCallbackURL":"192.168.133.233:1111/callback0conf",
  "confirmedCallbackURL":"192.168.133.233:1111/callback1conf",
  "watching_since":"2018-09-06 21:14:03"}
  ]
}
*/

// GET http://cyphernode/executecallbacks
// executes the callbacks that would be usually executed when "conf" is called by the node.

/*

CALLBACK STRUCTS

ZEROCONF{
  "id":"3832",
  "address":"2NB96fbwy8eoHttuZTtbwvvhEYrBwz494ov",
  "hash":"af867c86000da76df7ddb1054b273ca9e034e8c89d049b5b2795f9f590f67648",
  "vout_n":1,
  "sent_amount":0.84050318,
  "confirmations":0,
  "received":"2018-10-18T15:41:06+0000",
  "size":371,
  "vsize":166,
  "fees":0.00002992,
  "replaceable":false,
  "blockhash":"",
  "blocktime":"",
  "blockheight":""
}

ONECONF{
  "id":"3832",
  "address":"2NB96fbwy8eoHttuZTtbwvvhEYrBwz494ov",
  "hash":"af867c86000da76df7ddb1054b273ca9e034e8c89d049b5b2795f9f590f67648",
  "vout_n":1,
  "sent_amount":0.84050318,
  "confirmations":1,
  "received":"2018-10-18T15:41:06+0000",
  "size":371,
  "vsize":166,
  "fees":0.00002992,
  "replaceable":false,
  "blockhash":"00000000000000000011bb83bb9bed0f6e131d0d0c903ec3a063e00b3aa00bf6",
  "blocktime":"2018-10-18T16:58:49+0000",
  "blockheight":""
}
*/
//
