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
    pub async fn eval(&self, expression: impl Into<String>) -> Result<i32> {
        eval_expr(GetEvalDenoRuntime, expression).await

        // let expression = expression.into();
        //
        // let val = tokio::task::spawn_blocking(move || {
        //     let runtime = tokio::runtime::Builder::new_current_thread()
        //         .enable_all()
        //         .build()
        //         .map_err(|err| Error::ExecuteError(err.to_string()))?;
        //
        //     runtime.block_on(async {
        //         let mut runtime = Self::get_js_runtime();
        //
        //         let result = (&mut runtime).execute_script("", expression);
        //
        //         match result {
        //             Err(err) => Err(Error::from_deno_execute_script_error(err)),
        //             Ok(val) => from_v8_value(&mut runtime, val).await,
        //         }
        //     })
        // })
        // .await??;
        //
        // Ok(val)

        // Ok(233)
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
