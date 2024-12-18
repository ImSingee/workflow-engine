use crate::workflow::WorkflowDef;
use anyhow::Result;

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

    println!("workflow definition:\n{:#?}", workflow_def);

    // TODO create an workflow engine (runner)

    // TODO submit the workflow to the engine

    Ok(())
}
