use crate::decl::*;
use thiserror::Error;

#[non_exhaustive]
#[derive(Debug, Default)]
pub struct WorkflowDeclBuilder {
    pub id: String,
    pub name: Option<String>,
}

impl WorkflowDeclBuilder {
    /// create a WorkflowDeclBuilder for building WorkflowDecl
    ///
    /// id: the workflow id; this id should only relate to the workflow itself (not the execution);
    ///     if the id isn't useful in your purpose, simply pass in an empty string
    ///     the id can only contain letters, numbers and underscores and start with letters
    pub fn new(id: String) -> Self {
        Self {
            id,
            ..Default::default()
        }
    }

    /// set the workflow name, this name is just for display purpose
    ///
    /// if the passed in name is empty, this function will unset the name
    pub fn name(&mut self, name: String) -> &mut Self {
        if name.is_empty() {
            self.name = None
        } else {
            self.name = Some(name);
        }

        self
    }

    pub fn build(self) -> Result<WorkflowDecl, WorkflowDeclBuildError> {
        let id = if self.id.is_empty() {
            // TODO generate a random workflow id for this

            return Err(WorkflowDeclBuildError::InvalidID(self.id));
        } else {
            // TODO check format

            self.id
        };

        Ok(WorkflowDecl {
            id,
            name: self.name,
        })
    }
}

#[non_exhaustive]
#[derive(Debug, Error)]
pub enum WorkflowDeclBuildError {
    #[error("workflow id {0} is invalid")]
    InvalidID(String),
}

#[non_exhaustive]
#[derive(Debug, Default)]
pub struct NodeDeclBuilder {
    pub id: String,
    pub name: Option<String>,
}

impl NodeDeclBuilder {
    /// create a NodeDeclBuilder for building NodeDecl
    ///
    /// id: the node id; this id must be unique in a workflow, though the builder can't check it
    ///     if you really don't need an id, simply pass in an empty string
    ///     the id can only contain letters, numbers and underscores and start with letters
    pub fn new(id: String) -> Self {
        Self {
            id,
            ..Default::default()
        }
    }

    /// set the node name, this name is just for display purpose
    ///
    /// if the passed in name is empty, this function will unset the name
    pub fn name(&mut self, name: String) -> &mut Self {
        if name.is_empty() {
            self.name = None
        } else {
            self.name = Some(name);
        }

        self
    }

    pub fn build(self) -> Result<NodeDecl, NodeDeclBuildError> {
        let id = if self.id.is_empty() {
            // TODO generate a random node id for this

            return Err(NodeDeclBuildError::InvalidID(self.id));
        } else {
            // TODO check format

            self.id
        };

        Ok(NodeDecl {
            id,
            name: self.name,
        })
    }
}

#[non_exhaustive]
#[derive(Debug, Error)]
pub enum NodeDeclBuildError {
    #[error("node id {0} is invalid")]
    InvalidID(String),
}
