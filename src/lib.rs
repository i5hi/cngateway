/// cyphernode gateway client
///
/// cngateway provides an async rust client to interact with cyphernodes gatekeeper
/// 
///
/// 
/// # Examples
///
/// Basic usage:
///
/// ```
/// let gatekeeper_ip = "gatekeeper:2009".to_string(); // if you are connected to cyphernodeappsnet IF NOT expose gatekeeper outside network and use localhost
/// let kid = "003".to_string();
/// let key = "c06f9fc30c50ab7541cefaeb58708fe28babcf7d5ed1767a59685f63d0b63c54".to_string();
/// let cert_path = "/path/to/cacert.pem";
/// let client = CnGateway::new(
///     gatekeeper_ip.clone(),
///     kid.clone(),
///     key.clone(),
///     cert_path.clone(),
/// )
/// .await?;
/// // Use bitcoin core
/// let mempool = client.getmempoolinfo().await?;
/// let balance = client.getbalance().await?;
/// let address = client.getnewaddress(AddressType::Bech32,"dup".to_string()).await?; // uses the POST api format {address_type, label}
/// // Use lightning
/// let lninfo = client.ln_getinfo().await.unwrap();
/// let newaddr = client.ln_newaddr().await.unwrap();
/// let connstr = client.ln_getconnectionstring().await.unwrap();
/// let invoice = "lnbc920u1p3khp67pp5mcqxhupukc5te86wfkryerk8f69gg9ptzcep33ry94svm4wvwzqqdqqcqzzgxqyz5vqrzjqwnvuc0u4txn35cafc7w94gxvq5p3cu9dd95f7hlrh0fvs46wpvhdjx4k0kekn630gqqqqryqqqqthqqpyrzjqw8c7yfutqqy3kz8662fxutjvef7q2ujsxtt45csu0k688lkzu3ldjx4k0kekn630gqqqqryqqqqthqqpysp58nxs2nm5wphu234ggawaeul2tnpl6jqc9a0ymfhwpr64vq0k3l4s9qypqsqlkrver3pdxm0teyye0n6y5sje8u90t4j8vpxq3qjwjh9ue46cctj2nzw8fdudfec6nd0e8gx9v485ek7p624j5leeykg70wmv59y3pqqn9ulv2".to_string();
/// let bolt11_decoded = client.ln_decodebolt11(invoice).await.unwrap();
/// let peer =
///     "02eadbd9e7557375161df8b646776a547c5cbc2e95b3071ec81553f8ec2cea3b8c@18.191.253.246:9735"
///         .to_string();
/// let msatoshis = 3_690_000;
/// let callback_url = "http:///yourcypherapp/callback/".to_string();
/// let fund_stat = client
///     .ln_connectfund(peer, msatoshis, callback_url)
///     .await
///     .err();
/// let list_funds = client.ln_listfunds().await.unwrap();
/// let list_pays = client.ln_listpays().await.unwrap();
/// ```
use std::time::SystemTime;
use serde::{Deserialize, Serialize};
use reqwest::Certificate;
use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};

pub mod watcher;
pub mod core;
pub mod e;
pub mod batcher;
pub mod lightning;

use crate::core::{
    MempoolInfo, 
    AddressType, AddressRequest, Balance, Address
};

use crate::lightning::{
    LnBolt11, LnConnString, LnConnectFund, LnFundAddress, 
    LnInfo, LnListFunds, LnListPays,
    LnRoutes, LnWithdraw,LnConnectFundReq, LnWithdrawReq
};
use batcher::{
    AddToBatchRequest, BatchDetailResponse, BatchInfoResponse, BatchSpendRequest,
    BatchSpendResponse, CreateBatcherRequest, CreateBatcherResponse, GetBatchDetailRequest,
    GetBatcherRequest, ListBatchersResponse, RemoveFromBatchRequest,
    UpdateBatcherRequest, UpdateBatcherResponse,
};
use watcher::{
    ActiveWatches, UnwatchAddress, UnwatchXpub, 
    WatchAddress, WatchXpub,WatchAddressReq, WatchXpubReq
};

const LIFETIME: u128 = 3_600_000; // 1h

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    id: String,
    exp: u128,
}

