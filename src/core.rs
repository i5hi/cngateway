use ureq;
use serde::{Deserialize, Serialize};
use crate::lib::e::{ErrorKind, S5Error};

// GET http://cyphernode:8888/getmempoolinfo
/*
RESPONSE{
  "size": 25,
  "bytes": 5462,
  "usage": 34736,
  "maxmempool": 64000000,
  "mempoolminfee": 1e-05,
  "minrelaytxfee": 1e-05
}
*/

// GET http://cyphernode:8888/getblockchaininfo
/* 
RESPONSE{
  "chain": "test",
  "blocks": 1486864,
  "headers": 1486864,
  "bestblockhash": "000000000000002fb99d683e64bbfc2b7ad16f9a425cf7be77b481fb1afa363b",
  "difficulty": 13971064.71015782,
  "mediantime": 1554149114,
  "verificationprogress": 0.9999994536561675,
  "initialblockdownload": false,
  "chainwork": "000000000000000000000000000000000000000000000103ceb57a5896f347ce",
  "size_on_disk": 23647567017,
  "pruned": false,
  "softforks": [
    {
      "id": "bip34",
      "version": 2,
      "reject": {
        "status": true
      }
    },
    {
      "id": "bip66",
      "version": 3,
      "reject": {
        "status": true
      }
    },
    {
      "id": "bip65",
      "version": 4,
      "reject": {
        "status": true
      }
    }
  ],
  "bip9_softforks": {
    "csv": {
      "status": "active",
      "startTime": 1456790400,
      "timeout": 1493596800,
      "since": 770112
    },
    "segwit": {
      "status": "active",
      "startTime": 1462060800,
      "timeout": 1493596800,
      "since": 834624
    }
  },
  "warnings": "Warning: unknown new rules activated (versionbit 28)"
}
*/

// GET http://cyphernode:8888/getbalance
/*
RESPONSE{
  "balance":1.51911837
}
*/

// POST http://cyphernode:8888/bitcoin_estimatesmartfee
/*
REQUEST{
    "confTarget":2
}
RESPONSE{
    "result": {
      "feerate": 0.00001000,
      "blocks": 4
    },
    "error": null,
    "id": null
}
*/


#[cfg(test)]
mod tests {
    use super::*;

}