pub use compute_metadata::get_project_id;
pub use pubsub_grpc::v1::*;
pub use publisher_client::PublisherClient;
use pubsub_grpc::tonic::{
    metadata::MetadataValue,
    metadata::errors::InvalidMetadataValue,
    transport::{Channel, ClientTlsConfig},
    Request,
};
use futures::{future::TryFutureExt, try_join};
use std::env;
use thiserror::Error;

pub mod compute_metadata;

const URL: &'static str = "https://pubsub.googleapis.com";
const DOMAIN: &'static str = "pubsub.googleapis.com";

#[derive(Error, Debug)]
pub enum PsClientError {
    #[error("empty error")]
    EmptyError,

    /// Represents all other cases of `std::io::Error`.
    #[error(transparent)]
    OtherError(#[from] std::io::Error),

    #[error(transparent)]
    PsError(#[from] pubsub_grpc::tonic::Status),

    #[error(transparent)]
    TpError(#[from] pubsub_grpc::tonic::transport::Error),

    #[error(transparent)]
    Tp2Error(#[from] InvalidMetadataValue),

    #[error(transparent)]
    SerdeError(#[from] serde_json::Error),

    #[error(transparent)]
    HyperHttpError(#[from] hyper::http::Error),

    #[error(transparent)]
    HyperError(#[from] hyper::Error),

    #[error(transparent)]
    FooError(#[from] std::string::FromUtf8Error ),

    #[error(transparent)]
    InvalidUriError(#[from] http::uri::InvalidUri),
}

// <<<<<<< HEAD
pub async fn get_client() -> Result<PublisherClient<Channel>, PsClientError> {
    // let mut addr = return match env::var("FIRESTORE_EMULATOR_HOST") {
        match env::var("PUBSUB_EMULATOR_HOST") {
        Ok(addr) => {
            // TODO : need to get the value from addr instead of hard-coded values
            //let endpoint = Channel::from_static("http://localhost:8201");
            //let endpoint = Channel::from_static("http://localhost:8201");
            let endpoint = Channel::from_shared(addr.clone())?;
            // let add: &'static str = addr.;
            // let endpoint = Channel::from_static(add);
// // =======
// pub async fn get_client() -> Result<PublisherClient<Channel>, BoxError> {
//     let mut addr = return match env::var("FIRESTORE_EMULATOR_HOST") {
//         Ok(addr) => {
//             let endpoint = Channel::from_shared(addr.clone())?;
// // >>>>>>> 52f029b705a5f1385d038312982ea2837c09a9d4
            let channel = endpoint.connect().await?;
            let service = PublisherClient::with_interceptor(channel, move |mut req: Request<()>| {
                req.metadata_mut();
                Ok(req)
            });
            Ok(service)
        }
        Err(_e) => {
            let endpoint = Channel::from_static(URL).tls_config(ClientTlsConfig::new().domain_name(DOMAIN));
            let (channel, token) = try_join!(
                endpoint.connect().map_err(|e| PsClientError::from(e)),
                compute_metadata::get_token()
            )?;

            let bearer_token = format!("Bearer {}", token);
            let header_value = MetadataValue::from_str(&bearer_token)?;

            let service = PublisherClient::with_interceptor(channel, move |mut req: Request<()>| {
                req.metadata_mut()
                    .insert("authorization", header_value.clone());
                Ok(req)
            });
            Ok(service)
        }
    }
}
