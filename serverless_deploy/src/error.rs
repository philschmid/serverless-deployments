use aws_sdk_lambda::{
    error::{CreateFunctionError, DeleteFunctionError, GetFunctionError},
    types::SdkError,
};
use thiserror::Error;

// pub type Result<T> = std::result::Result<T, ServerlessDeployError>;

#[derive(Error, Debug)]
pub enum ServerlessDeployError {
    #[error("Lambda function `{0}` already exists")]
    AlreadyExists(String),
    #[error("Couldn't create function")]
    // Creation(#[from] SdkError<CreateFunctionError>),
    Creation,
    #[error("Couldn't get function")]
    Get(#[from] SdkError<GetFunctionError>),
    #[error("Couldn't delete function")]
    Delete(#[from] SdkError<DeleteFunctionError>),
    // #[error("invalid header (expected {expected:?}, found {found:?})")]
    // InvalidHeader { expected: String, found: String },
    // #[error("unknown data store error")]
    // Unknown,
    #[error(transparent)]
    Other(#[from] anyhow::Error), // source and Display delegate to anyhow::Error
}
