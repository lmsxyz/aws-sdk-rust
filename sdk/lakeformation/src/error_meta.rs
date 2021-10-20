// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// All possible error types for this service.
#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub enum Error {
    /// <p>Access to a resource was denied.</p>
    AccessDeniedException(crate::error::AccessDeniedException),
    /// <p>A resource to be created or added already exists.</p>
    AlreadyExistsException(crate::error::AlreadyExistsException),
    /// <p>Two processes are trying to modify a resource simultaneously.</p>
    ConcurrentModificationException(crate::error::ConcurrentModificationException),
    /// <p>A specified entity does not exist</p>
    EntityNotFoundException(crate::error::EntityNotFoundException),
    /// <p>An encryption operation failed.</p>
    GlueEncryptionException(crate::error::GlueEncryptionException),
    /// <p>An internal service error occurred.</p>
    InternalServiceException(crate::error::InternalServiceException),
    /// <p>The input provided was not valid.</p>
    InvalidInputException(crate::error::InvalidInputException),
    /// <p>The operation timed out.</p>
    OperationTimeoutException(crate::error::OperationTimeoutException),
    /// <p>A resource numerical limit was exceeded.</p>
    ResourceNumberLimitExceededException(crate::error::ResourceNumberLimitExceededException),
    /// An unhandled error occurred.
    Unhandled(Box<dyn std::error::Error + Send + Sync + 'static>),
}
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::AccessDeniedException(inner) => inner.fmt(f),
            Error::AlreadyExistsException(inner) => inner.fmt(f),
            Error::ConcurrentModificationException(inner) => inner.fmt(f),
            Error::EntityNotFoundException(inner) => inner.fmt(f),
            Error::GlueEncryptionException(inner) => inner.fmt(f),
            Error::InternalServiceException(inner) => inner.fmt(f),
            Error::InvalidInputException(inner) => inner.fmt(f),
            Error::OperationTimeoutException(inner) => inner.fmt(f),
            Error::ResourceNumberLimitExceededException(inner) => inner.fmt(f),
            Error::Unhandled(inner) => inner.fmt(f),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::AddLFTagsToResourceError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::AddLFTagsToResourceError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::AddLFTagsToResourceErrorKind::AccessDeniedException(inner) => {
                    Error::AccessDeniedException(inner)
                }
                crate::error::AddLFTagsToResourceErrorKind::ConcurrentModificationException(
                    inner,
                ) => Error::ConcurrentModificationException(inner),
                crate::error::AddLFTagsToResourceErrorKind::EntityNotFoundException(inner) => {
                    Error::EntityNotFoundException(inner)
                }
                crate::error::AddLFTagsToResourceErrorKind::InternalServiceException(inner) => {
                    Error::InternalServiceException(inner)
                }
                crate::error::AddLFTagsToResourceErrorKind::InvalidInputException(inner) => {
                    Error::InvalidInputException(inner)
                }
                crate::error::AddLFTagsToResourceErrorKind::OperationTimeoutException(inner) => {
                    Error::OperationTimeoutException(inner)
                }
                crate::error::AddLFTagsToResourceErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::BatchGrantPermissionsError, R>>
    for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::BatchGrantPermissionsError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::BatchGrantPermissionsErrorKind::InvalidInputException(inner) => {
                    Error::InvalidInputException(inner)
                }
                crate::error::BatchGrantPermissionsErrorKind::OperationTimeoutException(inner) => {
                    Error::OperationTimeoutException(inner)
                }
                crate::error::BatchGrantPermissionsErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::BatchRevokePermissionsError, R>>
    for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::BatchRevokePermissionsError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::BatchRevokePermissionsErrorKind::InvalidInputException(inner) => {
                    Error::InvalidInputException(inner)
                }
                crate::error::BatchRevokePermissionsErrorKind::OperationTimeoutException(inner) => {
                    Error::OperationTimeoutException(inner)
                }
                crate::error::BatchRevokePermissionsErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::CreateLFTagError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: aws_smithy_http::result::SdkError<crate::error::CreateLFTagError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::CreateLFTagErrorKind::AccessDeniedException(inner) => {
                    Error::AccessDeniedException(inner)
                }
                crate::error::CreateLFTagErrorKind::EntityNotFoundException(inner) => {
                    Error::EntityNotFoundException(inner)
                }
                crate::error::CreateLFTagErrorKind::InternalServiceException(inner) => {
                    Error::InternalServiceException(inner)
                }
                crate::error::CreateLFTagErrorKind::InvalidInputException(inner) => {
                    Error::InvalidInputException(inner)
                }
                crate::error::CreateLFTagErrorKind::OperationTimeoutException(inner) => {
                    Error::OperationTimeoutException(inner)
                }
                crate::error::CreateLFTagErrorKind::ResourceNumberLimitExceededException(inner) => {
                    Error::ResourceNumberLimitExceededException(inner)
                }
                crate::error::CreateLFTagErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::DeleteLFTagError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: aws_smithy_http::result::SdkError<crate::error::DeleteLFTagError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::DeleteLFTagErrorKind::AccessDeniedException(inner) => {
                    Error::AccessDeniedException(inner)
                }
                crate::error::DeleteLFTagErrorKind::EntityNotFoundException(inner) => {
                    Error::EntityNotFoundException(inner)
                }
                crate::error::DeleteLFTagErrorKind::InternalServiceException(inner) => {
                    Error::InternalServiceException(inner)
                }
                crate::error::DeleteLFTagErrorKind::InvalidInputException(inner) => {
                    Error::InvalidInputException(inner)
                }
                crate::error::DeleteLFTagErrorKind::OperationTimeoutException(inner) => {
                    Error::OperationTimeoutException(inner)
                }
                crate::error::DeleteLFTagErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::DeregisterResourceError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::DeregisterResourceError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::DeregisterResourceErrorKind::EntityNotFoundException(inner) => {
                    Error::EntityNotFoundException(inner)
                }
                crate::error::DeregisterResourceErrorKind::InternalServiceException(inner) => {
                    Error::InternalServiceException(inner)
                }
                crate::error::DeregisterResourceErrorKind::InvalidInputException(inner) => {
                    Error::InvalidInputException(inner)
                }
                crate::error::DeregisterResourceErrorKind::OperationTimeoutException(inner) => {
                    Error::OperationTimeoutException(inner)
                }
                crate::error::DeregisterResourceErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::DescribeResourceError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::DescribeResourceError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::DescribeResourceErrorKind::EntityNotFoundException(inner) => {
                    Error::EntityNotFoundException(inner)
                }
                crate::error::DescribeResourceErrorKind::InternalServiceException(inner) => {
                    Error::InternalServiceException(inner)
                }
                crate::error::DescribeResourceErrorKind::InvalidInputException(inner) => {
                    Error::InvalidInputException(inner)
                }
                crate::error::DescribeResourceErrorKind::OperationTimeoutException(inner) => {
                    Error::OperationTimeoutException(inner)
                }
                crate::error::DescribeResourceErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::GetDataLakeSettingsError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::GetDataLakeSettingsError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::GetDataLakeSettingsErrorKind::EntityNotFoundException(inner) => {
                    Error::EntityNotFoundException(inner)
                }
                crate::error::GetDataLakeSettingsErrorKind::InternalServiceException(inner) => {
                    Error::InternalServiceException(inner)
                }
                crate::error::GetDataLakeSettingsErrorKind::InvalidInputException(inner) => {
                    Error::InvalidInputException(inner)
                }
                crate::error::GetDataLakeSettingsErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R>
    From<aws_smithy_http::result::SdkError<crate::error::GetEffectivePermissionsForPathError, R>>
    for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<
            crate::error::GetEffectivePermissionsForPathError,
            R,
        >,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, ..} => match err.kind {
                crate::error::GetEffectivePermissionsForPathErrorKind::EntityNotFoundException(inner) => Error::EntityNotFoundException(inner),
                crate::error::GetEffectivePermissionsForPathErrorKind::InternalServiceException(inner) => Error::InternalServiceException(inner),
                crate::error::GetEffectivePermissionsForPathErrorKind::InvalidInputException(inner) => Error::InvalidInputException(inner),
                crate::error::GetEffectivePermissionsForPathErrorKind::OperationTimeoutException(inner) => Error::OperationTimeoutException(inner),
                crate::error::GetEffectivePermissionsForPathErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            }
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::GetLFTagError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: aws_smithy_http::result::SdkError<crate::error::GetLFTagError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::GetLFTagErrorKind::AccessDeniedException(inner) => {
                    Error::AccessDeniedException(inner)
                }
                crate::error::GetLFTagErrorKind::EntityNotFoundException(inner) => {
                    Error::EntityNotFoundException(inner)
                }
                crate::error::GetLFTagErrorKind::InternalServiceException(inner) => {
                    Error::InternalServiceException(inner)
                }
                crate::error::GetLFTagErrorKind::InvalidInputException(inner) => {
                    Error::InvalidInputException(inner)
                }
                crate::error::GetLFTagErrorKind::OperationTimeoutException(inner) => {
                    Error::OperationTimeoutException(inner)
                }
                crate::error::GetLFTagErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::GetResourceLFTagsError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::GetResourceLFTagsError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::GetResourceLFTagsErrorKind::AccessDeniedException(inner) => {
                    Error::AccessDeniedException(inner)
                }
                crate::error::GetResourceLFTagsErrorKind::EntityNotFoundException(inner) => {
                    Error::EntityNotFoundException(inner)
                }
                crate::error::GetResourceLFTagsErrorKind::GlueEncryptionException(inner) => {
                    Error::GlueEncryptionException(inner)
                }
                crate::error::GetResourceLFTagsErrorKind::InternalServiceException(inner) => {
                    Error::InternalServiceException(inner)
                }
                crate::error::GetResourceLFTagsErrorKind::InvalidInputException(inner) => {
                    Error::InvalidInputException(inner)
                }
                crate::error::GetResourceLFTagsErrorKind::OperationTimeoutException(inner) => {
                    Error::OperationTimeoutException(inner)
                }
                crate::error::GetResourceLFTagsErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::GrantPermissionsError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::GrantPermissionsError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::GrantPermissionsErrorKind::ConcurrentModificationException(inner) => {
                    Error::ConcurrentModificationException(inner)
                }
                crate::error::GrantPermissionsErrorKind::EntityNotFoundException(inner) => {
                    Error::EntityNotFoundException(inner)
                }
                crate::error::GrantPermissionsErrorKind::InvalidInputException(inner) => {
                    Error::InvalidInputException(inner)
                }
                crate::error::GrantPermissionsErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::ListLFTagsError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: aws_smithy_http::result::SdkError<crate::error::ListLFTagsError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::ListLFTagsErrorKind::EntityNotFoundException(inner) => {
                    Error::EntityNotFoundException(inner)
                }
                crate::error::ListLFTagsErrorKind::InternalServiceException(inner) => {
                    Error::InternalServiceException(inner)
                }
                crate::error::ListLFTagsErrorKind::InvalidInputException(inner) => {
                    Error::InvalidInputException(inner)
                }
                crate::error::ListLFTagsErrorKind::OperationTimeoutException(inner) => {
                    Error::OperationTimeoutException(inner)
                }
                crate::error::ListLFTagsErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::ListPermissionsError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: aws_smithy_http::result::SdkError<crate::error::ListPermissionsError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::ListPermissionsErrorKind::InternalServiceException(inner) => {
                    Error::InternalServiceException(inner)
                }
                crate::error::ListPermissionsErrorKind::InvalidInputException(inner) => {
                    Error::InvalidInputException(inner)
                }
                crate::error::ListPermissionsErrorKind::OperationTimeoutException(inner) => {
                    Error::OperationTimeoutException(inner)
                }
                crate::error::ListPermissionsErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::ListResourcesError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: aws_smithy_http::result::SdkError<crate::error::ListResourcesError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::ListResourcesErrorKind::InternalServiceException(inner) => {
                    Error::InternalServiceException(inner)
                }
                crate::error::ListResourcesErrorKind::InvalidInputException(inner) => {
                    Error::InvalidInputException(inner)
                }
                crate::error::ListResourcesErrorKind::OperationTimeoutException(inner) => {
                    Error::OperationTimeoutException(inner)
                }
                crate::error::ListResourcesErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::PutDataLakeSettingsError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::PutDataLakeSettingsError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::PutDataLakeSettingsErrorKind::InternalServiceException(inner) => {
                    Error::InternalServiceException(inner)
                }
                crate::error::PutDataLakeSettingsErrorKind::InvalidInputException(inner) => {
                    Error::InvalidInputException(inner)
                }
                crate::error::PutDataLakeSettingsErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::RegisterResourceError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::RegisterResourceError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::RegisterResourceErrorKind::AlreadyExistsException(inner) => {
                    Error::AlreadyExistsException(inner)
                }
                crate::error::RegisterResourceErrorKind::InternalServiceException(inner) => {
                    Error::InternalServiceException(inner)
                }
                crate::error::RegisterResourceErrorKind::InvalidInputException(inner) => {
                    Error::InvalidInputException(inner)
                }
                crate::error::RegisterResourceErrorKind::OperationTimeoutException(inner) => {
                    Error::OperationTimeoutException(inner)
                }
                crate::error::RegisterResourceErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::RemoveLFTagsFromResourceError, R>>
    for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::RemoveLFTagsFromResourceError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, ..} => match err.kind {
                crate::error::RemoveLFTagsFromResourceErrorKind::AccessDeniedException(inner) => Error::AccessDeniedException(inner),
                crate::error::RemoveLFTagsFromResourceErrorKind::ConcurrentModificationException(inner) => Error::ConcurrentModificationException(inner),
                crate::error::RemoveLFTagsFromResourceErrorKind::EntityNotFoundException(inner) => Error::EntityNotFoundException(inner),
                crate::error::RemoveLFTagsFromResourceErrorKind::GlueEncryptionException(inner) => Error::GlueEncryptionException(inner),
                crate::error::RemoveLFTagsFromResourceErrorKind::InternalServiceException(inner) => Error::InternalServiceException(inner),
                crate::error::RemoveLFTagsFromResourceErrorKind::InvalidInputException(inner) => Error::InvalidInputException(inner),
                crate::error::RemoveLFTagsFromResourceErrorKind::OperationTimeoutException(inner) => Error::OperationTimeoutException(inner),
                crate::error::RemoveLFTagsFromResourceErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            }
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::RevokePermissionsError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::RevokePermissionsError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::RevokePermissionsErrorKind::ConcurrentModificationException(
                    inner,
                ) => Error::ConcurrentModificationException(inner),
                crate::error::RevokePermissionsErrorKind::EntityNotFoundException(inner) => {
                    Error::EntityNotFoundException(inner)
                }
                crate::error::RevokePermissionsErrorKind::InvalidInputException(inner) => {
                    Error::InvalidInputException(inner)
                }
                crate::error::RevokePermissionsErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::SearchDatabasesByLFTagsError, R>>
    for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::SearchDatabasesByLFTagsError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::SearchDatabasesByLFTagsErrorKind::AccessDeniedException(inner) => {
                    Error::AccessDeniedException(inner)
                }
                crate::error::SearchDatabasesByLFTagsErrorKind::EntityNotFoundException(inner) => {
                    Error::EntityNotFoundException(inner)
                }
                crate::error::SearchDatabasesByLFTagsErrorKind::GlueEncryptionException(inner) => {
                    Error::GlueEncryptionException(inner)
                }
                crate::error::SearchDatabasesByLFTagsErrorKind::InternalServiceException(inner) => {
                    Error::InternalServiceException(inner)
                }
                crate::error::SearchDatabasesByLFTagsErrorKind::InvalidInputException(inner) => {
                    Error::InvalidInputException(inner)
                }
                crate::error::SearchDatabasesByLFTagsErrorKind::OperationTimeoutException(
                    inner,
                ) => Error::OperationTimeoutException(inner),
                crate::error::SearchDatabasesByLFTagsErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::SearchTablesByLFTagsError, R>>
    for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::SearchTablesByLFTagsError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::SearchTablesByLFTagsErrorKind::AccessDeniedException(inner) => {
                    Error::AccessDeniedException(inner)
                }
                crate::error::SearchTablesByLFTagsErrorKind::EntityNotFoundException(inner) => {
                    Error::EntityNotFoundException(inner)
                }
                crate::error::SearchTablesByLFTagsErrorKind::GlueEncryptionException(inner) => {
                    Error::GlueEncryptionException(inner)
                }
                crate::error::SearchTablesByLFTagsErrorKind::InternalServiceException(inner) => {
                    Error::InternalServiceException(inner)
                }
                crate::error::SearchTablesByLFTagsErrorKind::InvalidInputException(inner) => {
                    Error::InvalidInputException(inner)
                }
                crate::error::SearchTablesByLFTagsErrorKind::OperationTimeoutException(inner) => {
                    Error::OperationTimeoutException(inner)
                }
                crate::error::SearchTablesByLFTagsErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::UpdateLFTagError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: aws_smithy_http::result::SdkError<crate::error::UpdateLFTagError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::UpdateLFTagErrorKind::AccessDeniedException(inner) => {
                    Error::AccessDeniedException(inner)
                }
                crate::error::UpdateLFTagErrorKind::ConcurrentModificationException(inner) => {
                    Error::ConcurrentModificationException(inner)
                }
                crate::error::UpdateLFTagErrorKind::EntityNotFoundException(inner) => {
                    Error::EntityNotFoundException(inner)
                }
                crate::error::UpdateLFTagErrorKind::InternalServiceException(inner) => {
                    Error::InternalServiceException(inner)
                }
                crate::error::UpdateLFTagErrorKind::InvalidInputException(inner) => {
                    Error::InvalidInputException(inner)
                }
                crate::error::UpdateLFTagErrorKind::OperationTimeoutException(inner) => {
                    Error::OperationTimeoutException(inner)
                }
                crate::error::UpdateLFTagErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::UpdateResourceError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: aws_smithy_http::result::SdkError<crate::error::UpdateResourceError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::UpdateResourceErrorKind::EntityNotFoundException(inner) => {
                    Error::EntityNotFoundException(inner)
                }
                crate::error::UpdateResourceErrorKind::InternalServiceException(inner) => {
                    Error::InternalServiceException(inner)
                }
                crate::error::UpdateResourceErrorKind::InvalidInputException(inner) => {
                    Error::InvalidInputException(inner)
                }
                crate::error::UpdateResourceErrorKind::OperationTimeoutException(inner) => {
                    Error::OperationTimeoutException(inner)
                }
                crate::error::UpdateResourceErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl std::error::Error for Error {}
