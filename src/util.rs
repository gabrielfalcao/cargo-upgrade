use crate::{HttpResponse, Result};
use iocore::Path;
use reqwest::blocking::Request;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use slugify_filenames::slugify_string;
use std::string::ToString;

pub fn setup_logger(level: log::LevelFilter)  {
    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "[{}] {}",
                record.level(),
                message
            ))
        })
        .level(level)
        .chain(std::io::stderr())
        .apply().unwrap();
}

pub fn store_response(response: &HttpResponse) -> Result<(Path, Vec<u8>)> {
    let path = Path::new(format!(
        "{name}.response.json",
        name = slugify_string(response.url().path(), true)?
    ));
    let bytes = response.bytes()?.to_vec();
    path.write(&bytes)?;
    Ok((path, bytes))
}

pub fn store_request(request: &Request) -> Result<(Path, Vec<u8>)> {
    let path = Path::new(format!(
        "{name}.request.json",
        name = slugify_string(request.url().path(), true)?
    ));
    let serializable_request = SerializableRequest::from(request);
    let string = serde_json::to_string_pretty(&serializable_request)?;
    let bytes = string.bytes().collect::<Vec<u8>>();
    path.write(&bytes)?;
    Ok((path, bytes.to_vec()))
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct SerializableRequest {
    pub body: String,
    pub headers: BTreeMap<String, String>,
    pub method: String,
    pub url: String,
}

impl From<&Request> for SerializableRequest {
    fn from(req: &Request) -> SerializableRequest {
        let body = req
            .body()
            .map(|b| {
                String::from_utf8(b.as_bytes().map(|b| b.to_vec()).unwrap_or_default()).unwrap()
            })
            .unwrap_or_default();
        let mut headers = BTreeMap::new();
        for (key, value) in req.headers().iter() {
            headers.insert(
                key.to_string(),
                value.to_str().map(|s| s.to_string()).unwrap_or_default(),
            );
        }
        let method = req.method().to_string();
        let url = req.url().to_string();

        SerializableRequest {
            body,
            headers,
            method,
            url,
        }
    }
}
