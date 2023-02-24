# cngateway

a client library for cyphernode gatekeeper

# NOTE

EXTREME BETA: 
We need help building this client. It is not safe for production use yet. 

## setup

- add `certs/cert.pem` to the root directory for tests

- if you are testing outside the cyphernodeappsnet (gatekeeper bound to localhost), only run 

`cargo test local`

- if you want to test within the cyphernodeappsnet, use the docker-compose and run 

`docker exec -it cngateway sh -c 'cargo test cyphernodeappsnet'`

## usage
```
  let gatekeeper_ip = "gatekeeper:2009".to_string(); // if you are connected to cyphernodeappsnet IF NOT expose gatekeeper outside network and use localhost
  let kid = "003".to_string();
  let key = "c06f9fc30c50ab7541cefaeb58708fe28babcf7d5ed1767a59685f63d0b63c54".to_string();
  let cert_path = "/path/to/cacert.pem";
  let client = CnGateway::new(
    gatekeeper_ip,
    kid,
    key,
    cert_path,
  )
  .await?;
  // Use bitcoin core
  let mempool = client.getmempoolinfo().await?;
  let balance = client.getbalance().await?;
  let address = client.getnewaddress(AddressType::Bech32,"dup".to_string()).await?; // uses the POST api format {address_type, label}
  // Use lightning
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
- [ ] GET getactivexpubwatches
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
- [x] GET getnewaddress/bech32
- [x] POST getnewaddress (with label)
- [x] GET validateaddress/address
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
- [x] POST createbatcher
- [x] POST updatebatcher
- [x] POST addtobatch
- [x] POST removefrombatch
- [x] POST batchspend
- [x] POST getbatcher
- [x] POST getbatchdetails
- [x] GET listbatchers
- [ ] POST bitcoin_estimatesmartfee
