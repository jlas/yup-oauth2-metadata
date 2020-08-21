use ::metadata::MetadataServerFlow;
use google_bigquery2::{Bigquery, TableDataInsertAllRequest, TableDataInsertAllRequestRows};
use hyper;
use hyper_rustls;
use yup_oauth2::GetToken;

fn main() {
    let mut mdsf = MetadataServerFlow {};
    let scopes = &["https://www.googleapis.com/auth/bigquery"];
    match mdsf.token(scopes) {
        Err(e) => println!("error: {:?}", e),
        Ok(t) => println!("The token is {:?}", t),
    }

    let hub = Bigquery::new(
        hyper::Client::with_connector(hyper::net::HttpsConnector::new(
            hyper_rustls::TlsClient::new(),
        )),
        mdsf,
    );
    let mut rows = vec![];
    rows.push(TableDataInsertAllRequestRows {
        insert_id: None,
        json: serde_json::from_str("{\"foo\": 1}").unwrap(),
    });
    let mut request_data = TableDataInsertAllRequest::default();
    request_data.rows = Some(rows);
    hub.tabledata()
        .insert_all(request_data, "my-project", "my-dataset", "my-table")
        .doit()
        .unwrap();
}
