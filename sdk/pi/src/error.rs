// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// Error type for the `DescribeDimensionKeys` operation.
#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub struct DescribeDimensionKeysError {
    /// Kind of error that occurred.
    pub kind: DescribeDimensionKeysErrorKind,
    /// Additional metadata about the error, including error code, message, and request ID.
    pub(crate) meta: aws_smithy_types::Error,
}
/// Types of errors that can occur for the `DescribeDimensionKeys` operation.
#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub enum DescribeDimensionKeysErrorKind {
    /// <p>The request failed due to an unknown error.</p>
    InternalServiceError(crate::error::InternalServiceError),
    /// <p>One of the arguments provided is invalid for this request.</p>
    InvalidArgumentException(crate::error::InvalidArgumentException),
    /// <p>The user is not authorized to perform this request.</p>
    NotAuthorizedException(crate::error::NotAuthorizedException),
    /// An unexpected error, eg. invalid JSON returned by the service or an unknown error code
    Unhandled(Box<dyn std::error::Error + Send + Sync + 'static>),
}
impl std::fmt::Display for DescribeDimensionKeysError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.kind {
            DescribeDimensionKeysErrorKind::InternalServiceError(_inner) => _inner.fmt(f),
            DescribeDimensionKeysErrorKind::InvalidArgumentException(_inner) => _inner.fmt(f),
            DescribeDimensionKeysErrorKind::NotAuthorizedException(_inner) => _inner.fmt(f),
            DescribeDimensionKeysErrorKind::Unhandled(_inner) => _inner.fmt(f),
        }
    }
}
impl aws_smithy_types::retry::ProvideErrorKind for DescribeDimensionKeysError {
    fn code(&self) -> Option<&str> {
        DescribeDimensionKeysError::code(self)
    }
    fn retryable_error_kind(&self) -> Option<aws_smithy_types::retry::ErrorKind> {
        None
    }
}
impl DescribeDimensionKeysError {
    /// Creates a new `DescribeDimensionKeysError`.
    pub fn new(kind: DescribeDimensionKeysErrorKind, meta: aws_smithy_types::Error) -> Self {
        Self { kind, meta }
    }

    /// Creates the `DescribeDimensionKeysError::Unhandled` variant from any error type.
    pub fn unhandled(err: impl Into<Box<dyn std::error::Error + Send + Sync + 'static>>) -> Self {
        Self {
            kind: DescribeDimensionKeysErrorKind::Unhandled(err.into()),
            meta: Default::default(),
        }
    }

    /// Creates the `DescribeDimensionKeysError::Unhandled` variant from a `aws_smithy_types::Error`.
    pub fn generic(err: aws_smithy_types::Error) -> Self {
        Self {
            meta: err.clone(),
            kind: DescribeDimensionKeysErrorKind::Unhandled(err.into()),
        }
    }

    // TODO: Consider if this should actually be `Option<Cow<&str>>`. This would enable us to use display
    // as implemented by std::Error to generate a message in that case.
    /// Returns the error message if one is available.
    pub fn message(&self) -> Option<&str> {
        self.meta.message()
    }

    /// Returns error metadata, which includes the error code, message,
    /// request ID, and potentially additional information.
    pub fn meta(&self) -> &aws_smithy_types::Error {
        &self.meta
    }

    /// Returns the request ID if it's available.
    pub fn request_id(&self) -> Option<&str> {
        self.meta.request_id()
    }

