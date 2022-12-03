# cngateway

a simple client library for cyphernode gatekeeper

## setup

- add cert.pem to the root directory for tests
- if you are testing outside the cyphernodeappsnet, only run `cargo test local`
- if you want to test within the cyphernodeappsnet, use the docker-compose and run `docker exec -it cngateway sh -c 'cargo test cyphernodeappsnet'`

## usage

```rust
let client = CnGatewayClient::new(url,api_key,None);
// last None may be Some(std::env::var(CYPHERNODE_GATEKEEPER_CERT_CA))
client.ping().await?;
let lninfo = client.ln_getinfo().await?;
let newaddr = client.ln_newaddr().await?;
let connstr = client.ln_getconnectionstring().await?;
let some_invoice = "lnbc920u1p3khp67pp5mcqxhupukc5te86wfkryerk8f69gg9ptzcep33ry94svm4wvwzqqdqqcqzzgxqyz5vqrzjqwnvuc0u4txn35cafc7w94gxvq5p3cu9dd95f7hlrh0fvs46wpvhdjx4k0kekn630gqqqqryqqqqthqqpyrzjqw8c7yfutqqy3kz8662fxutjvef7q2ujsxtt45csu0k688lkzu3ldjx4k0kekn630gqqqqryqqqqthqqpysp58nxs2nm5wphu234ggawaeul2tnpl6jqc9a0ymfhwpr64vq0k3l4s9qypqsqlkrver3pdxm0teyye0n6y5sje8u90t4j8vpxq3qjwjh9ue46cctj2nzw8fdudfec6nd0e8gx9v485ek7p624j5leeykg70wmv59y3pqqn9ulv2".to_string();
let bolt11_decoded = client.ln_decodebolt11(some_invoice).await?;
let peer =
  "02b856473d51e796fc5ff6098afa424d5a35a6e06ce5aa83904a4dcc6f457196d3@149.56.123.56:9735"
  .to_string();
let msatoshis = 3_690_000;
let callback_url = "yourcypherapp/callback/".to_string();
let fund_stat = client.ln_connectfund(
  peer, 
  msatoshis, 
  callback_url
).await?
```

The cyphernode api request and response json types are internally converted into a native rust types.

The client feeds request parameters (if any) as function inputs.

Example: `ln_connectfund`

```rust
let fund_stat = client.ln_connectfund(
  peer, 
  msatoshis, 
  callback_url
).await?
```

The client recieves the response as a native rust type.

```rust
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LnConnectFund {
    pub result: String,
    pub txid: String,
    #[serde(rename = "channelId")]
    pub channel_id: String,
}
```

NOTE: Rust uses snake_case for variable and function names. Cyphernode uses camelCase. 
All datatypes returned will internally be snake_case.

## API

