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

## Current:

- Lightning (in progress)

## Pending

- Watcher
- Batcher 
- Bitcoin
