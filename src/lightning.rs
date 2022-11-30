use crate::e::{ErrorKind, S5Error};
use serde_derive::{Deserialize, Serialize};
use serde_json::Value;

/// Response from <- GET http://cyphernode:8888/ln_getinfo
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LnInfo {
    pub id: String,
    pub alias: String,
    pub color: String,
    pub address: Vec<Value>,
    pub binding: Vec<Binding>,
    pub version: String,
    pub blockheight: i64,
    pub network: String,
}
impl LnInfo {
    /// Used internally to convert api json string to native struct
    pub fn structify(stringified: &str) -> Result<LnInfo, S5Error> {
        match serde_json::from_str(stringified) {
            Ok(result) => Ok(result),
            Err(_) => Err(S5Error::new(
                ErrorKind::Internal,
                "Error stringifying LightningInfo",
            )),
        }
    }
}
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Binding {
    #[serde(rename = "type")]
    pub type_field: String,
    pub address: String,
    pub port: i64,
}

pub async fn ln_getinfo(ip: String) -> Result<LnInfo, String> {
    let full_url: String = format!("http://{}:8888/ln_getinfo", ip).to_string();

    match ureq::get(&full_url).call() {
        Ok(response) => match LnInfo::structify(&response.into_string().unwrap()) {
            Ok(result) => Ok(result),
            Err(e) => Err(e.message),
        },
        Err(e) => Err(e.to_string()),
    }
}

/// Response from <- GET http://cyphernode:8888/ln_newaddr.
/// !Only supports bech32!
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LnFundAddress {
    pub bech32: String, // notify typo in doc/apiv0
}
impl LnFundAddress {
    /// Used internally to convert api json string to native struct
    pub fn structify(stringified: &str) -> Result<LnFundAddress, S5Error> {
        match serde_json::from_str(stringified) {
            Ok(result) => Ok(result),
            Err(_) => Err(S5Error::new(
                ErrorKind::Internal,
                "Error stringifying LnFundAddress",
            )),
        }
    }
}

pub async fn ln_newaddr(ip: String) -> Result<LnFundAddress, String> {
    let full_url: String = format!("http://{}:8888/ln_newaddr", ip).to_string();

    match ureq::get(&full_url).call() {
        Ok(response) => match LnFundAddress::structify(&response.into_string().unwrap()) {
            Ok(result) => Ok(result),
            Err(e) => Err(e.message),
        },
        Err(e) => Err(e.to_string()),
    }
}

/// Response from <- GET http://cyphernode:8888/ln_getconnectionstring
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LnConnString {
    pub connectstring: String, // notify typo in doc/apiv0
}
impl LnConnString {
    /// Used internally to convert api json string to native struct
    pub fn structify(stringified: &str) -> Result<LnConnString, S5Error> {
        match serde_json::from_str(stringified) {
            Ok(result) => Ok(result),
            Err(_) => Err(S5Error::new(
                ErrorKind::Internal,
                "Error stringifying LnConnString",
            )),
        }
    }
}