- [x] POST http://cyphernode:8888/watch
- [x] GET http://cyphernode:8888/unwatch/2N8DcqzfkYi8CkYzvNNS5amoq3SbAcQNXKp
- [x] GET http://cyphernode:8888/getactivewatches
- [ ] GET http://cyphernode:8888/get_txns_by_watchlabel/Label
- [ ] GET http://cyphernode:8888/get_unused_addresses_by_watchlabel/Label
- [x] POST http://cyphernode:8888/watchxpub
- [x] GET http://cyphernode:8888/unwatchxpubbyxpub/upub57Wa4MvRPNyAhxr578mQUdPr6MHwpg3Su875hj8K75AeUVZLXtFeiP52BrhNqDg93gjALU1MMh5UPRiiQPrwiTiuBBBRHzeyBMgrbwkmmkq
- [ ] GET http://cyphernode:8888/unwatchxpubbylabel/4421
- [ ] POST http://cyphernode:8888/watchtxid
- [] GET http://cyphernode:8888/getactivexpubwatches
- [ ] GET http://cyphernode:8888/getactivewatchesbyxpub/tpubD6NzVbkrYhZ4YR3QK2tyfMMvBghAvqtNaNK1LTyDWcRHLcMUm3ZN2cGm5BS3MhCRCeCkXQkTXXjiJgqxpqXK7PeUSp86DTTgkLpcjMtpKWk
- [ ] GET http://cyphernode:8888/getactivewatchesbylabel/2219
- [ ] GET http://cyphernode:8888/conf/b081ca7724386f549cf0c16f71db6affeb52ff7a0d9b606fb2e5c43faffd3387
- [ ] GET http://cyphernode:8888/getmempoolinfo
- [ ] GET http://cyphernode:8888/getblockchaininfo
- [ ] GET http://cyphernode:8888/getblockhash/593104
- [ ] GET http://cyphernode:8888/getbestblockhash
- [ ] GET http://cyphernode:8888/getblockinfo/000000006f82a384c208ecfa04d05beea02d420f3f398ddda5c7f900de5718ea
- [ ] GET http://cyphernode:8888/getbestblockinfo
- [ ] GET http://cyphernode:8888/gettransaction/af867c86000da76df7ddb1054b273ca9e034e8c89d049b5b2795f9f590f67648
- [ ] GET http://cyphernode:8888/executecallbacks
- [ ] GET http://cyphernode:8888/get_txns_spending
- [ ] GET http://cyphernode:8888/getbalance
- [ ] GET http://cyphernode:8888/getbalances
- [ ] GET http://cyphernode:8888/getnewaddress/bech32
- [ ] POST http://cyphernode:8888/getnewaddress (with label)
- [ ] GET http://cyphernode:8888/validateaddress/address
- [ ] POST http://cyphernode:8888/spend
- [ ] POST http://cyphernode:8888/bumpfee
- [ ] GET http://cyphernode:8888/batchspend
- [ ] GET http://cyphernode:8888/deriveindex/25-30
- [ ] POST http://cyphernode:8888/derivepubpath
- [ ] GET http://cyphernode:8888/deriveindex_bitcoind/25-30
- [ ] POST http://cyphernode:8888/derivepubpath_bitcoind
- [x] GET http://cyphernode:8888/ln_getinfo
- [ ] POST http://cyphernode:8888/ln_create_invoice
- [ ] POST http://cyphernode:8888/ln_pay
- [x] GET http://cyphernode:8888/ln_newaddr
- [x] GET http://cyphernode:8888/ln_getconnectionstring
- [x] POST http://cyphernode:8888/ln_connectfund
- [ ] GET http://cyphernode:8888/ln_getinvoice/label
- [ ] GET http://cyphernode:8888/ln_delinvoice/label
- [x] GET http://cyphernode:8888/ln_decodebolt11/bolt11
- [ ] GET http://cyphernode:8888/ln_listpeers
- [x] GET http://cyphernode:8888/ln_listfunds
- [x] GET http://cyphernode:8888/ln_listpays
- [x] GET http://cyphernode:8888/ln_getroute/<node_id>/<msatoshi>/<?riskfactor>
- [x] POST http://192.168.111.152:8080/ln_withdraw
- [ ] POST http://cyphernode:8888/ots_stamp
- [ ] GET http://cyphernode:8888/ots_getfile/1ddfb769eb0b8876bc570e25580e6a53afcf973362ee1ee4b54a807da2e5eed7
- [ ] POST http://cyphernode:8888/ots_verify
- [ ] POST http://cyphernode:8888/ots_info
- [ ] POST http://cyphernode:8888/createbatcher
- [ ] POST http://cyphernode:8888/updatebatcher
- [ ] POST http://cyphernode:8888/addtobatch
- [ ] POST http://cyphernode:8888/removefrombatch
- [ ] POST http://cyphernode:8888/batchspend
- [ ] POST http://cyphernode:8888/getbatcher
- [ ] POST http://cyphernode:8888/getbatchdetails
- [ ] GET http://cyphernode:8888/listbatchers
- [ ] POST http://cyphernode:8888/bitcoin_estimatesmartfee
