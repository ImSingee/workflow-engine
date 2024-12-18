use thiserror::Error;

pub type ExecuteResult = Result<ExecuteOutput, ExecuteError>;

pub type ExecuteOutput = ();

#[derive(Debug, Error)]
pub enum ExecuteError {}
