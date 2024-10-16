use aide::transform::TransformOperation;

pub mod extractors;

pub fn route_documentation<'a>(
    summary: &'a str,
    description: &'a str,
) -> impl FnOnce(TransformOperation) -> TransformOperation + 'a {
    |op| op.summary(summary).description(description)
}
