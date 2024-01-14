use iceberg_rust::error::Error as IcebergError;

use crate::apis::{
    v1_api::{CommitMultipleOperationsError, GetContentError, GetEntriesError, GetNamespacesError},
    Error,
};

impl From<Error<GetContentError>> for IcebergError {
    fn from(value: Error<GetContentError>) -> Self {
        match value {
            Error::ResponseError(e) => {
                IcebergError::InvalidFormat(format!("{}: {}", e.status, e.content))
            }
            value => IcebergError::InvalidFormat(format!("{}", value)),
        }
    }
}

impl From<Error<GetEntriesError>> for IcebergError {
    fn from(value: Error<GetEntriesError>) -> Self {
        match value {
            Error::ResponseError(e) => {
                IcebergError::InvalidFormat(format!("{}: {}", e.status, e.content))
            }
            value => IcebergError::InvalidFormat(format!("{}", value)),
        }
    }
}

impl From<Error<GetNamespacesError>> for IcebergError {
    fn from(value: Error<GetNamespacesError>) -> Self {
        match value {
            Error::ResponseError(e) => {
                IcebergError::InvalidFormat(format!("{}: {}", e.status, e.content))
            }
            value => IcebergError::InvalidFormat(format!("{}", value)),
        }
    }
}

impl From<Error<CommitMultipleOperationsError>> for IcebergError {
    fn from(value: Error<CommitMultipleOperationsError>) -> Self {
        match value {
            Error::ResponseError(e) => {
                IcebergError::InvalidFormat(format!("{}: {}", e.status, e.content))
            }
            value => IcebergError::InvalidFormat(format!("{}", value)),
        }
    }
}
