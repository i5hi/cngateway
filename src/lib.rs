// mod bitcoin;
mod core;
mod e;
// mod lightning;
// mod proxy;
use serde::{Deserialize, Serialize};
// pub use bitcoin::{ActiveWatches, UnwatchAddress, WatchAddress,WatchXpub,UnwatchXpub};
// use bitcoin::{WatchAddressReq, WatchXpubReq};
// use lightning::LnConnectFundReq;
// pub use lightning::{
//     LnBolt11, LnConnString, LnConnectFund, LnFundAddress, LnInfo, LnListFunds, LnListPays,
//     LnRoutes, LnWithdraw, LnWithdrawReq,
// };
use crate::core::{MempoolInfo};

use jsonwebtoken::{
    encode, Algorithm, EncodingKey, Header
};
use std::time::SystemTime;
const LIFETIME: u128 = 3_600_000; // 1h
#[derive(Debug, Serialize, Deserialize)]
struct Claims {
   id: String,
   exp: u128,
}

/// The main client.
/// ip is the only requirement for initialization.
/// methods follow the same naming convention as cyphernode proxy api.
pub struct CnGateway {
    pub host: String,
    id: String,
    key: String,
}
impl CnGateway {
    /// Initialize
    pub fn new(host: String, id: String, key: String) -> Self {
        CnGateway { host, id, key }
    }
    fn auth_token(&self)->Result<String,String>{
        let now = match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
            Ok(n) => n.as_millis(),
            Err(_) => {
                return Err(
                    "Clock Went Backwards!".to_string())
            }
        };
        let payload = Claims {
            id: self.id.clone(),
            exp: now + LIFETIME,
        };
        let header = Header {
            alg: Algorithm::HS256,
            ..Default::default()
        };
        match encode(&header, &payload, &EncodingKey::from_secret(self.clone().key.as_ref())) {
            Ok(token) => Ok(token),
            Err(_) => Err("Error Encoding JWT!".to_string()),
        }
    }
    
    /// Check mempool info
    pub async fn getmempoolinfo(&self)->Result<MempoolInfo, String>{
        let jwt = self.auth_token().unwrap();
        core::getmempoolinfo(self.host.clone(), jwt).await
    }
    // /// Health check
    // pub async fn ping(&self) -> Result<(), String> {
    //     proxy::helloworld(self.ip.clone()).await
    // }
    // /// Watch a bitcoin address
    // pub async fn watch(
    //     &self,
    //     address: String,
    //     unconfirmed_callback_url: String,
    //     confirmed_callback_url: String,
    //     event_message: String,
    //     label: String,
    // ) -> Result<WatchAddress, String> {
    //     let body = WatchAddressReq::new(
    //         address,
    //         unconfirmed_callback_url,
    //         confirmed_callback_url,
    //         event_message,
    //         label,
    //     );
    //     bitcoin::watch(self.ip.clone(), body).await
    // }
    // /// Unwatch a bitcoin address
    // pub async fn unwatch(&self, address: String) -> Result<UnwatchAddress, String> {
    //     bitcoin::unwatch(self.ip.clone(), address).await
    // }
    // /// Get addresses currently being watched
    // pub async fn watchxpub(
    //     &self,
    //     label: String,
    //     pub32: String,
    //     path: String,
    //     nstart: i64,
    //     unconfirmed_callback_url: String,
    //     confirmed_callback_url: String,
    // ) -> Result<WatchXpub, String> {
    //     let body = WatchXpubReq::new(
    //         label,
    //         pub32,
    //         path,
    //         nstart,
    //         unconfirmed_callback_url,
    //         confirmed_callback_url,
    //     );
    //     bitcoin::watchxpub(self.ip.clone(), body).await
    // }
    // /// Unwatch a bitcoin xpub
    // pub async fn unwatchxpubbyxpub(&self, xpub: String) -> Result<UnwatchXpub, String> {
    //     bitcoin::unwatchxpubbyxpub(self.ip.clone(), xpub).await
    // }
    // /// Get addresses currently being watched
    // pub async fn getactivewatches(&self, address: String) -> Result<ActiveWatches, String> {
    //     bitcoin::getactivewatches(self.ip.clone()).await
    // }
    // /// Ln node info
    // pub async fn ln_getinfo(&self) -> Result<LnInfo, String> {
    //     lightning::ln_getinfo(self.ip.clone()).await
    // }
    // /// Get new address to deposit funds to open channels with
    // pub async fn ln_newaddr(&self) -> Result<LnFundAddress, String> {
    //     lightning::ln_newaddr(self.ip.clone()).await
    // }
    // /// Get your nodes connection string to share with peers
    // pub async fn ln_getconnectionstring(&self) -> Result<LnConnString, String> {
    //     lightning::ln_getconnectionstring(self.ip.clone()).await
    // }
    // /// Decode an invoice
    // pub async fn ln_decodebolt11(&self, invoice: String) -> Result<LnBolt11, String> {
    //     lightning::ln_decodebolt11(self.ip.clone(), invoice).await
    // }
    // /// Connect to a given peer and attempt opening a channel and fund it with msatoshis. Get notified at callback_url.
    // pub async fn ln_connectfund(
    //     &self,
    //     peer: String,
    //     msatoshis: u128,
    //     callback_url: String,
    // ) -> Result<LnConnectFund, String> {
    //     let body = LnConnectFundReq::new(peer, msatoshis, callback_url);
    //     println!("{:#?}", body);
    //     lightning::ln_connectfund(self.ip.clone(), body).await
    // }
    // /// Returns the list of unused outputs and funds in open channels
    // pub async fn ln_listfunds(&self) -> Result<LnListFunds, String> {
    //     lightning::ln_listfunds(self.ip.clone()).await
    // }
    // /// Returns history of paid invoices
    // pub async fn ln_listpays(&self) -> Result<LnListPays, String> {
    //     lightning::ln_listpays(self.ip.clone()).await
    // }
    // /// Returns an array representing hops of nodes to get to the destination node from our node
    // pub async fn ln_getroute(
    //     &self,
    //     node_id: String,
    //     msatoshis: u128,
    //     risk_factor: f32,
    // ) -> Result<LnRoutes, String> {
    //     lightning::ln_getroute(self.ip.clone(), node_id, msatoshis, risk_factor).await
    // }
    // /// Withdraw funds from channel back on main chain
    // pub async fn ln_withdraw(
    //     &self,
    //     address: String,
    //     satoshis: u128,
    //     feerate: String,
    // ) -> Result<LnWithdraw, String> {
    //     let body = LnWithdrawReq::new(address, satoshis, feerate);
    //     lightning::ln_withdraw(self.ip.clone(), body).await
    // }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_lib() {
        let output = std::process::Command::new("docker")
            .arg("inspect")
            .arg("-f {{ .NetworkSettings.Networks.cyphernodenet.IPAddress }}")
            .arg("cyphernode_gatekeeper_1")
            .output()
            .expect("Failed to execute command");

        let gatekeeper_ip = std::str::from_utf8(output.stdout.as_slice())
            .unwrap()
            .replace(" ", "")
            .to_string() + ":2009";
        let kid = "003".to_string();
        let key = "57072275edcd91d556b8917b71ab8b8b7c84c2c0ec7b0e50575788d1e51678fe".to_string();

        let client = CnGateway::new(gatekeeper_ip, kid, key);
        let mempool = client.getmempoolinfo().await.unwrap();
        // client.ping().await.unwrap();

        // let lninfo = client.ln_getinfo().await.unwrap();
        // let newaddr = client.ln_newaddr().await.unwrap();
        // let connstr = client.ln_getconnectionstring().await.unwrap();
        // let some_invoice = "lnbc920u1p3khp67pp5mcqxhupukc5te86wfkryerk8f69gg9ptzcep33ry94svm4wvwzqqdqqcqzzgxqyz5vqrzjqwnvuc0u4txn35cafc7w94gxvq5p3cu9dd95f7hlrh0fvs46wpvhdjx4k0kekn630gqqqqryqqqqthqqpyrzjqw8c7yfutqqy3kz8662fxutjvef7q2ujsxtt45csu0k688lkzu3ldjx4k0kekn630gqqqqryqqqqthqqpysp58nxs2nm5wphu234ggawaeul2tnpl6jqc9a0ymfhwpr64vq0k3l4s9qypqsqlkrver3pdxm0teyye0n6y5sje8u90t4j8vpxq3qjwjh9ue46cctj2nzw8fdudfec6nd0e8gx9v485ek7p624j5leeykg70wmv59y3pqqn9ulv2".to_string();
        // let bolt11_decoded = client.ln_decodebolt11(some_invoice).await.unwrap();
        // let peer =
        //     "02eadbd9e7557375161df8b646776a547c5cbc2e95b3071ec81553f8ec2cea3b8c@18.191.253.246:9735"
        //         .to_string();
        // let msatoshis = 3_690_000;
        // let callback_url = "http://yourcypherapp/callback/".to_string();
        // let fund_stat = client
        //     .ln_connectfund(peer, msatoshis, callback_url)
        //     .await
        //     .err();
        // let list_funds = client.ln_listfunds().await.unwrap();
        // let list_pays = client.ln_listpays().await.unwrap();

        // let peer = "02b856473d51e796fc5ff6098afa424d5a35a6e06ce5aa83904a4dcc6f457196d3".to_string();
        // let msatoshis = 3511;
        // let risk_factor = 0.1;
        // let routes = client
        //     .ln_getroute(peer, msatoshis, risk_factor)
        //     .await
        //     .unwrap();

        println!("{:#?}", mempool);

        // println!("{:#?}", lninfo);
        // println!("{:#?}", newaddr);
        // println!("{:#?}", connstr);
        // println!("{:#?}", bolt11_decoded);
        // println!("{:#?}", fund_stat);
        // println!("{:#?}", list_funds);
        // println!("{:#?}", list_pays);
        // println!("{:#?}", routes);
    }
}
