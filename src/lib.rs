use reqwest;
use serde::Deserialize;
use std::error::Error;
use std::io::{Error as IoErr, ErrorKind};
use std::result::Result;
use yup_oauth2::{GetToken, Token};

#[derive(Deserialize)]
pub struct _Token {
    access_token: String,
    expires_in: i64,
    token_type: String,
}

pub struct MetadataServerFlow;

impl GetToken for MetadataServerFlow {
    fn api_key(&mut self) -> Option<String> {
        None
    }
    fn token<'b, I, T>(&mut self, _scopes: I) -> Result<Token, Box<dyn Error>>
    where
        T: AsRef<str> + 'b,
        I: IntoIterator<Item = &'b T>,
    {
        let uri = "http://metadata.google.internal/computeMetadata/v1/instance/service-accounts/default/token";
        let client = reqwest::blocking::Client::new();
        let response = client.get(uri).header("Metadata-Flavor", "Google").send()?;
        match response.status() {
            reqwest::StatusCode::OK => {
                let dt: _Token = response.json()?;
                let token = Token {
                    access_token: dt.access_token,
                    expires_in: Some(dt.expires_in),
                    token_type: dt.token_type,
                    refresh_token: "".to_string(),
                    expires_in_timestamp: Some(0),
                };
                return Ok(token);
            }
            s => log::warn!("Received response status: {:?}", s),
        };
        Err(Box::new(IoErr::new(ErrorKind::Other, "Unexpected error")))
    }
}