    /// Returns the error code if it's available.
    pub fn code(&self) -> Option<&str> {
        self.meta.code()
    }
    /// Returns true if the error kind is `DescribeDimensionKeysErrorKind::InternalServiceError`.
    pub fn is_internal_service_error(&self) -> bool {
        matches!(
            &self.kind,
            DescribeDimensionKeysErrorKind::InternalServiceError(_)
        )
    }
    /// Returns true if the error kind is `DescribeDimensionKeysErrorKind::InvalidArgumentException`.
    pub fn is_invalid_argument_exception(&self) -> bool {
        matches!(
            &self.kind,
            DescribeDimensionKeysErrorKind::InvalidArgumentException(_)
        )
    }
    /// Returns true if the error kind is `DescribeDimensionKeysErrorKind::NotAuthorizedException`.
    pub fn is_not_authorized_exception(&self) -> bool {
        matches!(
            &self.kind,
            DescribeDimensionKeysErrorKind::NotAuthorizedException(_)
        )
    }
}
impl std::error::Error for DescribeDimensionKeysError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match &self.kind {
            DescribeDimensionKeysErrorKind::InternalServiceError(_inner) => Some(_inner),
            DescribeDimensionKeysErrorKind::InvalidArgumentException(_inner) => Some(_inner),
            DescribeDimensionKeysErrorKind::NotAuthorizedException(_inner) => Some(_inner),
            DescribeDimensionKeysErrorKind::Unhandled(_inner) => Some(_inner.as_ref()),
        }
    }
}

/// Error type for the `GetDimensionKeyDetails` operation.
#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub struct GetDimensionKeyDetailsError {
    /// Kind of error that occurred.
    pub kind: GetDimensionKeyDetailsErrorKind,
    /// Additional metadata about the error, including error code, message, and request ID.
    pub(crate) meta: aws_smithy_types::Error,
}
/// Types of errors that can occur for the `GetDimensionKeyDetails` operation.
#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub enum GetDimensionKeyDetailsErrorKind {
    /// <p>The request failed due to an unknown error.</p>
    InternalServiceError(crate::error::InternalServiceError),
    /// <p>One of the arguments provided is invalid for this request.</p>
    InvalidArgumentException(crate::error::InvalidArgumentException),
    /// <p>The user is not authorized to perform this request.</p>
    NotAuthorizedException(crate::error::NotAuthorizedException),
    /// An unexpected error, eg. invalid JSON returned by the service or an unknown error code
    Unhandled(Box<dyn std::error::Error + Send + Sync + 'static>),
}
impl std::fmt::Display for GetDimensionKeyDetailsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.kind {
            GetDimensionKeyDetailsErrorKind::InternalServiceError(_inner) => _inner.fmt(f),
            GetDimensionKeyDetailsErrorKind::InvalidArgumentException(_inner) => _inner.fmt(f),
            GetDimensionKeyDetailsErrorKind::NotAuthorizedException(_inner) => _inner.fmt(f),
            GetDimensionKeyDetailsErrorKind::Unhandled(_inner) => _inner.fmt(f),
        }
    }
}
impl aws_smithy_types::retry::ProvideErrorKind for GetDimensionKeyDetailsError {
    fn code(&self) -> Option<&str> {
        GetDimensionKeyDetailsError::code(self)
    }
    fn retryable_error_kind(&self) -> Option<aws_smithy_types::retry::ErrorKind> {
        None
    }
}
impl GetDimensionKeyDetailsError {
    /// Creates a new `GetDimensionKeyDetailsError`.
    pub fn new(kind: GetDimensionKeyDetailsErrorKind, meta: aws_smithy_types::Error) -> Self {
        Self { kind, meta }
    }

    /// Creates the `GetDimensionKeyDetailsError::Unhandled` variant from any error type.
    pub fn unhandled(err: impl Into<Box<dyn std::error::Error + Send + Sync + 'static>>) -> Self {
        Self {
            kind: GetDimensionKeyDetailsErrorKind::Unhandled(err.into()),
            meta: Default::default(),
        }
    }

    /// Creates the `GetDimensionKeyDetailsError::Unhandled` variant from a `aws_smithy_types::Error`.
    pub fn generic(err: aws_smithy_types::Error) -> Self {
        Self {
            meta: err.clone(),
            kind: GetDimensionKeyDetailsErrorKind::Unhandled(err.into()),
        }
    }

    // TODO: Consider if this should actually be `Option<Cow<&str>>`. This would enable us to use display
    // as implemented by std::Error to generate a message in that case.
    /// Returns the error message if one is available.
    pub fn message(&self) -> Option<&str> {
        self.meta.message()
    }

