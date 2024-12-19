use std::ops::{Deref, DerefMut};
use workflow_engine_core as core;
use workflow_engine_core::NodeDecl;

#[derive(Debug)]
pub struct Workflow {
    nodes: Vec<Node>,
    current_node_index: usize,
}

impl Workflow {
    // TODO only for temp use, remove in production
    pub fn demo() -> Self {
        let nodes = vec![];

        Self::new(nodes)
    }
}

impl Workflow {
    pub(crate) fn new(nodes: Vec<Node>) -> Self {
        Self {
            nodes,
            current_node_index: 0,
        }
    }

    pub(crate) fn mark_current_node_success(&mut self) {
        self.current_node_index += 1;
    }
}

impl core::Workflow for Workflow {
    fn get_decl(&self) -> core::WorkflowDecl {
        todo!()
    }

    fn get_next_node(&mut self) -> Option<NodeRef> {
        if self.current_node_index >= self.nodes.len() {
            return None;
        }

        Some(NodeRef::new(self, self.current_node_index))
    }
}

#[derive(Debug)]
pub struct Node {}

impl Node {}

impl core::Node<Workflow> for Node {
    fn get_decl(&self) -> NodeDecl {
        todo!()
    }
}

#[derive(Debug)]
pub(crate) struct NodeRef<'w> {
    workflow: &'w mut Workflow,
    node_index: usize,
}

impl<'w> NodeRef<'w> {
    fn new(workflow: &'w mut Workflow, node_index: usize) -> Self {
        Self {
            workflow,
            node_index,
        }
    }

    pub fn get(&mut self) -> (&mut Workflow, &mut Node) {
        let workflow_ptr = self.workflow as *mut Workflow;
        let node_ptr = unsafe { &mut (*workflow_ptr).nodes[self.node_index] as *mut Node };

        unsafe { (&mut *workflow_ptr, &mut *node_ptr) }
    }
}

impl<'w> Deref for NodeRef<'w> {
    type Target = Node;

    fn deref(&self) -> &Self::Target {
        &self.workflow.nodes[self.node_index]
    }
}

impl<'w> DerefMut for NodeRef<'w> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.workflow.nodes[self.node_index]
    }
}

impl<'w> core::Node<Workflow> for NodeRef<'w> {
    fn get_decl(&self) -> NodeDecl {
        self.deref().get_decl()
    }
}
