#[non_exhaustive]
#[derive(Default)]
/// Defines a workflow's basic info
pub struct WorkflowDecl {
    pub(crate) id: String,
    pub(crate) name: Option<String>,
}

#[non_exhaustive]
#[derive(Default)]
/// Defines a node's basic info
pub struct NodeDecl {
    pub(crate) id: String,
    pub(crate) name: Option<String>,
}