    /// Returns error metadata, which includes the error code, message,
    /// request ID, and potentially additional information.
    pub fn meta(&self) -> &aws_smithy_types::Error {
        &self.meta
    }

    /// Returns the request ID if it's available.
    pub fn request_id(&self) -> Option<&str> {
        self.meta.request_id()
    }

    /// Returns the error code if it's available.
    pub fn code(&self) -> Option<&str> {
        self.meta.code()
    }
    /// Returns true if the error kind is `GetDimensionKeyDetailsErrorKind::InternalServiceError`.
    pub fn is_internal_service_error(&self) -> bool {
        matches!(
            &self.kind,
            GetDimensionKeyDetailsErrorKind::InternalServiceError(_)
        )
    }
    /// Returns true if the error kind is `GetDimensionKeyDetailsErrorKind::InvalidArgumentException`.
    pub fn is_invalid_argument_exception(&self) -> bool {
        matches!(
            &self.kind,
            GetDimensionKeyDetailsErrorKind::InvalidArgumentException(_)
        )
    }
    /// Returns true if the error kind is `GetDimensionKeyDetailsErrorKind::NotAuthorizedException`.
    pub fn is_not_authorized_exception(&self) -> bool {
        matches!(
            &self.kind,
            GetDimensionKeyDetailsErrorKind::NotAuthorizedException(_)
        )
    }
}
impl std::error::Error for GetDimensionKeyDetailsError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match &self.kind {
            GetDimensionKeyDetailsErrorKind::InternalServiceError(_inner) => Some(_inner),
            GetDimensionKeyDetailsErrorKind::InvalidArgumentException(_inner) => Some(_inner),
            GetDimensionKeyDetailsErrorKind::NotAuthorizedException(_inner) => Some(_inner),
            GetDimensionKeyDetailsErrorKind::Unhandled(_inner) => Some(_inner.as_ref()),
        }
    }
}

/// Error type for the `GetResourceMetrics` operation.
#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub struct GetResourceMetricsError {
    /// Kind of error that occurred.
    pub kind: GetResourceMetricsErrorKind,
    /// Additional metadata about the error, including error code, message, and request ID.
    pub(crate) meta: aws_smithy_types::Error,
}
/// Types of errors that can occur for the `GetResourceMetrics` operation.
#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub enum GetResourceMetricsErrorKind {
    /// <p>The request failed due to an unknown error.</p>
    InternalServiceError(crate::error::InternalServiceError),
    /// <p>One of the arguments provided is invalid for this request.</p>
    InvalidArgumentException(crate::error::InvalidArgumentException),
    /// <p>The user is not authorized to perform this request.</p>
    NotAuthorizedException(crate::error::NotAuthorizedException),
    /// An unexpected error, eg. invalid JSON returned by the service or an unknown error code
    Unhandled(Box<dyn std::error::Error + Send + Sync + 'static>),
}
impl std::fmt::Display for GetResourceMetricsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.kind {
            GetResourceMetricsErrorKind::InternalServiceError(_inner) => _inner.fmt(f),
            GetResourceMetricsErrorKind::InvalidArgumentException(_inner) => _inner.fmt(f),
            GetResourceMetricsErrorKind::NotAuthorizedException(_inner) => _inner.fmt(f),
            GetResourceMetricsErrorKind::Unhandled(_inner) => _inner.fmt(f),
        }
    }
}
impl aws_smithy_types::retry::ProvideErrorKind for GetResourceMetricsError {
    fn code(&self) -> Option<&str> {
        GetResourceMetricsError::code(self)
    }
    fn retryable_error_kind(&self) -> Option<aws_smithy_types::retry::ErrorKind> {
        None
    }
}
impl GetResourceMetricsError {
    /// Creates a new `GetResourceMetricsError`.
    pub fn new(kind: GetResourceMetricsErrorKind, meta: aws_smithy_types::Error) -> Self {
        Self { kind, meta }
    }

    /// Creates the `GetResourceMetricsError::Unhandled` variant from any error type.
    pub fn unhandled(err: impl Into<Box<dyn std::error::Error + Send + Sync + 'static>>) -> Self {
        Self {
            kind: GetResourceMetricsErrorKind::Unhandled(err.into()),
            meta: Default::default(),
        }
    }

