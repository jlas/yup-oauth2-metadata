use ::metadata::MetadataServerFlow;
use yup_oauth2::GetToken;

#[tokio::main]
async fn main() {
    let mut mdsf = MetadataServerFlow {};
    let scopes = &["https://www.googleapis.com/auth/bigquery"];
    match mdsf.token(scopes) {
        Err(e) => println!("error: {:?}", e),
        Ok(t) => println!("The token is {:?}", t),
    }
}
