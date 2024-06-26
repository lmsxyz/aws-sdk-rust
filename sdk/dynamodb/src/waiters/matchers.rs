// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// Matcher union: {"output":{"path":"Table.TableStatus","expected":"ACTIVE","comparator":"stringEquals"}}
pub(crate) fn match_describe_table_0429b99996ae6dab6(
    _result: ::std::result::Result<&crate::operation::describe_table::DescribeTableOutput, &crate::operation::describe_table::DescribeTableError>,
) -> bool {
    fn path_traversal<'a>(
        _output: &'a crate::operation::describe_table::DescribeTableOutput,
    ) -> ::std::option::Option<&'a crate::types::TableStatus> {
        let _fld_1 = _output.table.as_ref()?;
        let _fld_2 = _fld_1.table_status.as_ref()?;
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
pub(crate) fn match_describe_table_1cce2c05524fb92d4(
    _result: ::std::result::Result<&crate::operation::describe_table::DescribeTableOutput, &crate::operation::describe_table::DescribeTableError>,
) -> bool {
    if let ::std::result::Result::Err(err) = _result {
        if let ::std::option::Option::Some(code) = ::aws_smithy_types::error::metadata::ProvideErrorMetadata::code(err) {
            return code == "ResourceNotFoundException";
        }
    }
    false
}
