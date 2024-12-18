use crate::workflow::WorkflowDef;
use anyhow::Result;
use std::sync::Arc;
use workflow_engine::{DemoWorkflow, Registry, WorkflowEngine};

mod actions;
mod workflow;

#[tokio::main]
async fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();

    let workflow_file_path = if args.len() >= 2 {
        args[1].to_string()
    } else {
        format!(
            "{}/single-workflow/workflow.yml",
            env!("CARGO_MANIFEST_DIR")
        )
    };

    let workflow_def = WorkflowDef::read_from_yaml_file(&workflow_file_path)?;

    // TODO convert workflow_def to Workflow
    let workflow = DemoWorkflow::new();

    println!("Workflow Definition:\n{:#?}", workflow_def);

    let mut registry = Registry::new();

    let engine = WorkflowEngine::new(Arc::new(registry));

    let input = ();
    let options = ();
    let result = engine.execute_workflow(workflow, input, options).await?;
    println!("Execute Result:\n{:#?}", result);

    Ok(())
}
