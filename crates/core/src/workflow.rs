use crate::decl::*;

pub trait Workflow {
    fn get_decl(&self) -> WorkflowDecl;

    fn get_next_node(&mut self) -> Option<impl Node<Self>>;
}

pub trait Node<W: Workflow + ?Sized> {
    fn get_decl(&self) -> NodeDecl;

    // fn get_action(&mut self) -> impl Action<W, Self>;
}

// pub trait Action<W: Workflow, N: Node<W>> {
//     // type Input;
//     // type Output;
//     // type Error;
//
//     async fn execute(&self, input: Self::Input) -> Result<Self::Output, Self::Error>;
// }