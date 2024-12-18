use crate::decl::*;

pub trait Workflow {
    fn get_decl(&self) -> WorkflowDecl;

    fn get_next_nodes(&mut self) -> Vec<Box<dyn Node<Self>>>;
}

pub trait Node<W: Workflow> {
    fn get_decl(&self) -> NodeDecl;
}