    /// Creates the `GetResourceMetricsError::Unhandled` variant from a `aws_smithy_types::Error`.
    pub fn generic(err: aws_smithy_types::Error) -> Self {
        Self {
            meta: err.clone(),
            kind: GetResourceMetricsErrorKind::Unhandled(err.into()),
        }
    }

    // TODO: Consider if this should actually be `Option<Cow<&str>>`. This would enable us to use display
    // as implemented by std::Error to generate a message in that case.
    /// Returns the error message if one is available.
    pub fn message(&self) -> Option<&str> {
        self.meta.message()
    }

    /// Returns error metadata, which includes the error code, message,
    /// request ID, and potentially additional information.
    pub fn meta(&self) -> &aws_smithy_types::Error {
        &self.meta
    }

    /// Returns the request ID if it's available.
    pub fn request_id(&self) -> Option<&str> {
        self.meta.request_id()
    }

    /// Returns the error code if it's available.
    pub fn code(&self) -> Option<&str> {
        self.meta.code()
    }
    /// Returns true if the error kind is `GetResourceMetricsErrorKind::InternalServiceError`.
    pub fn is_internal_service_error(&self) -> bool {
        matches!(
            &self.kind,
            GetResourceMetricsErrorKind::InternalServiceError(_)
        )
    }
    /// Returns true if the error kind is `GetResourceMetricsErrorKind::InvalidArgumentException`.
    pub fn is_invalid_argument_exception(&self) -> bool {
        matches!(
            &self.kind,
            GetResourceMetricsErrorKind::InvalidArgumentException(_)
        )
    }
    /// Returns true if the error kind is `GetResourceMetricsErrorKind::NotAuthorizedException`.
    pub fn is_not_authorized_exception(&self) -> bool {
        matches!(
            &self.kind,
            GetResourceMetricsErrorKind::NotAuthorizedException(_)
        )
    }
}
impl std::error::Error for GetResourceMetricsError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match &self.kind {
            GetResourceMetricsErrorKind::InternalServiceError(_inner) => Some(_inner),
            GetResourceMetricsErrorKind::InvalidArgumentException(_inner) => Some(_inner),
            GetResourceMetricsErrorKind::NotAuthorizedException(_inner) => Some(_inner),
            GetResourceMetricsErrorKind::Unhandled(_inner) => Some(_inner.as_ref()),
        }
    }
}

/// <p>The user is not authorized to perform this request.</p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct NotAuthorizedException {
    #[allow(missing_docs)] // documentation missing in model
    pub message: std::option::Option<std::string::String>,
}
impl std::fmt::Debug for NotAuthorizedException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("NotAuthorizedException");
        formatter.field("message", &self.message);
        formatter.finish()
    }
}
impl NotAuthorizedException {
    /// Returns the error message.
    pub fn message(&self) -> Option<&str> {
        self.message.as_deref()
    }
}
impl std::fmt::Display for NotAuthorizedException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "NotAuthorizedException")?;
        if let Some(inner_1) = &self.message {
            write!(f, ": {}", inner_1)?;
        }
        Ok(())
    }
}
impl std::error::Error for NotAuthorizedException {}
/// See [`NotAuthorizedException`](crate::error::NotAuthorizedException)
pub mod not_authorized_exception {
    /// A builder for [`NotAuthorizedException`](crate::error::NotAuthorizedException)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) message: std::option::Option<std::string::String>,
    }
    impl Builder {
        #[allow(missing_docs)] // documentation missing in model
        pub fn message(mut self, input: impl Into<std::string::String>) -> Self {
            self.message = Some(input.into());
            self
        }
        #[allow(missing_docs)] // documentation missing in model
        pub fn set_message(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.message = input;
            self
        }
        /// Consumes the builder and constructs a [`NotAuthorizedException`](crate::error::NotAuthorizedException)
        pub fn build(self) -> crate::error::NotAuthorizedException {
            crate::error::NotAuthorizedException {
                message: self.message,
            }
        }
    }
}
impl NotAuthorizedException {
    /// Creates a new builder-style object to manufacture [`NotAuthorizedException`](crate::error::NotAuthorizedException)
    pub fn builder() -> crate::error::not_authorized_exception::Builder {
        crate::error::not_authorized_exception::Builder::default()
    }
}

