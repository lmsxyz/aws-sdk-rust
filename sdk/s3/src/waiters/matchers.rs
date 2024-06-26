// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// Matcher union: {"success":true}
pub(crate) fn match_head_bucket_c955e57777ec0d736(
    _result: ::std::result::Result<&crate::operation::head_bucket::HeadBucketOutput, &crate::operation::head_bucket::HeadBucketError>,
) -> bool {
    _result.is_ok()
}

/// Matcher union: {"errorType":"NotFound"}
pub(crate) fn match_head_bucket_01223db2f9ae15f05(
    _result: ::std::result::Result<&crate::operation::head_bucket::HeadBucketOutput, &crate::operation::head_bucket::HeadBucketError>,
) -> bool {
    if let ::std::result::Result::Err(err) = _result {
        if let ::std::option::Option::Some(code) = ::aws_smithy_types::error::metadata::ProvideErrorMetadata::code(err) {
            return code == "NotFound";
        }
    }
    false
}

/// Matcher union: {"success":true}
pub(crate) fn match_head_object_c955e57777ec0d736(
    _result: ::std::result::Result<&crate::operation::head_object::HeadObjectOutput, &crate::operation::head_object::HeadObjectError>,
) -> bool {
    _result.is_ok()
}

/// Matcher union: {"errorType":"NotFound"}
pub(crate) fn match_head_object_01223db2f9ae15f05(
    _result: ::std::result::Result<&crate::operation::head_object::HeadObjectOutput, &crate::operation::head_object::HeadObjectError>,
) -> bool {
    if let ::std::result::Result::Err(err) = _result {
        if let ::std::option::Option::Some(code) = ::aws_smithy_types::error::metadata::ProvideErrorMetadata::code(err) {
            return code == "NotFound";
        }
    }
    false
}
