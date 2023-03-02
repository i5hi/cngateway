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
///     gatekeeper_ip,
///     kid,
///     key,
///     cert_path,
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
    UpdateBatcherRequest, UpdateBatcherResponse, Batchers,
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
        host: impl ToString,
        id: impl ToString,
        key: impl ToString,
        cert_path: impl ToString,
    ) -> Result<Self, String> {
            let cert_content = match tokio::fs::read_to_string(cert_path.to_string()).await {
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
                id: id.to_string(),
                exp: now + LIFETIME,
            };
            let header = Header {
                alg: Algorithm::HS256,
                ..Default::default()
            };
            let token = match encode(
                &header,
                &payload,
                &EncodingKey::from_secret(key.to_string().as_bytes()),
            ) {
                Ok(token) => token,
                Err(_) => return Err("Error Encoding JWT!".to_string()),
            };
            Ok(CnGateway {
                host: host.to_string(),
                token: token,
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
    pub async fn getnewaddress(&self, address_type: AddressType, label: impl ToString) -> Result<Address, String> {
        let request = AddressRequest{
            address_type: address_type,
            label: label.to_string()
        };
        core::getnewaddress(self.host.clone(), self.token.clone(), self.cert.clone(), request).await
    }
    /// Validate onchain address
    pub async fn validateaddress(&self, address: impl ToString) -> Result<bool, String> {
        core::validateaddress(self.host.clone(), self.token.clone(), self.cert.clone(), address.to_string()).await
    }
    //
    // BATCHER
    //
    pub async fn createbatcher(
        &self,
        batcher_label: impl ToString,
        conf_target: u64,
    ) -> Result<CreateBatcherResponse, String> {
        let request = CreateBatcherRequest::new(batcher_label.to_string(), conf_target);
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
        address: impl ToString,
        amount: f64,
        batcher_label: impl ToString,
        webhook_url: Option<String>,
    ) -> Result<BatchInfoResponse, String> {
        let request = AddToBatchRequest::new(address.to_string(), amount, batcher_label.to_string(), webhook_url);
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
    pub async fn listbatchers(&self) -> Result<Batchers, String> {
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
        address: impl ToString,
        unconfirmed_callback_url: impl ToString,
        confirmed_callback_url: impl ToString,
        label: impl ToString,
        event_message: Option<String>,
    ) -> Result<WatchAddress, String> {
        let body = WatchAddressReq::new(
            address.to_string(),
            unconfirmed_callback_url.to_string(),
            confirmed_callback_url.to_string(),
            event_message,
            label.to_string(),
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
        label: impl ToString,
        pub32: impl ToString,
        path: impl ToString,
        nstart: i64,
        unconfirmed_callback_url: impl ToString,
        confirmed_callback_url: impl ToString,
    ) -> Result<WatchXpub, String> {
        let body = WatchXpubReq::new(
            label.to_string(),
            pub32.to_string(),
            path.to_string(),
            nstart,
            unconfirmed_callback_url.to_string(),
            confirmed_callback_url.to_string(),
        );
        watcher::watchxpub(self.host.clone(), self.token.clone(), self.cert.clone(), body).await
    }
    /// Unwatch a bitcoin xpub
    pub async fn unwatchxpubbyxpub(&self, xpub: impl ToString) -> Result<UnwatchXpub, String> {
        watcher::unwatchxpubbyxpub(self.host.clone(), self.token.clone(), self.cert.clone(), xpub.to_string()).await
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
    pub async fn ln_decodebolt11(&self, invoice: impl ToString) -> Result<LnBolt11, String> {
        lightning::ln_decodebolt11(self.host.clone(), self.token.clone(), self.cert.clone(), invoice.to_string()).await
    }
    /// Connect to a given peer and attempt opening a channel and fund it with msatoshis. Get notified at callback_url.
    pub async fn ln_connectfund(
        &self,
        peer: impl ToString,
        msatoshis: u128,
        callback_url: impl ToString,
    ) -> Result<LnConnectFund, String> {
        let body = LnConnectFundReq::new(peer.to_string(), msatoshis, callback_url.to_string());
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
        address: impl ToString,
        satoshis: u128,
        feerate: impl ToString,
    ) -> Result<LnWithdraw, String> {
        let body = LnWithdrawReq::new(address.to_string(), satoshis, feerate.to_string());
        lightning::ln_withdraw(self.host.clone(), self.token.clone(), self.cert.clone(), body).await
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn local_bitcoin_testnet() {
        let client = new_client_localhost().await;

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
    async fn local_batcher_testnet() {
        let client = new_client_localhost().await;

        // default slow and fast batchers already exist
        // let batcher = client.createbatcher("sm11p".to_string(), 3).await.unwrap();
        // println!("batcher: {:#?}", batcher);
        let address = "tb1qks9n9440qesu5hvnafc7m2hvuemtynwmwmj2va";
        let amount = 0.00003000;
        let batcher_label = "default";
        let batcher_id = 1;
        let webhook_url: Option<String> = None;

        let batch_details = client.getbatchdetails(batcher_id, None, None).await.unwrap();
        println!("batch_details: {:#?}", batch_details);

        // let add_status = client.addtobatch(address, amount, batcher_label, webhook_url.clone()).await.unwrap();
        // println!("add_status: {:#?}", add_status);

        let batchers =client.listbatchers().await.unwrap();
        println!("batchers: {:#?}", batchers);

        let batcher1 =client.getbatchdetails(batcher_id, None, None).await.unwrap();
        println!("batcher1: {:#?}", batcher1);

        // let spend_status = client.batchspend(Some(batcher_label.to_string()), None, None).await.unwrap();
        // println!("spend_status: {:#?}", spend_status);

    }
    #[tokio::test]
    async fn local_ln_testnet() {
        let client = new_client_localhost().await;

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
        let client = new_client_cyphernodeappsnet().await;
        let mempool = client.getmempoolinfo().await.unwrap();
        println!("{:#?}", mempool);
    }
    /*
    

    HELPERS
    

     */
    async fn new_client_localhost()->CnGateway{
        let gatekeeper_ip = "localhost:2009".to_string();
        let kid = "003".to_string();
        let key = "c06f9fc30c50ab7541cefaeb58708fe28babcf7d5ed1767a59685f63d0b63c54".to_string();
        let project_path = env!("CARGO_MANIFEST_DIR");
        let cert_path = format!("{}/certs/cacert.pem", project_path);
        CnGateway::new(
            gatekeeper_ip.clone(),
            kid.clone(),
            key.clone(),
            cert_path.clone(),
        )
        .await
        .unwrap()
    }
    async fn new_client_cyphernodeappsnet()->CnGateway{
        let gatekeeper_ip = "gatekeeper:2009".to_string();
        let kid = "003".to_string();
        let key = "c06f9fc30c50ab7541cefaeb58708fe28babcf7d5ed1767a59685f63d0b63c54".to_string();
        let project_path = env!("CARGO_MANIFEST_DIR");
        let cert_path = format!("{}/certs/cacert.pem", project_path);
        CnGateway::new(
            gatekeeper_ip.clone(),
            kid.clone(),
            key.clone(),
            cert_path.clone(),
        )
        .await
        .unwrap()
    }
}
