// extern crate cngateway;
// extern crate tokio;

#[tokio::main]
async fn main() {
//     let output = std::process::Command::new("docker")
//     .arg("inspect")
//     .arg("-f {{ .NetworkSettings.Networks.cyphernodenet.IPAddress }}")
//     .arg("cyphernode_proxy_1")
//     .output()
//     .expect("Failed to execute command");
    
//     let cngateway_ip = std::str::from_utf8(output.stdout.as_slice())
//         .unwrap()
//         .replace(" ", "")
//         .to_string();

//     let client = cngateway::CnProxyClient::new(cngateway_ip);
//     client.ping().await.unwrap();
//     let lninfo = client.ln_getinfo().await.unwrap();
//     let newaddr = client.ln_newaddr().await.unwrap();
//     let connstr = client.ln_getconnectionstring().await.unwrap();
//     let some_invoice = "lnbc920u1p3khp67pp5mcqxhupukc5te86wfkryerk8f69gg9ptzcep33ry94svm4wvwzqqdqqcqzzgxqyz5vqrzjqwnvuc0u4txn35cafc7w94gxvq5p3cu9dd95f7hlrh0fvs46wpvhdjx4k0kekn630gqqqqryqqqqthqqpyrzjqw8c7yfutqqy3kz8662fxutjvef7q2ujsxtt45csu0k688lkzu3ldjx4k0kekn630gqqqqryqqqqthqqpysp58nxs2nm5wphu234ggawaeul2tnpl6jqc9a0ymfhwpr64vq0k3l4s9qypqsqlkrver3pdxm0teyye0n6y5sje8u90t4j8vpxq3qjwjh9ue46cctj2nzw8fdudfec6nd0e8gx9v485ek7p624j5leeykg70wmv59y3pqqn9ulv2".to_string();
//     let bolt11_decoded = client.ln_decodebolt11(some_invoice).await.unwrap();
//     let peer =
//         "02b856473d51e796fc5ff6098afa424d5a35a6e06ce5aa83904a4dcc6f457196d3@149.56.123.56:9735"
//             .to_string();
//     let msatoshis = 3_690_000;
//     let callback_url = "yourcypherapp/callback/".to_string();
//     let fund_stat = client
//         .ln_connectfund(peer, msatoshis, callback_url)
//         .await
//         .err();
//     println!("{:#?}", lninfo);
//     println!("{:#?}", newaddr);
//     println!("{:#?}", connstr);
//     println!("{:#?}", bolt11_decoded);
//     println!("{:#?}", fund_stat);
    
}