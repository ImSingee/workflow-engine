use workflow_engine_core as core;
use workflow_engine_core::NodeDecl;

#[derive(Debug)]
pub struct Workflow {
    step: usize,
}

impl Workflow {
    pub fn demo() -> Self {
        Self { step: 1 }
    }
}

impl Workflow {
    pub(crate) fn mark_current_node_success(&mut self) {
        self.step += 1;
    }
}

impl core::Workflow for Workflow {
    fn get_decl(&self) -> core::WorkflowDecl {
        todo!()
    }

    fn get_next_node(&mut self) -> Option<NodeRef> {
        match self.step {
            1 => Some(NodeRef::new(self, 1)),
            2 => Some(NodeRef::new(self, 2)),
            _ => None,
        }
    }
}

#[derive(Debug)]
pub struct Node {}

#[derive(Debug)]
pub(crate) struct NodeRef<'w> {
    workflow: &'w mut Workflow,
    step: usize,
}

impl<'w> NodeRef<'w> {
    fn new(workflow: &'w mut Workflow, step: usize) -> Self {
        Self { workflow, step }
    }

    pub fn workflow(&mut self) -> &mut Workflow {
        self.workflow
    }
}

impl<'w> core::Node<Workflow> for NodeRef<'w> {
    fn get_decl(&self) -> NodeDecl {
        todo!()
    }
}
