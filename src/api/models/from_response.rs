use crate::Result;
use reqwest::blocking::Response;

pub trait FromResponse {
    fn from_response(response: Response) -> Result<Self>
    where
        Self: Sized;
}
