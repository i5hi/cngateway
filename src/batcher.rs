use crate::lib::e::{ErrorKind, S5Error};
use serde::{Deserialize, Serialize};
use ureq;


// ip=$(docker container inspect -f '{{ .NetworkSettings.Networks.cyphernodenet.IPAddress }}' cyphernode_proxy_1)
// POST http://$ip:8888/createbatcher
/*
REQUEST{
    "batcherLabel":"lowfees",
    "confTarget":32
}
RESPONSE{
  "result": {
    "batcherId": 1
  },
  "error": null
}
*/

// POST http://cyphernode:8888/updatebatcher
/*
REQUEST{
    "batcherLabel":"fast",
    "confTarget":2
}
RESPONSE{
  "result": {
    "batcherId": 1,
    "batcherLabel": "fast",
    "confTarget": 2
  },
  "error": null
}
*/

// POST http://cyphernode:8888/addtobatch
/*
REQUEST{
    "address":"2N8DcqzfkYi8CkYzvNNS5amoq3SbAcQNXKp",
    "amount":0.00233
    "batcherLabel":"lowfees",
    "webhookUrl":"https://myCypherApp:3000/batchExecuted"
}
RESPONSE{
  "result": {
    "batcherId": 1,
    "outputId": 34,
    "nbOutputs": 7,
    "oldest": "2020-09-09 14:00:01",
    "total": 0.04016971
  },
  "error": null
}
*/

// POST http://cyphernode:8888/removefrombatch
/*
REQUEST{
    "outputId":72
}
RESPONSE{
  "result": {
    "batcherId": 1,
    "outputId": 72,
    "nbOutputs": 6,
    "oldest": "2020-09-09 14:00:01",
    "total": 0.03783971
  },
  "error": null
}

*/


// POST http://cyphernode:8888/getbatcher
/*
REQUEST{}
or
{"batcherId":34}
or
{"batcherLabel":"fastest"}


RESPONSE{
  "result": {
    "batcherId": 1,
    "batcherLabel": "default",
    "confTarget": 6,
    "nbOutputs": 12,
    "oldest": 123123,
    "total": 0.86990143
  },
  "error": null
}
*/
// POST http://cyphernode:8888/getbatchdetails
/*
REQUEST{
}
or
{"batcherId":34}
or
{"batcherLabel":"fastest","txid":"af867c86000da76df7ddb1054b273ca9e034e8c89d049b5b2795f9f590f67648"}

RESPONSE{
  "result": {
    "batcherId": 34,
    "batcherLabel": "Special batcher for a special client",
    "confTarget": 6,
    "nbOutputs": 83,
    "oldest": 123123,
    "total": 10.86990143,
    "txid": "af867c86000da76df7ddb1054b273ca9e034e8c89d049b5b2795f9f590f67648",
    "hash": "af867c86000da76df7ddb1054b273ca9e034e8c89d049b5b2795f9f590f67648",
    "details": {
      "firstseen": 123123,
      "size": 424,
      "vsize": 371,
      "replaceable":true,
      "fee": 0.00004112
    },
    "outputs": {
      "1abc": 0.12,
      "3abc": 0.66,
      "bc1abc": 2.848,
      ...
    }
  },
  "error": null
}
*/

// GET http://cyphernode:8888/listbatchers
/*
RESPONSE{
  "result": [
    {"batcherId":1,"batcherLabel":"default","confTarget":6,"nbOutputs":12,"oldest":123123,"total":0.86990143},
    {"batcherId":2,"batcherLabel":"lowfee","confTarget":32,"nbOutputs":44,"oldest":123123,"total":0.49827387},
    {"batcherId":3,"batcherLabel":"highfee","confTarget":2,"nbOutputs":7,"oldest":123123,"total":4.16843782}
  ],
  "error": null
}
*/
// GET http://cyphernode:8888/batchspend
/*
RESPONSE{
  "status": "accepted",
  "hash": "af867c86000da76df7ddb1054b273ca9e034e8c89d049b5b2795f9f590f67648"
}

*/
#[cfg(test)]
mod tests {
    use super::*;
}