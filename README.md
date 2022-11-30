# cnproxy

a simple client library for cyphernode gatekeeper

## usage

```rust
let client = CnGatewayClient::new(url,api_key,None);// last None may be Some
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

## test

Test with cli output:

```bash
# Requires an instance of cyphernode running
cargo test -- --nocapture
```

## Current:

- Lightning (in progress)

## Pending

- Watcher
- Batcher 
- Bitcoin

                                 ...                        
                               .:^.                         
                      :      .^~:  .                        
                    .J#~     :7!^:.~^                :.     
  ~J^              7P@@#P.   ^7~:.~^^~7::.       :^.:P:     
  ~@#^~5~      .::YB#@@@@?   ^?777?77?!!~!!!^!^~:JGJ:G^^    
  :&@&@@@GJ^   7PYB#@@@@@B   .~~7JJ7!???7~~~^!~~~7!7^5?5    
   B#@@@@@@P  .G#&&#@@@@@B^  !!!&@@@57! !??77?7?!^7~^77J    
   5#@@@@@@@!^!&@@&&@@@#&#! .?!!B##&J?7: .. . .. ~~ ~P??    
   ?B@@@@@@@PJP&@@@@@@@B##J..?!^:^^^~J7:         .:!7PJ~    
   ~5@@@@@@@#PG&@@@@@@@G#@P..?!!B7:JP!?!!?!!!77!. :J?P7     
   .?@@@@@@@&G#@@@@@@@@PB@#. !!!! ~PP^~~~!!7~~~!~^ :7Y.     
    :B@@@@@@@B@@@@@@@@@5B@&:.!!7!.:7?!^?^ :!7!!~!!^ ^.      
     .7G@@@@@@@@@@@@@@@GB@#?:7!!!~~~!?.?^:!?Y?YJJ7~         
       .!B@@@@@@@@@@@@@##@J5.~!!.!~^Y5^7G5. .....           
          ~Y#@@@@@@@@@@@&@Y#:.!^^!!!J?!7!BP~          ..    
             ~JPB#&@@@@@5&@@Y ^!!!!~^:^:^~^.         ^!Y^.  
      .?Y:       .:~7?JP!P@@G  ~?7!:^:                :5!J  
   .!5&@@#~^          7P?&@@#:.!!7!!?!~.           .~:~Y~J  
   7@@@@@&5@?7:    :7G@##@@@&:.!!~~!??~?~.     . . !5~!Y!J  
  JG@@@@@&G@GPB  .G&@@@@@@@G~ .P&?~7...#@5..  .Y~~~~~!^!~Y  
  #@@@@@@&#@#B@^.J@@@@@@@@G     7P^7^~:JJ5J7..^:!7~~!!!G!J  
  G@@@@@@@&@&@@7~B@@@@@@@#:      !!~!7~?JYY7 :~?7Y7?~!~^7J  
  ~@@@@@@@@@@@@J?&@@@@@@&~        .^  JJ!!~!  ^!.!?!~!~^7J  
   5?7#@@@@@@@@YY@@@@@@@5: !^ .?57:7~?75!J!7..YP:.!!~^^:^^  
   .  ^&@@@@@@@YB@@@@@@@G7P@7 7P5!~J7?7J7J~^!:PY:^5!!!.     
       7GG@@@@@P&@@@@@@@&&@@J.~7.  .:::::^:^!~?!:!?!~!.     
       ..^G@@@@#@@@@@@@@@@@@Y   ^:         .!^755Y?!::      
         .!&@@@@@@@@@@@@@@@@G  .~::!!~!~!!. 7^Y@@@B:        
           P@@@@@@@@@@@@@@@@Y .!~^^!!!^!!!~^!~JBBB?         
           ?@@@B@@@@@@@@@@@&!  J?!::~7!!~~^!77~~7!:         
           7G&!!#@@@@@@@@@@&^  !5!:7^^!~~77~!~^^!~.         
           !7? :7@@@@@@@@@@B.  !5!~?!!^: ~Y~Y: :^.          
           ~^   .G&@@@@@@@5G.  :?7:~!!^!777!Y.   .          
           ..    ~B@@@@@@#?5   .7J::~!!7!~!!J               
                  G@@@@@@P^7   ^!7^.~~~~^:^~~               
                  5B@@&@@! .   :~^  ^. :^^^^:               
                  :J@@G5&.     ~!   ::  :^^..               
                   7@@Y^J      ^~   ...::^.                 
                   7@@7 !      :.       :^.                 
                   7G&~ ^                ^^                 
                   ^!#^                  .^                 
                    .B:                   :                 
                     Y.                                     
                     7                                      
                     ^ 