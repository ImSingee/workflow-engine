use workflow_engine_core as core;
use workflow_engine_core::NodeDecl;

#[derive(Debug)]
pub struct DemoWorkflow {
    step: usize,
}

impl DemoWorkflow {
    pub fn new() -> Self {
        Self { step: 1 }
    }

    pub(crate) fn mark_current_node_success(&mut self) {
        self.step += 1;
    }
}

impl core::Workflow for DemoWorkflow {
    fn get_decl(&self) -> core::WorkflowDecl {
        todo!()
    }

    fn get_next_node(&mut self) -> Option<DemoNode> {
        match self.step {
            1 => Some(DemoNode::new(self, 1)),
            2 => Some(DemoNode::new(self, 2)),
            _ => None,
        }
    }
}

#[derive(Debug)]
pub struct DemoNode<'w> {
    workflow: &'w mut DemoWorkflow,
    step: usize,
}

impl<'w> DemoNode<'w> {
    fn new(workflow: &'w mut DemoWorkflow, step: usize) -> Self {
        Self { workflow, step }
    }

    pub(crate) fn workflow(&mut self) -> &mut DemoWorkflow {
        self.workflow
    }
}

impl<'w> core::Node<DemoWorkflow> for DemoNode<'w> {
    fn get_decl(&self) -> NodeDecl {
        todo!()
    }
}
