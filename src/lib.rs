use futures::executor;
use hyper::{Body, Client, Request};
use std::error::Error;
use std::result::Result;
use yup_oauth2::{GetToken, Token};

/// foo
pub struct MetadataServerFlow;

/// bar
impl GetToken for MetadataServerFlow {
    fn api_key(&mut self) -> Option<String> {
        None
    }
    fn token<'b, I, T>(&mut self, _scopes: I) -> Result<Token, Box<dyn Error>>
    where
        T: AsRef<str> + 'b,
        I: IntoIterator<Item = &'b T>,
    {
        log::debug!("DefaultApplicationCredentials: checking metadata server...");
        let error;
        let uri = "http://metadata.google.internal/computeMetadata/v1/instance/service-accounts/default/token";
        let req = Request::builder()
            .uri(uri)
            .header("Metadata-Flavor", "Google")
            .body(Body::empty())
            .unwrap();
        let client = Client::new();
        let response = executor::block_on(client.request(req));
        match response {
            Ok(response) => {
                let (head, body) = response.into_parts();
                //let body = hyper::body::to_bytes(body).await?;
                log::debug!("Received response; head: {:?} body: {:?}", head, body);
                let token = Token {
                    access_token: "".to_string(),
                    refresh_token: "".to_string(),
                    token_type: "".to_string(),
                    expires_in: Some(0),
                    expires_in_timestamp: Some(0),
                };
                //return Ok(TokenInfo::from_json(&body)?);
                //let token = serde_json::from_slice(body)?;
                return Ok(token);
            }
            Err(new_error) => error = new_error,
        }
        Err(error.into())
    }
}
