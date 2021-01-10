pub use compute_metadata::get_project_id;
pub use firestore_grpc::v1::*;
use firestore_client::FirestoreClient;
use firestore_grpc::tonic::{
    metadata::MetadataValue,
    metadata::errors::InvalidMetadataValue,
    transport::{Channel, ClientTlsConfig},
    Request,
};
use futures::{future::TryFutureExt, try_join};
use std::env;
use thiserror::Error;

pub mod compute_metadata;

const URL: &'static str = "https://firestore.googleapis.com";
const DOMAIN: &'static str = "firestore.googleapis.com";

#[derive(Error, Debug)]
pub enum FsClientError {
    #[error("empty error")]
    EmptyError,

    /// Represents all other cases of `std::io::Error`.
    #[error(transparent)]
    OtherError(#[from] std::io::Error),

    #[error(transparent)]
    FsError(#[from] firestore_grpc::tonic::Status),

    #[error(transparent)]
    TpError(#[from] firestore_grpc::tonic::transport::Error),

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
}

// <<<<<<< HEAD
pub async fn get_client() -> Result<FirestoreClient<Channel>, FsClientError> {
    // let mut addr = return match env::var("FIRESTORE_EMULATOR_HOST") {
        match env::var("FIRESTORE_EMULATOR_HOST") {
        Ok(_addr) => {
            // TODO : need to get the value from addr instead of hard-coded values
            let endpoint = Channel::from_static("http://localhost:8200");
            // let add: &'static str = addr.;
            // let endpoint = Channel::from_static(add);
// // =======
// pub async fn get_client() -> Result<FirestoreClient<Channel>, BoxError> {
//     let mut addr = return match env::var("FIRESTORE_EMULATOR_HOST") {
//         Ok(addr) => {
//             let endpoint = Channel::from_shared(addr.clone())?;
// // >>>>>>> 52f029b705a5f1385d038312982ea2837c09a9d4
            let channel = endpoint.connect().await?;
            let service = FirestoreClient::with_interceptor(channel, move |mut req: Request<()>| {
                req.metadata_mut();
                Ok(req)
            });
            Ok(service)
        }
        Err(_e) => {
            let endpoint = Channel::from_static(URL).tls_config(ClientTlsConfig::new().domain_name(DOMAIN));
            let (channel, token) = try_join!(
                endpoint.connect().map_err(|e| FsClientError::from(e)),
                compute_metadata::get_token()
            )?;

            let bearer_token = format!("Bearer {}", token);
            let header_value = MetadataValue::from_str(&bearer_token)?;

            let service = FirestoreClient::with_interceptor(channel, move |mut req: Request<()>| {
                req.metadata_mut()
                    .insert("authorization", header_value.clone());
                Ok(req)
            });
            Ok(service)
        }
    }
}
