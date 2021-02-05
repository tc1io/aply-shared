pub use compute_metadata::get_project_id;
pub use datastore_grpc::v1::*;
pub use datastore_client::DatastoreClient;
use datastore_grpc::tonic::{
    metadata::MetadataValue,
    metadata::errors::InvalidMetadataValue,
    transport::{Channel, ClientTlsConfig},
    Request,
};
use futures::{future::TryFutureExt, try_join};
use std::env;
use thiserror::Error;

pub mod compute_metadata;

const URL: &'static str = "https://datastore.googleapis.com";
const DOMAIN: &'static str = "datastore.googleapis.com";

#[derive(Error, Debug)]
pub enum DsClientError {
    #[error("empty error")]
    EmptyError,

    /// Represents all other cases of `std::io::Error`.
    #[error(transparent)]
    OtherError(#[from] std::io::Error),

    #[error(transparent)]
    DsError(#[from] datastore_grpc::tonic::Status),

    #[error(transparent)]
    TpError(#[from] datastore_grpc::tonic::transport::Error),

    #[error(transparent)]
    Tp2Error(#[from] InvalidMetadataValue),

    #[error(transparent)]
    SerdeError(#[from] serde_json::Error),

    #[error(transparent)]
    HyperHttpError(#[from] hyper::http::Error),

    #[error(transparent)]
    HyperError(#[from] hyper::Error),

    #[error(transparent)]
    FooError(#[from] std::string::FromUtf8Error),

    #[error(transparent)]
    InvalidUriError(#[from] http::uri::InvalidUri),
}

pub async fn get_client() -> Result<DatastoreClient<Channel>, DsClientError> {
        match env::var("DATASTORE_EMULATOR_HOST") {
        Ok(addr) => {
            let endpoint = Channel::from_shared(addr.clone())?;
            let channel = endpoint.connect().await?;
            let service = DatastoreClient::with_interceptor(channel, move |mut req: Request<()>| {
                req.metadata_mut();
                Ok(req)
            });
            Ok(service)
        }
        Err(_e) => {
            let endpoint = Channel::from_static(URL).tls_config(ClientTlsConfig::new().domain_name(DOMAIN)).expect("TODO - handle the connect error");
            let (channel, token) = try_join!(
                endpoint.connect().map_err(|e| DsClientError::from(e)),
                compute_metadata::get_token()
            )?;

            let bearer_token = format!("Bearer {}", token);
            let header_value = MetadataValue::from_str(&bearer_token)?;

            let service = DatastoreClient::with_interceptor(channel, move |mut req: Request<()>| {
                req.metadata_mut()
                    .insert("authorization", header_value.clone());
                Ok(req)
            });
            Ok(service)
        }
    }
}
