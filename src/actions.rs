extern crate serde;

use editor::Editor;
use serde_json::{Value};
use self::serde::ser::{Serializer, Serialize, SerializeMap};
use std::sync::{Arc, Mutex};
use std::fmt;

#[derive(Serialize, Deserialize, Debug)]
pub enum Method {
    #[serde(rename = "connect")]
    Connect
}

#[derive(Copy, Clone)]
pub enum ResponseErr {
    MalformedInput,
    InvalidMethod,
    MissingMethod,
    TestError,
    DeserializationError
}

pub enum ResponseOk {
    ConnectResponse(ConnRespStruct),
    Ok
}

#[derive(Serialize)]
pub struct ConnRespStruct {
    pub test_field: i32
}

impl fmt::Display for ResponseErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &ResponseErr::MalformedInput => { write!(f, "malformed input") }
            &ResponseErr::InvalidMethod => { write!(f, "invalid method") }
            &ResponseErr::MissingMethod => { write!(f, "missing method") }
            &ResponseErr::TestError => { write!(f, "test error") },
            &ResponseErr::DeserializationError => { write!(f, "deserialization error") }
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ConnectRequest {
    #[serde(rename = "clientId")]
    pub client_id: String,
    pub method: Method
}

pub struct Response(pub Result<ResponseOk, ResponseErr>);

impl Serialize for ResponseErr {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        // Serialize as objection with two fields, an error code and an error message
        let mut map = serializer.serialize_map(Some(2))?;
        map.serialize_entry("code", &(*self as i32))?;
        map.serialize_entry("message", &self.to_string())?;
        map.end()
    }
}

impl Serialize for ResponseOk {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        match self {
            &ResponseOk::Ok => {
                // Serialize generic Ok result as an empty object
                let map = serializer.serialize_map(Some(0))?;
                map.end()
            }
            &ResponseOk::ConnectResponse(ref s) => {
                s.serialize(serializer)
            }
        }
    }
}

impl Serialize for Response {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        match self.0 {
            Ok(ref s) => {
                let mut map = serializer.serialize_map(Some(2))?;
                map.serialize_entry("error", &false)?;
                map.serialize_entry("payload", &s)?;
                map.end()
            }
            Err(ref err) => {
                let mut map = serializer.serialize_map(Some(2))?;
                map.serialize_entry("error", &true)?;
                map.serialize_entry("payload", &err)?;
                map.end()
            }
        }
    }
}

pub trait Request {
    fn exec(&self, editor: &mut Arc<Mutex<Editor>>) -> Response;
}

impl Request for ConnectRequest {
    fn exec(&self, editor: &mut Arc<Mutex<Editor>>) -> Response {
        debug!("Calling Message exec() for ConnectMessage {:?}", self);
        let ed = editor.lock().unwrap();
        Response(Ok(ResponseOk::ConnectResponse(ConnRespStruct {
            test_field: 974
        })))
    }
}
