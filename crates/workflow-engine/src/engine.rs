use crate::execute::{ExecuteError, ExecuteOutput, ExecuteResult};
use crate::workflow::DemoWorkflow;
use crate::{DemoNode, Registry};
use std::sync::Arc;
use workflow_engine_core::Workflow as CoreWorkflow;

pub struct WorkflowEngine {
    registry: Arc<Registry>,
}

impl WorkflowEngine {
    /// note: this is for development only - it will be replaced by a builder in the future
    pub fn new(registry: Arc<Registry>) -> Self {
        Self { registry }
    }

    pub async fn execute_workflow(
        &self,
        mut workflow: DemoWorkflow, // TODO generic
        input: (),
        options: (),
    ) -> ExecuteResult {
        let mut last_step_output: Option<ExecuteOutput> = None;

        loop {
            let node = match workflow.get_next_node() {
                None => break,
                Some(node) => node,
            };

            println!("node: {:?}", node);

            let node_result = self.execute_node(node).await;
            match node_result {
                Ok(output) => last_step_output = Some(output),
                Err(err) => {
                    // TODO attach more context e.g. current node

                    return Err(err);
                }
            }
        }

        match last_step_output {
            Some(output) => Ok(output),
            None => {
                // TODO no steps, should there be an error?
                Ok(())
            }
        }
    }

    async fn execute_node(&self, mut node: DemoNode<'_>) -> ExecuteResult {
        let workflow = node.workflow();

        // TODO execute action
        let output = ();

        // mark node done
        workflow.mark_current_node_success();

        Ok(output)
    }
}

// pub struct WorkflowEngineBuilder {}
//
// impl WorkflowEngine {
//     pub fn builder() -> WorkflowEngineBuilder {
//         WorkflowEngineBuilder {}
//     }
// }
//
// impl WorkflowEngineBuilder {
//     pub fn build(self) -> Result<WorkflowEngine, WorkflowEngineBuildError> {
//         Ok(WorkflowEngine {})
//     }
// }
//
// #[derive(Debug, Error)]
// #[non_exhaustive]
// pub enum WorkflowEngineBuildError {}
