use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Serialize, Deserialize, Debug)]
pub struct WebSocketMessage {
    pub status: String,
    pub key: String,
    pub data: f64,
}


pub fn _new(status: String, key: String, data: f64) -> WebSocketMessage{
        WebSocketMessage{
            status: status,
            key: key,
            data: data
        }
}

#[derive(Debug, PartialEq)]
pub enum Status {
    Authenticate,
    Aggregate,
    Close,
}

impl FromStr for Status {

    type Err = ();

    fn from_str(input: &str) -> Result<Status, Self::Err> {
        match input {
            "Authenticate"  => Ok(Status::Authenticate),
            "Aggregate"  => Ok(Status::Aggregate),
            "Close"  => Ok(Status::Close),
            _      => Err(()),
        }
    }
}

impl Status {
    
    pub fn _to_str(&self) -> String {
        match self {
            Status::Authenticate => String::from("Authenticate"),
            Status::Aggregate  => String::from("Aggregate".to_string()),
            Status::Close  => String::from("Close".to_string()),
        }
    }
}