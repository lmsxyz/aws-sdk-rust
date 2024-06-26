// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// Matcher union: {"output":{"path":"assetStatus.state","expected":"ACTIVE","comparator":"stringEquals"}}
pub(crate) fn match_describe_asset_64fe9a784b7fe20de(
    _result: ::std::result::Result<&crate::operation::describe_asset::DescribeAssetOutput, &crate::operation::describe_asset::DescribeAssetError>,
) -> bool {
    fn path_traversal<'a>(_output: &'a crate::operation::describe_asset::DescribeAssetOutput) -> ::std::option::Option<&'a crate::types::AssetState> {
        let _fld_1 = _output.asset_status.as_ref()?;
        let _fld_2 = &_fld_1.state;
        ::std::option::Option::Some(_fld_2)
    }
    _result
        .as_ref()
        .ok()
        .and_then(|output| path_traversal(output))
        .map(|value| {
            let _tmp_2 = value.as_str();
            let right = "ACTIVE";
            let _cmp_1 = _tmp_2 == right;
            _cmp_1
        })
        .unwrap_or_default()
}

/// Matcher union: {"output":{"path":"assetStatus.state","expected":"FAILED","comparator":"stringEquals"}}
pub(crate) fn match_describe_asset_dc39ba47e8ffcf272(
    _result: ::std::result::Result<&crate::operation::describe_asset::DescribeAssetOutput, &crate::operation::describe_asset::DescribeAssetError>,
) -> bool {
    fn path_traversal<'a>(_output: &'a crate::operation::describe_asset::DescribeAssetOutput) -> ::std::option::Option<&'a crate::types::AssetState> {
        let _fld_1 = _output.asset_status.as_ref()?;
        let _fld_2 = &_fld_1.state;
        ::std::option::Option::Some(_fld_2)
    }
    _result
        .as_ref()
        .ok()
        .and_then(|output| path_traversal(output))
        .map(|value| {
            let _tmp_2 = value.as_str();
            let right = "FAILED";
            let _cmp_1 = _tmp_2 == right;
            _cmp_1
        })
        .unwrap_or_default()
}

/// Matcher union: {"output":{"path":"assetModelStatus.state","expected":"ACTIVE","comparator":"stringEquals"}}
pub(crate) fn match_describe_asset_model_356d04b58e720c0ba(
    _result: ::std::result::Result<
        &crate::operation::describe_asset_model::DescribeAssetModelOutput,
        &crate::operation::describe_asset_model::DescribeAssetModelError,
    >,
) -> bool {
    fn path_traversal<'a>(
        _output: &'a crate::operation::describe_asset_model::DescribeAssetModelOutput,
    ) -> ::std::option::Option<&'a crate::types::AssetModelState> {
        let _fld_1 = _output.asset_model_status.as_ref()?;
        let _fld_2 = &_fld_1.state;
        ::std::option::Option::Some(_fld_2)
    }
    _result
        .as_ref()
        .ok()
        .and_then(|output| path_traversal(output))
        .map(|value| {
            let _tmp_2 = value.as_str();
            let right = "ACTIVE";
            let _cmp_1 = _tmp_2 == right;
            _cmp_1
        })
        .unwrap_or_default()
}

/// Matcher union: {"output":{"path":"assetModelStatus.state","expected":"FAILED","comparator":"stringEquals"}}
pub(crate) fn match_describe_asset_model_ff8b4ad9689c7214b(
    _result: ::std::result::Result<
        &crate::operation::describe_asset_model::DescribeAssetModelOutput,
        &crate::operation::describe_asset_model::DescribeAssetModelError,
    >,
) -> bool {
    fn path_traversal<'a>(
        _output: &'a crate::operation::describe_asset_model::DescribeAssetModelOutput,
    ) -> ::std::option::Option<&'a crate::types::AssetModelState> {
        let _fld_1 = _output.asset_model_status.as_ref()?;
        let _fld_2 = &_fld_1.state;
        ::std::option::Option::Some(_fld_2)
    }
    _result
        .as_ref()
        .ok()
        .and_then(|output| path_traversal(output))
        .map(|value| {
            let _tmp_2 = value.as_str();
            let right = "FAILED";
            let _cmp_1 = _tmp_2 == right;
            _cmp_1
        })
        .unwrap_or_default()
}

/// Matcher union: {"errorType":"ResourceNotFoundException"}
pub(crate) fn match_describe_asset_model_1cce2c05524fb92d4(
    _result: ::std::result::Result<
        &crate::operation::describe_asset_model::DescribeAssetModelOutput,
        &crate::operation::describe_asset_model::DescribeAssetModelError,
    >,
) -> bool {
    if let ::std::result::Result::Err(err) = _result {
        if let ::std::option::Option::Some(code) = ::aws_smithy_types::error::metadata::ProvideErrorMetadata::code(err) {
            return code == "ResourceNotFoundException";
        }
    }
    false
}

/// Matcher union: {"errorType":"ResourceNotFoundException"}
pub(crate) fn match_describe_asset_1cce2c05524fb92d4(
    _result: ::std::result::Result<&crate::operation::describe_asset::DescribeAssetOutput, &crate::operation::describe_asset::DescribeAssetError>,
) -> bool {
    if let ::std::result::Result::Err(err) = _result {
        if let ::std::option::Option::Some(code) = ::aws_smithy_types::error::metadata::ProvideErrorMetadata::code(err) {
            return code == "ResourceNotFoundException";
        }
    }
    false
}

/// Matcher union: {"output":{"path":"portalStatus.state","expected":"ACTIVE","comparator":"stringEquals"}}
pub(crate) fn match_describe_portal_baa8d81b8097ff5e4(
    _result: ::std::result::Result<&crate::operation::describe_portal::DescribePortalOutput, &crate::operation::describe_portal::DescribePortalError>,
) -> bool {
    fn path_traversal<'a>(
        _output: &'a crate::operation::describe_portal::DescribePortalOutput,
    ) -> ::std::option::Option<&'a crate::types::PortalState> {
        let _fld_1 = _output.portal_status.as_ref()?;
        let _fld_2 = &_fld_1.state;
        ::std::option::Option::Some(_fld_2)
    }
    _result
        .as_ref()
        .ok()
        .and_then(|output| path_traversal(output))
        .map(|value| {
            let _tmp_2 = value.as_str();
            let right = "ACTIVE";
            let _cmp_1 = _tmp_2 == right;
            _cmp_1
        })
        .unwrap_or_default()
}

/// Matcher union: {"errorType":"ResourceNotFoundException"}
pub(crate) fn match_describe_portal_1cce2c05524fb92d4(
    _result: ::std::result::Result<&crate::operation::describe_portal::DescribePortalOutput, &crate::operation::describe_portal::DescribePortalError>,
) -> bool {
    if let ::std::result::Result::Err(err) = _result {
        if let ::std::option::Option::Some(code) = ::aws_smithy_types::error::metadata::ProvideErrorMetadata::code(err) {
            return code == "ResourceNotFoundException";
        }
    }
    false
}