/// The gatekeeper client.
#[derive(Clone)]
pub struct CnGateway {
    pub host: String,
    token: String,
    cert: Certificate,
}
impl CnGateway {
    /// Initialize client with auth secrets
    pub async fn new(
        host: String,
        id: String,
        key: String,
        cert_path: String,
    ) -> Result<Self, String> {
            let cert_content = match tokio::fs::read_to_string(cert_path).await {
                Ok(result) => result,
                Err(_) => return Err("Bad Path".to_string()),
            };
            let cert = match Certificate::from_pem(&cert_content.as_bytes()) {
                Ok(result) => result,
                Err(_) => return Err("Bad Path".to_string()),
            };
            
            let now = match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
                Ok(n) => n.as_millis(),
                Err(_) => return Err("Clock Went Backwards!".to_string()),
            };
            let payload = Claims {
                id: id,
                exp: now + LIFETIME,
            };
            let header = Header {
                alg: Algorithm::HS256,
                ..Default::default()
            };
            let token = match encode(
                &header,
                &payload,
                &EncodingKey::from_secret(key.as_bytes()),
            ) {
                Ok(token) => token,
                Err(_) => return Err("Error Encoding JWT!".to_string()),
            };
            Ok(CnGateway {
                host,
                token,
                cert: cert,
            })
        
    }
    //
    // CORE
    //
    /// Check mempool info
    pub async fn getmempoolinfo(&self) -> Result<MempoolInfo, String> {
        core::getmempoolinfo(self.host.clone(), self.token.clone(), self.cert.clone()).await
    }
    /// Get balance
    pub async fn getbalance(&self) -> Result<Balance, String> {
        core::getbalance(self.host.clone(), self.token.clone(), self.cert.clone()).await
    }
    /// Get new address
    pub async fn getnewaddress(&self, address_type: AddressType, label: String) -> Result<Address, String> {
        let request = AddressRequest{
            address_type: address_type,
            label: label
        };
        core::getnewaddress(self.host.clone(), self.token.clone(), self.cert.clone(), request).await
    }
    /// Validate onchain address
    pub async fn validateaddress(&self, address: String) -> Result<bool, String> {
        core::validateaddress(self.host.clone(), self.token.clone(), self.cert.clone(), address).await
    }
    //
    // BATCHER
    //
    pub async fn createbatcher(
        &self,
        batcher_label: String,
        conf_target: u64,
    ) -> Result<CreateBatcherResponse, String> {
        let request = CreateBatcherRequest::new(batcher_label, conf_target);
        batcher::createbatcher(self.host.clone(), self.token.clone(), self.cert.clone(), request).await
    }
    pub async fn updatebatcher(
        &self,
        batcher_label: Option<String>,
        batcher_id: Option<String>,
        conf_target: u64,
    ) -> Result<UpdateBatcherResponse, String> {
        let request = UpdateBatcherRequest::new(batcher_label, batcher_id, conf_target);
        batcher::updatebatcher(self.host.clone(), self.token.clone(), self.cert.clone(), request).await
    }
    pub async fn addtobatch(
        &self,
        address: String,
        amount: f64,
        batcher_label: Option<String>,
        webhook_url: Option<String>,
    ) -> Result<BatchInfoResponse, String> {
        let request = AddToBatchRequest::new(address, amount, batcher_label, webhook_url);
        batcher::addtobatch(self.host.clone(), self.token.clone(), self.cert.clone(), request).await
    }
    pub async fn removefrombatch(&self, output_id: u64) -> Result<BatchInfoResponse, String> {
        let request = RemoveFromBatchRequest::new(output_id);
        batcher::removefrombatch(self.host.clone(), self.token.clone(), self.cert.clone(), request).await
    }
    pub async fn getbatcher(
        &self,
        batcher_label: Option<String>,
        batcher_id: Option<String>,
    ) -> Result<BatchInfoResponse, String> {
        let request = GetBatcherRequest::new(batcher_label, batcher_id);
        batcher::getbatcher(self.host.clone(), self.token.clone(), self.cert.clone(), request).await
    }
    pub async fn getbatchdetails(
        &self,
        batcher_id: u64,
        batcher_label: Option<String>,
        txid: Option<String>,
    ) -> Result<BatchDetailResponse, String> {
        let request = GetBatchDetailRequest::new(batcher_id, batcher_label, txid);
        batcher::getbatchdetails(self.host.clone(), self.token.clone(), self.cert.clone(), request).await
    }
    pub async fn listbatchers(&self) -> Result<ListBatchersResponse, String> {
        batcher::listbatchers(self.host.clone(), self.token.clone(), self.cert.clone()).await
    }
    pub async fn batchspend(
        &self,
        batcher_label: Option<String>,
        batcher_id: Option<String>,
        conf_target: Option<u64>,
    ) -> Result<BatchSpendResponse, String> {
        let request = BatchSpendRequest::new(batcher_label, batcher_id, conf_target);
        batcher::batchspend(self.host.clone(), self.token.clone(), self.cert.clone(), request).await
    }
    //
    // WATCHER
    //
    pub async fn watch(
        &self,
        address: String,
        unconfirmed_callback_url: String,
        confirmed_callback_url: String,
        event_message: String,
        label: String,
    ) -> Result<WatchAddress, String> {
        let body = WatchAddressReq::new(
            address,
            unconfirmed_callback_url,
            confirmed_callback_url,
            event_message,
            label,
        );
        watcher::watch(self.host.clone(), self.token.clone(), self.cert.clone(), body).await
    }
    /// Unwatch a bitcoin address
    pub async fn unwatch(&self, address: String) -> Result<UnwatchAddress, String> {

        watcher::unwatch(self.host.clone(), self.token.clone(), self.cert.clone(), address).await
    }
    /// Get addresses currently being watched
    pub async fn watchxpub(
        &self,
        label: String,
        pub32: String,
        path: String,
        nstart: i64,
        unconfirmed_callback_url: String,
        confirmed_callback_url: String,
    ) -> Result<WatchXpub, String> {
        let body = WatchXpubReq::new(
            label,
            pub32,
            path,
            nstart,
            unconfirmed_callback_url,
            confirmed_callback_url,
        );
        watcher::watchxpub(self.host.clone(), self.token.clone(), self.cert.clone(), body).await
    }
    /// Unwatch a bitcoin xpub
    pub async fn unwatchxpubbyxpub(&self, xpub: String) -> Result<UnwatchXpub, String> {
        watcher::unwatchxpubbyxpub(self.host.clone(), self.token.clone(), self.cert.clone(), xpub).await
    }
    /// Get addresses currently being watched
    pub async fn getactivewatches(&self) -> Result<ActiveWatches, String> {
        watcher::getactivewatches(self.host.clone(), self.token.clone(), self.cert.clone()).await
    }
    //
    // LIGHTNING
    //
    /// Ln node info
    pub async fn ln_getinfo(&self) -> Result<LnInfo, String> {
        lightning::ln_getinfo(self.host.clone(), self.token.clone(), self.cert.clone()).await
    }
    /// Get new address to deposit funds to open channels with
    pub async fn ln_newaddr(&self) -> Result<LnFundAddress, String> {
        lightning::ln_newaddr(self.host.clone(), self.token.clone(), self.cert.clone()).await
    }
    /// Get your nodes connection string to share with peers
    pub async fn ln_getconnectionstring(&self) -> Result<LnConnString, String> {
        lightning::ln_getconnectionstring(self.host.clone(), self.token.clone(), self.cert.clone()).await
    }
    /// Decode an invoice
    pub async fn ln_decodebolt11(&self, invoice: String) -> Result<LnBolt11, String> {
        lightning::ln_decodebolt11(self.host.clone(), self.token.clone(), self.cert.clone(), invoice).await
    }
    /// Connect to a given peer and attempt opening a channel and fund it with msatoshis. Get notified at callback_url.
    pub async fn ln_connectfund(
        &self,
        peer: String,
        msatoshis: u128,
        callback_url: String,
    ) -> Result<LnConnectFund, String> {
        let body = LnConnectFundReq::new(peer, msatoshis, callback_url);
        lightning::ln_connectfund(self.host.clone(), self.token.clone(), self.cert.clone(), body).await
    }
    /// Returns the list of unused outputs and funds in open channels
    pub async fn ln_listfunds(&self) -> Result<LnListFunds, String> {
        lightning::ln_listfunds(self.host.clone(), self.token.clone(), self.cert.clone()).await
    }
    /// Returns history of paid invoices
    pub async fn ln_listpays(&self) -> Result<LnListPays, String> {
        lightning::ln_listpays(self.host.clone(), self.token.clone(), self.cert.clone()).await
    }
    /// Returns an array representing hops of nodes to get to the destination node from our node
    pub async fn ln_getroute(
        &self,
        node_id: String,
        msatoshis: u128,
        risk_factor: f32,
    ) -> Result<LnRoutes, String> {
        lightning::ln_getroute(
            self.host.clone(),
            self.token.clone(),
            self.cert.clone(),
            node_id,
            msatoshis,
            risk_factor,
        )
        .await
    }
    /// Withdraw funds from channel back on main chain
    pub async fn ln_withdraw(
        &self,
        address: String,
        satoshis: u128,
        feerate: String,
    ) -> Result<LnWithdraw, String> {
        let body = LnWithdrawReq::new(address, satoshis, feerate);
        lightning::ln_withdraw(self.host.clone(), self.token.clone(), self.cert.clone(), body).await
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn local_bitcoin_testnet() {
        let gatekeeper_ip = "localhost:2009".to_string();
        let kid = "003".to_string();
        let key = "c06f9fc30c50ab7541cefaeb58708fe28babcf7d5ed1767a59685f63d0b63c54".to_string();
        let project_path = env!("CARGO_MANIFEST_DIR");
        let cert_path = format!("{}/certs/cacert.pem", project_path);
        let client = CnGateway::new(
            gatekeeper_ip.clone(),
            kid.clone(),
            key.clone(),
            cert_path.clone(),
        )
        .await
        .unwrap();
        println!("{}\n\n{:#?}", cert_path, client.cert);
        let mempool = client.getmempoolinfo().await.unwrap();
        let address = client.getnewaddress(AddressType::Bech32,"dup".to_string()).await.unwrap();
        let validate_ok = client.validateaddress(address.clone().address).await.unwrap();
        let validate_bad = client.validateaddress("bc1hahahaha".to_string()).await.unwrap();
        assert!(validate_ok);
        assert!(!validate_bad);
        let balance = client.getbalance().await.unwrap();

        println!("mempool: {:#?}", mempool);
        println!("address: {:#?}", address);
        println!("balance: {:#?}", balance);

    }
    #[tokio::test]
    async fn local_ln_testnet() {
        let gatekeeper_ip = "localhost:2009".to_string();
        let kid = "003".to_string();
        let key = "c06f9fc30c50ab7541cefaeb58708fe28babcf7d5ed1767a59685f63d0b63c54".to_string();
        let project_path = env!("CARGO_MANIFEST_DIR");
        let cert_path = format!("{}/certs/cacert.pem", project_path);

        let client = CnGateway::new(
            gatekeeper_ip.clone(),
            kid.clone(),
            key.clone(),
            cert_path.clone(),
        )
        .await
        .unwrap();

        let lninfo = client.ln_getinfo().await.unwrap();
        let newaddr = client.ln_newaddr().await.unwrap();
        let connstr = client.ln_getconnectionstring().await.unwrap();
        let invoice = "lnbc920u1p3khp67pp5mcqxhupukc5te86wfkryerk8f69gg9ptzcep33ry94svm4wvwzqqdqqcqzzgxqyz5vqrzjqwnvuc0u4txn35cafc7w94gxvq5p3cu9dd95f7hlrh0fvs46wpvhdjx4k0kekn630gqqqqryqqqqthqqpyrzjqw8c7yfutqqy3kz8662fxutjvef7q2ujsxtt45csu0k688lkzu3ldjx4k0kekn630gqqqqryqqqqthqqpysp58nxs2nm5wphu234ggawaeul2tnpl6jqc9a0ymfhwpr64vq0k3l4s9qypqsqlkrver3pdxm0teyye0n6y5sje8u90t4j8vpxq3qjwjh9ue46cctj2nzw8fdudfec6nd0e8gx9v485ek7p624j5leeykg70wmv59y3pqqn9ulv2".to_string();
        let bolt11_decoded = client.ln_decodebolt11(invoice).await.unwrap();
        let peer =
            "02eadbd9e7557375161df8b646776a547c5cbc2e95b3071ec81553f8ec2cea3b8c@18.191.253.246:9735"
                .to_string();
        let msatoshis = 3_690_000;
        let callback_url = "http://yourcypherapp/callback/".to_string();
        let fund_stat = client
            .ln_connectfund(peer, msatoshis, callback_url)
            .await
            .err();
        let list_funds = client.ln_listfunds().await.unwrap();
        let list_pays = client.ln_listpays().await.unwrap();

        // let peer = "02b856473d51e796fc5ff6098afa424d5a35a6e06ce5aa83904a4dcc6f457196d3".to_string();
        // let msatoshis = 3511;
        // let risk_factor = 0.1;
        // let routes = client
        //     .ln_getroute(peer, msatoshis, risk_factor)
        //     .await
        //     .unwrap();

        println!("{:#?}", lninfo);
        println!("{:#?}", newaddr);
        println!("{:#?}", connstr);
        println!("{:#?}", bolt11_decoded);
        println!("{:#?}", fund_stat);
        println!("{:#?}", list_funds);
        println!("{:#?}", list_pays);
        // println!("{:#?}", routes);
    }
    #[tokio::test]
    #[ignore]
    async fn docker_cypherappsnet() {
        let gatekeeper_ip = "gatekeeper:2009".to_string();
        let kid = "003".to_string();
        let key = "72c628d3748acb1de54ee683f1c16375732d3ae96f3908cf25a7fedda6ace01c".to_string();
        let project_path = env!("CARGO_MANIFEST_DIR");
        let cert_path = format!("{}/certs/cacert.pem", project_path);

        let client = CnGateway::new(gatekeeper_ip, kid, key, cert_path.clone())
            .await
            .unwrap();
        println!("{}\n\n{:#?}", cert_path, client.cert);
        let mempool = client.getmempoolinfo().await.unwrap();
        println!("{:#?}", mempool);
    }
}
