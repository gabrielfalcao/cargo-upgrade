use crate::{HttpResponse, Result};

use reqwest::blocking::Response;
use serde::Deserialize;

pub trait FromResponse: for<'a> Deserialize<'a> {
    fn parse(response: Response) -> Result<Self> {
        let response = HttpResponse::from(response);
        // let (_path, bytes) = store_response(&response)?;
    let bytes = response.bytes()?.to_vec();

        Ok(Self::from_json_bytes(bytes)?)
    }

    fn from_json_bytes(bytes: Vec<u8>) -> Result<Self> {
        let string = String::from_utf8(bytes)?;
        Ok(Self::from_json_string(string)?)
    }
    fn from_json_string(json: String) -> Result<Self> {
        let model = serde_json::from_str::<Self>(&json)?;
        Ok(model)
    }
    fn from_response(response: Response) -> Result<Self> {
        Ok(Self::from_json_bytes(response.bytes()?.to_vec())?)
    }
}
