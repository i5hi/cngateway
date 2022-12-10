use crate::e::{ErrorKind, S5Error};
use serde_derive::{Deserialize, Serialize};
use reqwest::{self, header::{HeaderMap, AUTHORIZATION}, Certificate};
use serde_json::Value;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ResponseType {
  Create(CreateBatcherResponse),
  Update(UpdateBatchResponse),
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
    pub error: Value,
}

// ip=$(docker container inspect -f '{{ .NetworkSettings.Networks.cyphernodenet.IPAddress }}' cyphernode_proxy_1)
// POST http://$ip:8888/createbatcher
/*
REQUEST{
    "batcherLabel":"lowfees",
    "confTarget":32
}
RESPONSE{
  "result": {
    "batcherId": 1
  },
  "error": null
}
*/
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateBatcherRequest {
    pub batcher_label: String,
    pub conf_target: u64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateBatcherResponse {
    pub batcher_id: u64,
}

// POST http://cyphernode:8888/updatebatcher
/*
REQUEST{
    "batcherLabel":"fast",
    "confTarget":2
}
RESPONSE{
  "result": {
    "batcherId": 1,
    "batcherLabel": "fast",
    "confTarget": 2
  },
  "error": null
}
*/
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateBatchRequest {
  pub batcher_label: Option<String>,
  pub batcher_id: Option<String>,
  pub conf_target: u64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateBatchResponse {
  pub batcher_label: String,
  pub batcher_id: String,
  pub conf_target: u64,
}
// POST http://cyphernode:8888/addtobatch
/*
REQUEST{
    "address":"2N8DcqzfkYi8CkYzvNNS5amoq3SbAcQNXKp",
    "amount":0.00233
    "batcherLabel":"lowfees",
    "webhookUrl":"https://myCypherApp:3000/batchExecuted"
}
RESPONSE{
  "result": {
    "batcherId": 1,
    "outputId": 34,
    "nbOutputs": 7,
    "oldest": "2020-09-09 14:00:01",
    "total": 0.04016971
  },
  "error": null
}
*/
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AddToBatchRequest {
    pub address: String,
    pub amount: f64,
    pub batcher_label: Option<String>,
    pub webhook_url: Option<String>,
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

// POST http://cyphernode:8888/removefrombatch
/*
REQUEST{
    "outputId":72
}
RESPONSE- SAME AS ADD

*/
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RemoveFromBatchRequest {
  pub output_id: u64,
}

// POST http://cyphernode:8888/getbatcher
/*
REQUEST{}
or
{"batcherId":34}
or
{"batcherLabel":"fastest"}


RESPONSE{
  "result": {
    "batcherId": 1,
    "batcherLabel": "default",
    "confTarget": 6,
    "nbOutputs": 12,
    "oldest": 123123,
    "total": 0.86990143
  },
  "error": null
}
*/

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetBatchRequest {
  pub batcher_label: Option<String>,
  pub batcher_id: Option<String>,
}

// POST http://cyphernode:8888/getbatchdetails
/*
REQUEST{
}
or
{"batcherId":34}
or
{"batcherLabel":"fastest","txid":"af867c86000da76df7ddb1054b273ca9e034e8c89d049b5b2795f9f590f67648"}

RESPONSE{
  "result": {
    "batcherId": 34,
    "batcherLabel": "Special batcher for a special client",
    "confTarget": 6,
    "nbOutputs": 83,
    "oldest": 123123,
    "total": 10.86990143,
    "txid": "af867c86000da76df7ddb1054b273ca9e034e8c89d049b5b2795f9f590f67648",
    "hash": "af867c86000da76df7ddb1054b273ca9e034e8c89d049b5b2795f9f590f67648",
    "details": {
      "firstseen": 123123,
      "size": 424,
      "vsize": 371,
      "replaceable":true,
      "fee": 0.00004112
    },
    "outputs": {
      "1abc": 0.12,
      "3abc": 0.66,
      "bc1abc": 2.848,
      ...
    }
  },
  "error": null
}
*/

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetBatchDetailRequest {
  pub batcher_id: i64,
  pub batcher_label: Option<String>,
  pub txid: Option<String>,
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
    pub outputs: HashMap<String,f64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Details {
    pub firstseen: i64,
    pub size: i64,
    pub vsize: i64,
    pub replaceable: bool,
    pub fee: f64,
}


// GET http://cyphernode:8888/listbatchers
/*
RESPONSE{
  "result": [
    {"batcherId":1,"batcherLabel":"default","confTarget":6,"nbOutputs":12,"oldest":123123,"total":0.86990143},
    {"batcherId":2,"batcherLabel":"lowfee","confTarget":32,"nbOutputs":44,"oldest":123123,"total":0.49827387},
    {"batcherId":3,"batcherLabel":"highfee","confTarget":2,"nbOutputs":7,"oldest":123123,"total":4.16843782}
  ],
  "error": null
}
*/

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

// GET http://cyphernode:8888/batchspend
/*
RESPONSE{
  "status": "accepted",
  "hash": "af867c86000da76df7ddb1054b273ca9e034e8c89d049b5b2795f9f590f67648"
}

*/

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BatchSpendRequest {
  pub batcher_label: Option<String>,
  pub batcher_id: Option<String>,
  pub conf_target: Option<u64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BatchSpendResponse {
  pub status: String,
  pub hash: String,
}