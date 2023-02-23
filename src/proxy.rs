use serde_derive::{Deserialize,Serialize};
use crate::e::{ErrorKind, S5Error};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProxyHello {
    pub hello: String,
}
impl ProxyHello {
    pub fn from_str(stringified: &str) -> Result<ProxyHello, S5Error> {
        match serde_json::from_str(stringified) {
            Ok(result) => Ok(result),
            Err(_) => Err(S5Error::new(
                ErrorKind::Internal,
                "Error stringifying ProxyHello",
            )),
        }
    }
}

pub async fn helloworld(ip: String) -> Result<(), String> {
    let full_url: String = format!("http://{}:8888/helloworld", ip).to_string();

    match ureq::get(&full_url).call() {
        Ok(response) => match ProxyHello::from_str(&response.into_string().unwrap()) {
            Ok(_) => Ok(()),
            Err(e) => Err(e.message),
        },
        Err(e) => Err(e.to_string()),
    }
}