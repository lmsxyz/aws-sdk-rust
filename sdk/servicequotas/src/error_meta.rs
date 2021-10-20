// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// All possible error types for this service.
#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub enum Error {
    /// <p>The action you attempted is not allowed unless Service Access with Service Quotas is
    /// enabled in your organization.</p>
    AwsServiceAccessNotEnabledException(crate::error::AwsServiceAccessNotEnabledException),
    /// <p>You do not have sufficient permission to perform this action.</p>
    AccessDeniedException(crate::error::AccessDeniedException),
    /// <p>You can't perform this action because a dependency does not have access.</p>
    DependencyAccessDeniedException(crate::error::DependencyAccessDeniedException),
    /// <p>Invalid input was provided.</p>
    IllegalArgumentException(crate::error::IllegalArgumentException),
    /// <p>Invalid input was provided.</p>
    InvalidPaginationTokenException(crate::error::InvalidPaginationTokenException),
    /// <p>The resource is in an invalid state.</p>
    InvalidResourceStateException(crate::error::InvalidResourceStateException),
    /// <p>The account making this call is not a member of an organization.</p>
    NoAvailableOrganizationException(crate::error::NoAvailableOrganizationException),
    /// <p>The specified resource does not exist.</p>
    NoSuchResourceException(crate::error::NoSuchResourceException),
    /// <p>The organization that your account belongs to is not in All Features mode.</p>
    OrganizationNotInAllFeaturesModeException(
        crate::error::OrganizationNotInAllFeaturesModeException,
    ),
    /// <p>You have exceeded your service quota. To perform the requested action, remove some of the
    /// relevant resources, or use Service Quotas to request a service quota increase.</p>
    QuotaExceededException(crate::error::QuotaExceededException),
    /// <p>The specified resource already exists.</p>
    ResourceAlreadyExistsException(crate::error::ResourceAlreadyExistsException),
    /// <p>Something went wrong.</p>
    ServiceException(crate::error::ServiceException),
    /// <p>The quota request template is not associated with your organization.</p>
    ServiceQuotaTemplateNotInUseException(crate::error::ServiceQuotaTemplateNotInUseException),
    /// <p>The specified tag is a reserved word and cannot be used.</p>
    TagPolicyViolationException(crate::error::TagPolicyViolationException),
    /// <p>The Service Quotas template is not available in this AWS Region.</p>
    TemplatesNotAvailableInRegionException(crate::error::TemplatesNotAvailableInRegionException),
    /// <p>Due to throttling, the request was denied. Slow down the rate of request calls, or request
    /// an increase for this quota.</p>
    TooManyRequestsException(crate::error::TooManyRequestsException),
    /// <p>You've exceeded the number of tags allowed for a resource. For more information, see
    /// <a href="https://docs.aws.amazon.com/servicequotas/latest/userguide/sq-tagging.html#sq-tagging-restrictions">Tag
    /// restrictions</a> in the <i>Service Quotas User Guide</i>.</p>
    TooManyTagsException(crate::error::TooManyTagsException),
    /// An unhandled error occurred.
    Unhandled(Box<dyn std::error::Error + Send + Sync + 'static>),
}
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::AwsServiceAccessNotEnabledException(inner) => inner.fmt(f),
            Error::AccessDeniedException(inner) => inner.fmt(f),
            Error::DependencyAccessDeniedException(inner) => inner.fmt(f),
            Error::IllegalArgumentException(inner) => inner.fmt(f),
            Error::InvalidPaginationTokenException(inner) => inner.fmt(f),
            Error::InvalidResourceStateException(inner) => inner.fmt(f),
            Error::NoAvailableOrganizationException(inner) => inner.fmt(f),
            Error::NoSuchResourceException(inner) => inner.fmt(f),
            Error::OrganizationNotInAllFeaturesModeException(inner) => inner.fmt(f),
            Error::QuotaExceededException(inner) => inner.fmt(f),
            Error::ResourceAlreadyExistsException(inner) => inner.fmt(f),
            Error::ServiceException(inner) => inner.fmt(f),
            Error::ServiceQuotaTemplateNotInUseException(inner) => inner.fmt(f),
            Error::TagPolicyViolationException(inner) => inner.fmt(f),
            Error::TemplatesNotAvailableInRegionException(inner) => inner.fmt(f),
            Error::TooManyRequestsException(inner) => inner.fmt(f),
            Error::TooManyTagsException(inner) => inner.fmt(f),
            Error::Unhandled(inner) => inner.fmt(f),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::AssociateServiceQuotaTemplateError, R>>
    for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::AssociateServiceQuotaTemplateError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, ..} => match err.kind {
                crate::error::AssociateServiceQuotaTemplateErrorKind::AccessDeniedException(inner) => Error::AccessDeniedException(inner),
                crate::error::AssociateServiceQuotaTemplateErrorKind::AwsServiceAccessNotEnabledException(inner) => Error::AwsServiceAccessNotEnabledException(inner),
                crate::error::AssociateServiceQuotaTemplateErrorKind::DependencyAccessDeniedException(inner) => Error::DependencyAccessDeniedException(inner),
                crate::error::AssociateServiceQuotaTemplateErrorKind::NoAvailableOrganizationException(inner) => Error::NoAvailableOrganizationException(inner),
                crate::error::AssociateServiceQuotaTemplateErrorKind::OrganizationNotInAllFeaturesModeException(inner) => Error::OrganizationNotInAllFeaturesModeException(inner),
                crate::error::AssociateServiceQuotaTemplateErrorKind::ServiceException(inner) => Error::ServiceException(inner),
                crate::error::AssociateServiceQuotaTemplateErrorKind::TemplatesNotAvailableInRegionException(inner) => Error::TemplatesNotAvailableInRegionException(inner),
                crate::error::AssociateServiceQuotaTemplateErrorKind::TooManyRequestsException(inner) => Error::TooManyRequestsException(inner),
                crate::error::AssociateServiceQuotaTemplateErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            }
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R>
    From<
        aws_smithy_http::result::SdkError<
            crate::error::DeleteServiceQuotaIncreaseRequestFromTemplateError,
            R,
        >,
    > for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<
            crate::error::DeleteServiceQuotaIncreaseRequestFromTemplateError,
            R,
        >,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, ..} => match err.kind {
                crate::error::DeleteServiceQuotaIncreaseRequestFromTemplateErrorKind::AccessDeniedException(inner) => Error::AccessDeniedException(inner),
                crate::error::DeleteServiceQuotaIncreaseRequestFromTemplateErrorKind::AwsServiceAccessNotEnabledException(inner) => Error::AwsServiceAccessNotEnabledException(inner),
                crate::error::DeleteServiceQuotaIncreaseRequestFromTemplateErrorKind::DependencyAccessDeniedException(inner) => Error::DependencyAccessDeniedException(inner),
                crate::error::DeleteServiceQuotaIncreaseRequestFromTemplateErrorKind::IllegalArgumentException(inner) => Error::IllegalArgumentException(inner),
                crate::error::DeleteServiceQuotaIncreaseRequestFromTemplateErrorKind::NoAvailableOrganizationException(inner) => Error::NoAvailableOrganizationException(inner),
                crate::error::DeleteServiceQuotaIncreaseRequestFromTemplateErrorKind::NoSuchResourceException(inner) => Error::NoSuchResourceException(inner),
                crate::error::DeleteServiceQuotaIncreaseRequestFromTemplateErrorKind::ServiceException(inner) => Error::ServiceException(inner),
                crate::error::DeleteServiceQuotaIncreaseRequestFromTemplateErrorKind::TemplatesNotAvailableInRegionException(inner) => Error::TemplatesNotAvailableInRegionException(inner),
                crate::error::DeleteServiceQuotaIncreaseRequestFromTemplateErrorKind::TooManyRequestsException(inner) => Error::TooManyRequestsException(inner),
                crate::error::DeleteServiceQuotaIncreaseRequestFromTemplateErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            }
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R>
    From<aws_smithy_http::result::SdkError<crate::error::DisassociateServiceQuotaTemplateError, R>>
    for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<
            crate::error::DisassociateServiceQuotaTemplateError,
            R,
        >,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, ..} => match err.kind {
                crate::error::DisassociateServiceQuotaTemplateErrorKind::AccessDeniedException(inner) => Error::AccessDeniedException(inner),
                crate::error::DisassociateServiceQuotaTemplateErrorKind::AwsServiceAccessNotEnabledException(inner) => Error::AwsServiceAccessNotEnabledException(inner),
                crate::error::DisassociateServiceQuotaTemplateErrorKind::DependencyAccessDeniedException(inner) => Error::DependencyAccessDeniedException(inner),
                crate::error::DisassociateServiceQuotaTemplateErrorKind::NoAvailableOrganizationException(inner) => Error::NoAvailableOrganizationException(inner),
                crate::error::DisassociateServiceQuotaTemplateErrorKind::ServiceException(inner) => Error::ServiceException(inner),
                crate::error::DisassociateServiceQuotaTemplateErrorKind::ServiceQuotaTemplateNotInUseException(inner) => Error::ServiceQuotaTemplateNotInUseException(inner),
                crate::error::DisassociateServiceQuotaTemplateErrorKind::TemplatesNotAvailableInRegionException(inner) => Error::TemplatesNotAvailableInRegionException(inner),
                crate::error::DisassociateServiceQuotaTemplateErrorKind::TooManyRequestsException(inner) => Error::TooManyRequestsException(inner),
                crate::error::DisassociateServiceQuotaTemplateErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            }
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R>
    From<
        aws_smithy_http::result::SdkError<
            crate::error::GetAssociationForServiceQuotaTemplateError,
            R,
        >,
    > for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<
            crate::error::GetAssociationForServiceQuotaTemplateError,
            R,
        >,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, ..} => match err.kind {
                crate::error::GetAssociationForServiceQuotaTemplateErrorKind::AccessDeniedException(inner) => Error::AccessDeniedException(inner),
                crate::error::GetAssociationForServiceQuotaTemplateErrorKind::AwsServiceAccessNotEnabledException(inner) => Error::AwsServiceAccessNotEnabledException(inner),
                crate::error::GetAssociationForServiceQuotaTemplateErrorKind::DependencyAccessDeniedException(inner) => Error::DependencyAccessDeniedException(inner),
                crate::error::GetAssociationForServiceQuotaTemplateErrorKind::NoAvailableOrganizationException(inner) => Error::NoAvailableOrganizationException(inner),
                crate::error::GetAssociationForServiceQuotaTemplateErrorKind::ServiceException(inner) => Error::ServiceException(inner),
                crate::error::GetAssociationForServiceQuotaTemplateErrorKind::ServiceQuotaTemplateNotInUseException(inner) => Error::ServiceQuotaTemplateNotInUseException(inner),
                crate::error::GetAssociationForServiceQuotaTemplateErrorKind::TemplatesNotAvailableInRegionException(inner) => Error::TemplatesNotAvailableInRegionException(inner),
                crate::error::GetAssociationForServiceQuotaTemplateErrorKind::TooManyRequestsException(inner) => Error::TooManyRequestsException(inner),
                crate::error::GetAssociationForServiceQuotaTemplateErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            }
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::GetAWSDefaultServiceQuotaError, R>>
    for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::GetAWSDefaultServiceQuotaError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::GetAWSDefaultServiceQuotaErrorKind::AccessDeniedException(inner) => {
                    Error::AccessDeniedException(inner)
                }
                crate::error::GetAWSDefaultServiceQuotaErrorKind::IllegalArgumentException(
                    inner,
                ) => Error::IllegalArgumentException(inner),
                crate::error::GetAWSDefaultServiceQuotaErrorKind::NoSuchResourceException(
                    inner,
                ) => Error::NoSuchResourceException(inner),
                crate::error::GetAWSDefaultServiceQuotaErrorKind::ServiceException(inner) => {
                    Error::ServiceException(inner)
                }
                crate::error::GetAWSDefaultServiceQuotaErrorKind::TooManyRequestsException(
                    inner,
                ) => Error::TooManyRequestsException(inner),
                crate::error::GetAWSDefaultServiceQuotaErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R>
    From<aws_smithy_http::result::SdkError<crate::error::GetRequestedServiceQuotaChangeError, R>>
    for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<
            crate::error::GetRequestedServiceQuotaChangeError,
            R,
        >,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::GetRequestedServiceQuotaChangeErrorKind::AccessDeniedException(
                    inner,
                ) => Error::AccessDeniedException(inner),
                crate::error::GetRequestedServiceQuotaChangeErrorKind::IllegalArgumentException(
                    inner,
                ) => Error::IllegalArgumentException(inner),
                crate::error::GetRequestedServiceQuotaChangeErrorKind::NoSuchResourceException(
                    inner,
                ) => Error::NoSuchResourceException(inner),
                crate::error::GetRequestedServiceQuotaChangeErrorKind::ServiceException(inner) => {
                    Error::ServiceException(inner)
                }
                crate::error::GetRequestedServiceQuotaChangeErrorKind::TooManyRequestsException(
                    inner,
                ) => Error::TooManyRequestsException(inner),
                crate::error::GetRequestedServiceQuotaChangeErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::GetServiceQuotaError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: aws_smithy_http::result::SdkError<crate::error::GetServiceQuotaError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::GetServiceQuotaErrorKind::AccessDeniedException(inner) => {
                    Error::AccessDeniedException(inner)
                }
                crate::error::GetServiceQuotaErrorKind::IllegalArgumentException(inner) => {
                    Error::IllegalArgumentException(inner)
                }
                crate::error::GetServiceQuotaErrorKind::NoSuchResourceException(inner) => {
                    Error::NoSuchResourceException(inner)
                }
                crate::error::GetServiceQuotaErrorKind::ServiceException(inner) => {
                    Error::ServiceException(inner)
                }
                crate::error::GetServiceQuotaErrorKind::TooManyRequestsException(inner) => {
                    Error::TooManyRequestsException(inner)
                }
                crate::error::GetServiceQuotaErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R>
    From<
        aws_smithy_http::result::SdkError<
            crate::error::GetServiceQuotaIncreaseRequestFromTemplateError,
            R,
        >,
    > for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<
            crate::error::GetServiceQuotaIncreaseRequestFromTemplateError,
            R,
        >,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, ..} => match err.kind {
                crate::error::GetServiceQuotaIncreaseRequestFromTemplateErrorKind::AccessDeniedException(inner) => Error::AccessDeniedException(inner),
                crate::error::GetServiceQuotaIncreaseRequestFromTemplateErrorKind::AwsServiceAccessNotEnabledException(inner) => Error::AwsServiceAccessNotEnabledException(inner),
                crate::error::GetServiceQuotaIncreaseRequestFromTemplateErrorKind::DependencyAccessDeniedException(inner) => Error::DependencyAccessDeniedException(inner),
                crate::error::GetServiceQuotaIncreaseRequestFromTemplateErrorKind::IllegalArgumentException(inner) => Error::IllegalArgumentException(inner),
                crate::error::GetServiceQuotaIncreaseRequestFromTemplateErrorKind::NoAvailableOrganizationException(inner) => Error::NoAvailableOrganizationException(inner),
                crate::error::GetServiceQuotaIncreaseRequestFromTemplateErrorKind::NoSuchResourceException(inner) => Error::NoSuchResourceException(inner),
                crate::error::GetServiceQuotaIncreaseRequestFromTemplateErrorKind::ServiceException(inner) => Error::ServiceException(inner),
                crate::error::GetServiceQuotaIncreaseRequestFromTemplateErrorKind::TemplatesNotAvailableInRegionException(inner) => Error::TemplatesNotAvailableInRegionException(inner),
                crate::error::GetServiceQuotaIncreaseRequestFromTemplateErrorKind::TooManyRequestsException(inner) => Error::TooManyRequestsException(inner),
                crate::error::GetServiceQuotaIncreaseRequestFromTemplateErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            }
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::ListAWSDefaultServiceQuotasError, R>>
    for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::ListAWSDefaultServiceQuotasError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, ..} => match err.kind {
                crate::error::ListAWSDefaultServiceQuotasErrorKind::AccessDeniedException(inner) => Error::AccessDeniedException(inner),
                crate::error::ListAWSDefaultServiceQuotasErrorKind::IllegalArgumentException(inner) => Error::IllegalArgumentException(inner),
                crate::error::ListAWSDefaultServiceQuotasErrorKind::InvalidPaginationTokenException(inner) => Error::InvalidPaginationTokenException(inner),
                crate::error::ListAWSDefaultServiceQuotasErrorKind::NoSuchResourceException(inner) => Error::NoSuchResourceException(inner),
                crate::error::ListAWSDefaultServiceQuotasErrorKind::ServiceException(inner) => Error::ServiceException(inner),
                crate::error::ListAWSDefaultServiceQuotasErrorKind::TooManyRequestsException(inner) => Error::TooManyRequestsException(inner),
                crate::error::ListAWSDefaultServiceQuotasErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            }
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R>
    From<
        aws_smithy_http::result::SdkError<
            crate::error::ListRequestedServiceQuotaChangeHistoryError,
            R,
        >,
    > for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<
            crate::error::ListRequestedServiceQuotaChangeHistoryError,
            R,
        >,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, ..} => match err.kind {
                crate::error::ListRequestedServiceQuotaChangeHistoryErrorKind::AccessDeniedException(inner) => Error::AccessDeniedException(inner),
                crate::error::ListRequestedServiceQuotaChangeHistoryErrorKind::IllegalArgumentException(inner) => Error::IllegalArgumentException(inner),
                crate::error::ListRequestedServiceQuotaChangeHistoryErrorKind::InvalidPaginationTokenException(inner) => Error::InvalidPaginationTokenException(inner),
                crate::error::ListRequestedServiceQuotaChangeHistoryErrorKind::NoSuchResourceException(inner) => Error::NoSuchResourceException(inner),
                crate::error::ListRequestedServiceQuotaChangeHistoryErrorKind::ServiceException(inner) => Error::ServiceException(inner),
                crate::error::ListRequestedServiceQuotaChangeHistoryErrorKind::TooManyRequestsException(inner) => Error::TooManyRequestsException(inner),
                crate::error::ListRequestedServiceQuotaChangeHistoryErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            }
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R>
    From<
        aws_smithy_http::result::SdkError<
            crate::error::ListRequestedServiceQuotaChangeHistoryByQuotaError,
            R,
        >,
    > for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<
            crate::error::ListRequestedServiceQuotaChangeHistoryByQuotaError,
            R,
        >,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, ..} => match err.kind {
                crate::error::ListRequestedServiceQuotaChangeHistoryByQuotaErrorKind::AccessDeniedException(inner) => Error::AccessDeniedException(inner),
                crate::error::ListRequestedServiceQuotaChangeHistoryByQuotaErrorKind::IllegalArgumentException(inner) => Error::IllegalArgumentException(inner),
                crate::error::ListRequestedServiceQuotaChangeHistoryByQuotaErrorKind::InvalidPaginationTokenException(inner) => Error::InvalidPaginationTokenException(inner),
                crate::error::ListRequestedServiceQuotaChangeHistoryByQuotaErrorKind::NoSuchResourceException(inner) => Error::NoSuchResourceException(inner),
                crate::error::ListRequestedServiceQuotaChangeHistoryByQuotaErrorKind::ServiceException(inner) => Error::ServiceException(inner),
                crate::error::ListRequestedServiceQuotaChangeHistoryByQuotaErrorKind::TooManyRequestsException(inner) => Error::TooManyRequestsException(inner),
                crate::error::ListRequestedServiceQuotaChangeHistoryByQuotaErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            }
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R>
    From<
        aws_smithy_http::result::SdkError<
            crate::error::ListServiceQuotaIncreaseRequestsInTemplateError,
            R,
        >,
    > for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<
            crate::error::ListServiceQuotaIncreaseRequestsInTemplateError,
            R,
        >,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, ..} => match err.kind {
                crate::error::ListServiceQuotaIncreaseRequestsInTemplateErrorKind::AccessDeniedException(inner) => Error::AccessDeniedException(inner),
                crate::error::ListServiceQuotaIncreaseRequestsInTemplateErrorKind::AwsServiceAccessNotEnabledException(inner) => Error::AwsServiceAccessNotEnabledException(inner),
                crate::error::ListServiceQuotaIncreaseRequestsInTemplateErrorKind::DependencyAccessDeniedException(inner) => Error::DependencyAccessDeniedException(inner),
                crate::error::ListServiceQuotaIncreaseRequestsInTemplateErrorKind::IllegalArgumentException(inner) => Error::IllegalArgumentException(inner),
                crate::error::ListServiceQuotaIncreaseRequestsInTemplateErrorKind::NoAvailableOrganizationException(inner) => Error::NoAvailableOrganizationException(inner),
                crate::error::ListServiceQuotaIncreaseRequestsInTemplateErrorKind::ServiceException(inner) => Error::ServiceException(inner),
                crate::error::ListServiceQuotaIncreaseRequestsInTemplateErrorKind::TemplatesNotAvailableInRegionException(inner) => Error::TemplatesNotAvailableInRegionException(inner),
                crate::error::ListServiceQuotaIncreaseRequestsInTemplateErrorKind::TooManyRequestsException(inner) => Error::TooManyRequestsException(inner),
                crate::error::ListServiceQuotaIncreaseRequestsInTemplateErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            }
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::ListServiceQuotasError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::ListServiceQuotasError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::ListServiceQuotasErrorKind::AccessDeniedException(inner) => {
                    Error::AccessDeniedException(inner)
                }
                crate::error::ListServiceQuotasErrorKind::IllegalArgumentException(inner) => {
                    Error::IllegalArgumentException(inner)
                }
                crate::error::ListServiceQuotasErrorKind::InvalidPaginationTokenException(
                    inner,
                ) => Error::InvalidPaginationTokenException(inner),
                crate::error::ListServiceQuotasErrorKind::NoSuchResourceException(inner) => {
                    Error::NoSuchResourceException(inner)
                }
                crate::error::ListServiceQuotasErrorKind::ServiceException(inner) => {
                    Error::ServiceException(inner)
                }
                crate::error::ListServiceQuotasErrorKind::TooManyRequestsException(inner) => {
                    Error::TooManyRequestsException(inner)
                }
                crate::error::ListServiceQuotasErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::ListServicesError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: aws_smithy_http::result::SdkError<crate::error::ListServicesError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::ListServicesErrorKind::AccessDeniedException(inner) => {
                    Error::AccessDeniedException(inner)
                }
                crate::error::ListServicesErrorKind::IllegalArgumentException(inner) => {
                    Error::IllegalArgumentException(inner)
                }
                crate::error::ListServicesErrorKind::InvalidPaginationTokenException(inner) => {
                    Error::InvalidPaginationTokenException(inner)
                }
                crate::error::ListServicesErrorKind::ServiceException(inner) => {
                    Error::ServiceException(inner)
                }
                crate::error::ListServicesErrorKind::TooManyRequestsException(inner) => {
                    Error::TooManyRequestsException(inner)
                }
                crate::error::ListServicesErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::ListTagsForResourceError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::ListTagsForResourceError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::ListTagsForResourceErrorKind::AccessDeniedException(inner) => {
                    Error::AccessDeniedException(inner)
                }
                crate::error::ListTagsForResourceErrorKind::IllegalArgumentException(inner) => {
                    Error::IllegalArgumentException(inner)
                }
                crate::error::ListTagsForResourceErrorKind::NoSuchResourceException(inner) => {
                    Error::NoSuchResourceException(inner)
                }
                crate::error::ListTagsForResourceErrorKind::ServiceException(inner) => {
                    Error::ServiceException(inner)
                }
                crate::error::ListTagsForResourceErrorKind::TooManyRequestsException(inner) => {
                    Error::TooManyRequestsException(inner)
                }
                crate::error::ListTagsForResourceErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R>
    From<
        aws_smithy_http::result::SdkError<
            crate::error::PutServiceQuotaIncreaseRequestIntoTemplateError,
            R,
        >,
    > for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<
            crate::error::PutServiceQuotaIncreaseRequestIntoTemplateError,
            R,
        >,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, ..} => match err.kind {
                crate::error::PutServiceQuotaIncreaseRequestIntoTemplateErrorKind::AccessDeniedException(inner) => Error::AccessDeniedException(inner),
                crate::error::PutServiceQuotaIncreaseRequestIntoTemplateErrorKind::AwsServiceAccessNotEnabledException(inner) => Error::AwsServiceAccessNotEnabledException(inner),
                crate::error::PutServiceQuotaIncreaseRequestIntoTemplateErrorKind::DependencyAccessDeniedException(inner) => Error::DependencyAccessDeniedException(inner),
                crate::error::PutServiceQuotaIncreaseRequestIntoTemplateErrorKind::IllegalArgumentException(inner) => Error::IllegalArgumentException(inner),
                crate::error::PutServiceQuotaIncreaseRequestIntoTemplateErrorKind::NoAvailableOrganizationException(inner) => Error::NoAvailableOrganizationException(inner),
                crate::error::PutServiceQuotaIncreaseRequestIntoTemplateErrorKind::NoSuchResourceException(inner) => Error::NoSuchResourceException(inner),
                crate::error::PutServiceQuotaIncreaseRequestIntoTemplateErrorKind::QuotaExceededException(inner) => Error::QuotaExceededException(inner),
                crate::error::PutServiceQuotaIncreaseRequestIntoTemplateErrorKind::ServiceException(inner) => Error::ServiceException(inner),
                crate::error::PutServiceQuotaIncreaseRequestIntoTemplateErrorKind::TemplatesNotAvailableInRegionException(inner) => Error::TemplatesNotAvailableInRegionException(inner),
                crate::error::PutServiceQuotaIncreaseRequestIntoTemplateErrorKind::TooManyRequestsException(inner) => Error::TooManyRequestsException(inner),
                crate::error::PutServiceQuotaIncreaseRequestIntoTemplateErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            }
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::RequestServiceQuotaIncreaseError, R>>
    for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::RequestServiceQuotaIncreaseError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, ..} => match err.kind {
                crate::error::RequestServiceQuotaIncreaseErrorKind::AccessDeniedException(inner) => Error::AccessDeniedException(inner),
                crate::error::RequestServiceQuotaIncreaseErrorKind::DependencyAccessDeniedException(inner) => Error::DependencyAccessDeniedException(inner),
                crate::error::RequestServiceQuotaIncreaseErrorKind::IllegalArgumentException(inner) => Error::IllegalArgumentException(inner),
                crate::error::RequestServiceQuotaIncreaseErrorKind::InvalidResourceStateException(inner) => Error::InvalidResourceStateException(inner),
                crate::error::RequestServiceQuotaIncreaseErrorKind::NoSuchResourceException(inner) => Error::NoSuchResourceException(inner),
                crate::error::RequestServiceQuotaIncreaseErrorKind::QuotaExceededException(inner) => Error::QuotaExceededException(inner),
                crate::error::RequestServiceQuotaIncreaseErrorKind::ResourceAlreadyExistsException(inner) => Error::ResourceAlreadyExistsException(inner),
                crate::error::RequestServiceQuotaIncreaseErrorKind::ServiceException(inner) => Error::ServiceException(inner),
                crate::error::RequestServiceQuotaIncreaseErrorKind::TooManyRequestsException(inner) => Error::TooManyRequestsException(inner),
                crate::error::RequestServiceQuotaIncreaseErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            }
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::TagResourceError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: aws_smithy_http::result::SdkError<crate::error::TagResourceError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::TagResourceErrorKind::AccessDeniedException(inner) => {
                    Error::AccessDeniedException(inner)
                }
                crate::error::TagResourceErrorKind::IllegalArgumentException(inner) => {
                    Error::IllegalArgumentException(inner)
                }
                crate::error::TagResourceErrorKind::NoSuchResourceException(inner) => {
                    Error::NoSuchResourceException(inner)
                }
                crate::error::TagResourceErrorKind::ServiceException(inner) => {
                    Error::ServiceException(inner)
                }
                crate::error::TagResourceErrorKind::TagPolicyViolationException(inner) => {
                    Error::TagPolicyViolationException(inner)
                }
                crate::error::TagResourceErrorKind::TooManyRequestsException(inner) => {
                    Error::TooManyRequestsException(inner)
                }
                crate::error::TagResourceErrorKind::TooManyTagsException(inner) => {
                    Error::TooManyTagsException(inner)
                }
                crate::error::TagResourceErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::UntagResourceError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: aws_smithy_http::result::SdkError<crate::error::UntagResourceError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::UntagResourceErrorKind::AccessDeniedException(inner) => {
                    Error::AccessDeniedException(inner)
                }
                crate::error::UntagResourceErrorKind::IllegalArgumentException(inner) => {
                    Error::IllegalArgumentException(inner)
                }
                crate::error::UntagResourceErrorKind::NoSuchResourceException(inner) => {
                    Error::NoSuchResourceException(inner)
                }
                crate::error::UntagResourceErrorKind::ServiceException(inner) => {
                    Error::ServiceException(inner)
                }
                crate::error::UntagResourceErrorKind::TooManyRequestsException(inner) => {
                    Error::TooManyRequestsException(inner)
                }
                crate::error::UntagResourceErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl std::error::Error for Error {}
