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

- [x] POST watch
- [x] GET unwatch/2N8DcqzfkYi8CkYzvNNS5amoq3SbAcQNXKp
- [x] GET getactivewatches
- [ ] GET get_txns_by_watchlabel/Label
- [ ] GET get_unused_addresses_by_watchlabel/Label
- [x] POST watchxpub
- [x] GET unwatchxpubbyxpub/upub57Wa4MvRPNyAhxr578mQUdPr6MHwpg3Su875hj8K75AeUVZLXtFeiP52BrhNqDg93gjALU1MMh5UPRiiQPrwiTiuBBBRHzeyBMgrbwkmmkq
- [ ] GET unwatchxpubbylabel/4421
- [ ] POST watchtxid
- [] GET getactivexpubwatches
- [ ] GET getactivewatchesbyxpub/tpubD6NzVbkrYhZ4YR3QK2tyfMMvBghAvqtNaNK1LTyDWcRHLcMUm3ZN2cGm5BS3MhCRCeCkXQkTXXjiJgqxpqXK7PeUSp86DTTgkLpcjMtpKWk
- [ ] GET getactivewatchesbylabel/2219
- [ ] GET conf/b081ca7724386f549cf0c16f71db6affeb52ff7a0d9b606fb2e5c43faffd3387
- [ ] GET getmempoolinfo
- [ ] GET getblockchaininfo
- [ ] GET getblockhash/593104
- [ ] GET getbestblockhash
- [ ] GET getblockinfo/000000006f82a384c208ecfa04d05beea02d420f3f398ddda5c7f900de5718ea
- [ ] GET getbestblockinfo
- [ ] GET gettransaction/af867c86000da76df7ddb1054b273ca9e034e8c89d049b5b2795f9f590f67648
- [ ] GET executecallbacks
- [ ] GET get_txns_spending
- [ ] GET getbalance
- [ ] GET getbalances
- [ ] GET getnewaddress/bech32
- [ ] POST getnewaddress (with label)
- [ ] GET validateaddress/address
- [ ] POST spend
- [ ] POST bumpfee
- [ ] GET batchspend
- [ ] GET deriveindex/25-30
- [ ] POST derivepubpath
- [ ] GET deriveindex_bitcoind/25-30
- [ ] POST derivepubpath_bitcoind
- [x] GET ln_getinfo
- [ ] POST ln_create_invoice
- [ ] POST ln_pay
- [x] GET ln_newaddr
- [x] GET ln_getconnectionstring
- [x] POST ln_connectfund
- [ ] GET ln_getinvoice/label
- [ ] GET ln_delinvoice/label
- [x] GET ln_decodebolt11/bolt11
- [ ] GET ln_listpeers
- [x] GET ln_listfunds
- [x] GET ln_listpays
- [x] GET ln_getroute/<node_id>/<msatoshi>/<?riskfactor>
- [x] POST ln_withdraw
- [ ] POST ots_stamp
- [ ] GET ots_getfile/1ddfb769eb0b8876bc570e25580e6a53afcf973362ee1ee4b54a807da2e5eed7
- [ ] POST ots_verify
- [ ] POST ots_info
- [ ] POST createbatcher
- [ ] POST updatebatcher
- [ ] POST addtobatch
- [ ] POST removefrombatch
- [ ] POST batchspend
- [ ] POST getbatcher
- [ ] POST getbatchdetails
- [ ] GET listbatchers
- [ ] POST bitcoin_estimatesmartfee