/// <p>One of the arguments provided is invalid for this request.</p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct InvalidArgumentException {
    #[allow(missing_docs)] // documentation missing in model
    pub message: std::option::Option<std::string::String>,
}
impl std::fmt::Debug for InvalidArgumentException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("InvalidArgumentException");
        formatter.field("message", &self.message);
        formatter.finish()
    }
}
impl InvalidArgumentException {
    /// Returns the error message.
    pub fn message(&self) -> Option<&str> {
        self.message.as_deref()
    }
}
impl std::fmt::Display for InvalidArgumentException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "InvalidArgumentException")?;
        if let Some(inner_2) = &self.message {
            write!(f, ": {}", inner_2)?;
        }
        Ok(())
    }
}
impl std::error::Error for InvalidArgumentException {}
/// See [`InvalidArgumentException`](crate::error::InvalidArgumentException)
pub mod invalid_argument_exception {
    /// A builder for [`InvalidArgumentException`](crate::error::InvalidArgumentException)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) message: std::option::Option<std::string::String>,
    }
    impl Builder {
        #[allow(missing_docs)] // documentation missing in model
        pub fn message(mut self, input: impl Into<std::string::String>) -> Self {
            self.message = Some(input.into());
            self
        }
        #[allow(missing_docs)] // documentation missing in model
        pub fn set_message(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.message = input;
            self
        }
        /// Consumes the builder and constructs a [`InvalidArgumentException`](crate::error::InvalidArgumentException)
        pub fn build(self) -> crate::error::InvalidArgumentException {
            crate::error::InvalidArgumentException {
                message: self.message,
            }
        }
    }
}
impl InvalidArgumentException {
    /// Creates a new builder-style object to manufacture [`InvalidArgumentException`](crate::error::InvalidArgumentException)
    pub fn builder() -> crate::error::invalid_argument_exception::Builder {
        crate::error::invalid_argument_exception::Builder::default()
    }
}

/// <p>The request failed due to an unknown error.</p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct InternalServiceError {
    #[allow(missing_docs)] // documentation missing in model
    pub message: std::option::Option<std::string::String>,
}
impl std::fmt::Debug for InternalServiceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("InternalServiceError");
        formatter.field("message", &self.message);
        formatter.finish()
    }
}
impl InternalServiceError {
    /// Returns the error message.
    pub fn message(&self) -> Option<&str> {
        self.message.as_deref()
    }
}
impl std::fmt::Display for InternalServiceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "InternalServiceError")?;
        if let Some(inner_3) = &self.message {
            write!(f, ": {}", inner_3)?;
        }
        Ok(())
    }
}
impl std::error::Error for InternalServiceError {}
/// See [`InternalServiceError`](crate::error::InternalServiceError)
pub mod internal_service_error {
    /// A builder for [`InternalServiceError`](crate::error::InternalServiceError)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) message: std::option::Option<std::string::String>,
    }
    impl Builder {
        #[allow(missing_docs)] // documentation missing in model
        pub fn message(mut self, input: impl Into<std::string::String>) -> Self {
            self.message = Some(input.into());
            self
        }
        #[allow(missing_docs)] // documentation missing in model
        pub fn set_message(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.message = input;
            self
        }
        /// Consumes the builder and constructs a [`InternalServiceError`](crate::error::InternalServiceError)
        pub fn build(self) -> crate::error::InternalServiceError {
            crate::error::InternalServiceError {
                message: self.message,
            }
        }
    }
}
impl InternalServiceError {
    /// Creates a new builder-style object to manufacture [`InternalServiceError`](crate::error::InternalServiceError)
    pub fn builder() -> crate::error::internal_service_error::Builder {
        crate::error::internal_service_error::Builder::default()
    }
}