pub async fn ln_getconnectionstring(ip: String) -> Result<LnConnString, String> {
    let full_url: String = format!("http://{}:8888/ln_getconnectionstring", ip).to_string();

    match ureq::get(&full_url).call() {
        Ok(response) => match LnConnString::structify(&response.into_string().unwrap()) {
            Ok(result) => Ok(result),
            Err(e) => Err(e.message),
        },
        Err(e) => Err(e.to_string()),
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LnConnectFundReq {
    pub peer: String,
    pub msatoshi: u128,
    #[serde(rename = "callbackUrl")]
    pub callback_url: String,
}
impl LnConnectFundReq {
    pub fn new(peer: String, msatoshi: u128, callback_url: String) -> Self {
        LnConnectFundReq {
            peer,
            msatoshi,
            callback_url,
        }
    }
    /// Used internally to convert to native struct to api json string
    pub fn stringify(&self) -> Result<String, S5Error> {
        match serde_json::to_string(&self.clone()) {
            Ok(result) => Ok(result),
            Err(_) => Err(S5Error::new(
                ErrorKind::Internal,
                "Error stringifying LnConnectFundRequest",
            )),
        }
    }
}

/// Response from <- POST http://cyphernode:8888/ln_connectfund
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LnConnectFund {
    pub result: String,
    pub txid: String,
    pub channel_id: String,
}
impl LnConnectFund {
    /// Used internally to convert api json string to native struct
    pub fn structify(stringified: &str) -> Result<LnConnectFund, S5Error> {
        match serde_json::from_str(stringified) {
            Ok(result) => Ok(result),
            Err(_) => Err(S5Error::new(
                ErrorKind::Internal,
                "Error stringifying LnConnectFund",
            )),
        }
    }
}
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LnConnectFundError {
    pub result: String,
    pub message: String,
}
impl LnConnectFundError {
    /// Used internally to convert api json string to native struct
    pub fn structify(stringified: &str) -> Result<LnConnectFundError, S5Error> {
        match serde_json::from_str(stringified) {
            Ok(result) => Ok(result),
            Err(_) => Err(S5Error::new(
                ErrorKind::Internal,
                "Error stringifying LnConnectFundError",
            )),
        }
    }
}

pub async fn ln_connectfund(ip: String, body: LnConnectFundReq) -> Result<LnConnectFund, String> {
    let full_url: String = format!("http://{}:8888/ln_connectfund", ip).to_string();
    match ureq::post(&full_url).send_json(body.stringify()) {
        Ok(response) => match LnConnectFund::structify(&response.into_string().unwrap()) {
            Ok(result) => Ok(result),
            Err(e) => Err(e.message),
        },
        Err(e) => Err(e.to_string()),
    }
}

/// Response from <- GET http://cyphernode:8888/ln_decodebolt11/$invoice
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LnBolt11 {
    pub currency: String,
    pub created_at: i64,
    pub expiry: i64,
    pub payee: String,
    pub description: String,
    pub min_final_cltv_expiry: i64,
    pub payment_hash: String,
    pub signature: String,
}

impl LnBolt11 {
    /// Used internally to convert api json string to native struct
    pub fn structify(stringified: &str) -> Result<LnBolt11, S5Error> {
        match serde_json::from_str(stringified) {
            Ok(result) => Ok(result),
            Err(_) => Err(S5Error::new(
                ErrorKind::Internal,
                "Error structifying LnBolt11",
            )),
        }
    }
}

pub async fn ln_decodebolt11(ip: String, invoice: String) -> Result<LnBolt11, String> {
    let full_url: String = format!("http://{}:8888/ln_decodebolt11/{}", ip, invoice).to_string();

    match ureq::get(&full_url).call() {
        Ok(response) => match LnBolt11::structify(&response.into_string().unwrap()) {
            Ok(result) => Ok(result),
            Err(e) => Err(e.message),
        },
        Err(e) => Err(e.to_string()),
    }
}
// GET http://cyphernode:8888/ln_listpeers
/*
RESPONSE{
   "peers": [
      {
         "id": "0[REDACTED]e",
         "connected": true,
         "netaddr": [
            "181.[REDACTED].228:9735"
         ],
         "globalfeatures": "",
         "localfeatures": "81",
         "features": "81",
         "channels": [
            {
               "state": "CHANNELD_NORMAL",
               "scratch_txid": "3[REDACTED]e",
               "owner": "channeld",
               "short_channel_id": "6[REDACTED]6x0",
               "direction": 0,
               "channel_id": "7[REDACTED]c",
               "funding_txid": "0[REDACTED]f",
               "close_to_addr": "bc1[REDACTED]f",
               "close_to": "0[REDACTED]6",
               "private": false,
               "funding_allocation_msat": {
                  "0[REDACTED]e": 0,
                  "0[REDACTED]a": 699139000
               },
               "funding_msat": {
                  "0[REDACTED]e": "0msat",
                  "0[REDACTED]a": "699139000msat"
               },
               "msatoshi_to_us": 699128000,
               "to_us_msat": "699128000msat",
               "msatoshi_to_us_min": 699128000,
               "min_to_us_msat": "699128000msat",
               "msatoshi_to_us_max": 699139000,
               "max_to_us_msat": "699139000msat",
               "msatoshi_total": 699139000,
               "total_msat": "699139000msat",
               "dust_limit_satoshis": 546,
               "dust_limit_msat": "546000msat",
               "max_htlc_value_in_flight_msat": 12446749275109551625,
               "max_total_htlc_in_msat": "12446749275109551625msat",
               "their_channel_reserve_satoshis": 6998,
               "their_reserve_msat": "6998000msat",
               "our_channel_reserve_satoshis": 6997,
               "our_reserve_msat": "6997000msat",
               "spendable_msatoshi": 688236000,
               "spendable_msat": "688236000msat",
               "htlc_minimum_msat": 0,
               "minimum_htlc_in_msat": "0msat",
               "their_to_self_delay": 144,
               "our_to_self_delay": 144,
               "max_accepted_htlcs": 483,
               "status": [
                  "CHANNELD_NORMAL:Reconnected, and reestablished.",
                  "CHANNELD_NORMAL:Funding transaction locked. Channel announced."
               ],
               "in_payments_offered": 0,
               "in_msatoshi_offered": 0,
               "in_offered_msat": "0msat",
               "in_payments_fulfilled": 0,
               "in_msatoshi_fulfilled": 0,
               "in_fulfilled_msat": "0msat",
               "out_payments_offered": 2,
               "out_msatoshi_offered": 13245566,
               "out_offered_msat": "13245566msat",
               "out_payments_fulfilled": 1,
               "out_msatoshi_fulfilled": 11000,
               "out_fulfilled_msat": "11000msat",
               "htlcs": []
            }
         ]
      },
      {
         "id": "0[REDACTED]9",
         "connected": true,
         "netaddr": [
            "wp[REDACTED]d.onion:9735"
         ],
         "globalfeatures": "",
         "localfeatures": "2281",
         "features": "2281",
         "channels": [
            {
               "state": "CHANNELD_NORMAL",
               "scratch_txid": "8[REDACTED]f",
               "owner": "channeld",
               "short_channel_id": "6[REDACTED]3x0",
               "direction": 1,
               "channel_id": "9[REDACTED]3",
               "funding_txid": "2[REDACTED]e",
               "close_to_addr": "bc1[REDACTED]d",
               "close_to": "0[REDACTED]f",
               "private": false,
               "funding_allocation_msat": {
                  "0[REDACTED]9": 0,
                  "0[REDACTED]a": 328682000
               },
               "funding_msat": {
                  "0[REDACTED]9": "0msat",
                  "0[REDACTED]a": "328682000msat"
               },
               "msatoshi_to_us": 328682000,
               "to_us_msat": "328682000msat",
               "msatoshi_to_us_min": 328682000,
               "min_to_us_msat": "328682000msat",
               "msatoshi_to_us_max": 328682000,
               "max_to_us_msat": "328682000msat",
               "msatoshi_total": 328682000,
               "total_msat": "328682000msat",
               "dust_limit_satoshis": 546,
               "dust_limit_msat": "546000msat",
               "max_htlc_value_in_flight_msat": 12446744073709551615,
               "max_total_htlc_in_msat": "12446744073709551615msat",
               "their_channel_reserve_satoshis": 7287,
               "their_reserve_msat": "7287000msat",
               "our_channel_reserve_satoshis": 7286,
               "our_reserve_msat": "7286000msat",
               "spendable_msatoshi": 727826000,
               "spendable_msat": "727826000msat",
               "htlc_minimum_msat": 0,
               "minimum_htlc_in_msat": "0msat",
               "their_to_self_delay": 144,
               "our_to_self_delay": 144,
               "max_accepted_htlcs": 483,
               "status": [
                  "CHANNELD_NORMAL:Sent reestablish, waiting for theirs"
               ],
               "in_payments_offered": 0,
               "in_msatoshi_offered": 0,
               "in_offered_msat": "0msat",
               "in_payments_fulfilled": 0,
               "in_msatoshi_fulfilled": 0,
               "in_fulfilled_msat": "0msat",
               "out_payments_offered": 20,
               "out_msatoshi_offered": 3104386818,
               "out_offered_msat": "3104386818msat",
               "out_payments_fulfilled": 0,
               "out_msatoshi_fulfilled": 0,
               "out_fulfilled_msat": "0msat",
               "htlcs": []
            }
         ]
      }
   ]
}
*/

/*
RESPONSE{
   "outputs": [
    {
      "txid": "d3a536efaa70671xxxxxxxxx8f349a3c326b79",
      "output": 0,
      "value": 9551,
      "amount_msat": "9551000msat",
      "address": "tb1qq0....j9kqze0",
      "status": "confirmed",
      "blockheight": 1715749
    },
    {}
  ],
  "channels": [
  {
      "peer_id": "03f60f736....34f05a93a8a897b75c7940a55bb9",
      "connected": true,
      "state": "CHANNELD_NORMAL",
      "short_channel_id": "166...x0",
      "channel_sat": 100000,
      "our_amount_msat": "100000000msat",
      "channel_total_sat": 100000,
      "amount_msat": "100000000msat",
      "funding_txid": "53cf8cd...0c41c2e2b17887b3",
      "funding_output": 0
    },
    {}
  ]
}
*/
/// Response from <- GET http://cyphernode:8888/ln_listfunds
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LnListFunds {
    pub outputs: Vec<Output>,
    pub channels: Vec<Channel>,
}
impl LnListFunds {
    /// Used internally to convert api json string to native struct
    pub fn structify(stringified: &str) -> Result<LnListFunds, S5Error> {
        match serde_json::from_str(stringified) {
            Ok(result) => Ok(result),
            Err(_) => Err(S5Error::new(
                ErrorKind::Internal,
                "Error structifying LnListFunds",
            )),
        }
    }
}
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Output {
    pub txid: String,
    pub output: i64,
    pub value: i64,
    pub amount_msat: String,
    pub address: String,
    pub status: String,
    pub blockheight: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Channel {
    pub peer_id: String,
    pub connected: bool,
    pub state: String,
    pub short_channel_id: String,
    pub channel_sat: i64,
    pub our_amount_msat: String,
    pub channel_total_sat: i64,
    pub amount_msat: String,
    pub funding_txid: String,
    pub funding_output: i64,
}

pub async fn ln_listfunds(ip: String) -> Result<LnListFunds, String> {
    let full_url: String = format!("http://{}:8888/ln_listfunds", ip).to_string();

    match ureq::get(&full_url).call() {
        Ok(response) => match LnListFunds::structify(&response.into_string().unwrap()) {
            Ok(result) => Ok(result),
            Err(e) => Err(e.message),
        },
        Err(e) => Err(e.to_string()),
    }
}

/// Response from <- GET http://cyphernode:8888/ln_listpays
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LnListPays {
    pub pays: Vec<Pay>,
}
impl LnListPays {
    /// Used internally to convert api json string to native struct
    pub fn structify(stringified: &str) -> Result<LnListPays, S5Error> {
        match serde_json::from_str(stringified) {
            Ok(result) => Ok(result),
            Err(_) => Err(S5Error::new(
                ErrorKind::Internal,
                "Error structifying LnListPays",
            )),
        }
    }
}
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Pay {
    pub bolt11: Option<String>,
    pub status: Option<String>,
    pub preimage: Option<String>,
    #[serde(rename = "amount_sent_msat")]
    pub amount_sent_msat: Option<String>,
}

pub async fn ln_listpays(ip: String) -> Result<LnListPays, String> {
    let full_url: String = format!("http://{}:8888/ln_listpays", ip).to_string();

    match ureq::get(&full_url).call() {
        Ok(response) => match LnListPays::structify(&response.into_string().unwrap()) {
            Ok(result) => Ok(result),
            Err(e) => Err(e.message),
        },
        Err(e) => Err(e.to_string()),
    }
}
/// Response from <- GET http://cyphernode:8888/ln_getroute/<node_id>/<msatoshi>/<?riskfactor>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LnRoutes {
    pub route: Vec<Route>,
}
impl LnRoutes {
    /// Used internally to convert api json string to native struct
    pub fn structify(stringified: &str) -> Result<LnRoutes, S5Error> {
        match serde_json::from_str(stringified) {
            Ok(result) => Ok(result),
            Err(_) => Err(S5Error::new(
                ErrorKind::Internal,
                "Error structifying LnRoutes",
            )),
        }
    }
}
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Route {
    pub id: String,
    pub channel: String,
    pub direction: i64,
    pub msatoshi: i64,
    pub amount_msat: String,
    pub delay: i64,
    pub style: String,
}

pub async fn ln_getroute(
    ip: String,
    node_id: String,
    msatoshis: u128,
    risk_factor: f32,
) -> Result<LnRoutes, String> {
    let full_url: String = format!(
        "http://{}:8888/ln_getroute/{}/{}/{}",
        ip,
        node_id,
        msatoshis.to_string(),
        risk_factor.to_string()
    )
    .to_string();
    match ureq::post(&full_url).call() {
        Ok(response) => match LnRoutes::structify(&response.into_string().unwrap()) {
            Ok(result) => Ok(result),
            Err(e) => Err(e.message),
        },
        Err(e) => Err(e.to_string()),
    }
}
/// Response from <- POST http://192.168.111.152:8080/ln_withdraw
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LnWithdraw {
    pub tx: String,
    pub txid: String,
}
impl LnWithdraw {
    /// Used internally to convert api json string to native struct
    pub fn structify(stringified: &str) -> Result<LnWithdraw, S5Error> {
        match serde_json::from_str(stringified) {
            Ok(result) => Ok(result),
            Err(_) => Err(S5Error::new(
                ErrorKind::Internal,
                "Error stringifying LnWithdraw",
            )),
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LnWithdrawReq {
    pub destination: String,
    pub satoshi: String,
    pub feerate: String,
    pub all: bool,
}
impl LnWithdrawReq {
    pub fn new(address: String, amount: u128, feerate: String) -> Self {
        LnWithdrawReq {
            destination: address,
            satoshi: amount.to_string(),
            feerate: feerate,
            all: false,
        }
    }
    /// Used internally to convert to native struct to api json string
    pub fn stringify(&self) -> Result<String, S5Error> {
        match serde_json::to_string(&self.clone()) {
            Ok(result) => Ok(result),
            Err(_) => Err(S5Error::new(
                ErrorKind::Internal,
                "Error stringifying LnWithdrawReq",
            )),
        }
    }
}

pub async fn ln_withdraw(ip: String, body: LnWithdrawReq) -> Result<LnWithdraw, String> {
    let full_url: String = format!("http://{}:8888/ln_withdraw", ip).to_string();
    match ureq::post(&full_url).send_json(body.stringify()) {
        Ok(response) => match LnWithdraw::structify(&response.into_string().unwrap()) {
            Ok(result) => Ok(result),
            Err(e) => Err(e.message),
        },
        Err(e) => Err(e.to_string()),
    }
}
