use super::result::Result;
use crate::deno_runtime::{DenoRuntime, GetDenoRuntime, GetDenoRuntimeResult};
use crate::eval::eval_expr;
use deno_core::RuntimeOptions;
use derive_builder::Builder;

#[derive(Builder)]
#[builder(pattern = "owned")]
pub struct EvalExecutor {}

impl EvalExecutor {
    /// evaluate the expression
    pub async fn eval<T: serde::de::DeserializeOwned + Send+'static>(
        &self,
        expression: impl Into<String>,
    ) -> Result<T> {
        eval_expr(GetEvalDenoRuntime, expression).await
    }

    pub fn builder() -> EvalExecutorBuilder {
        EvalExecutorBuilder::default()
    }
}

struct GetEvalDenoRuntime;
impl GetDenoRuntime for GetEvalDenoRuntime {
    fn get(self) -> GetDenoRuntimeResult {
        DenoRuntime::try_new(RuntimeOptions {
            ..Default::default()
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn new_executor() -> EvalExecutor {
        let executor = EvalExecutor::builder().build().unwrap();

        executor
    }

    #[tokio::test]
    async fn test_execute() {
        let executor = new_executor();

        assert_eq!(executor.eval("1+1").await, Ok(2));
    }
}
