use super::deno_runtime::{DenoRuntime, GetDenoRuntime};
use super::result::{Error, Result};
use deno_core::v8::{Global, Value};
use deno_core::{v8, PollEventLoopOptions};

pub async fn eval_expr(
    get_deno_runtime: impl GetDenoRuntime + 'static,
    expression: impl Into<String>,
) -> Result<i32> {
    let expression = expression.into();

    let val = tokio::task::spawn_blocking(move || {
        let runtime = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .map_err(|err| Error::ExecuteError(err.to_string()))?;

        runtime.block_on(async {
            let mut deno_runtime = {
                match get_deno_runtime.get() {
                    Ok(rt) => rt,
                    Err(err) => return Err(Error::ExecuteError(err.to_string())),
                }
            };

            let result = (&mut deno_runtime).execute_script("", expression);
            match result {
                Err(err) => Err(Error::from_deno_execute_script_error(err)),
                Ok(val) => from_v8_value(&mut deno_runtime, val).await,
            }
        })
    })
    .await??;

    Ok(val)
}

pub async fn from_v8_value(runtime: &mut DenoRuntime, val: Global<Value>) -> Result<i32> {
    let promise = runtime.resolve(val);
    let val = runtime
        .with_event_loop_promise(promise, PollEventLoopOptions::default())
        .await
        .map_err(Error::from_deno_execute_script_error)?;

    let scope = &mut runtime.handle_scope();
    let local = v8::Local::new(scope, val);

    serde_v8::from_v8::<i32>(scope, local).map_err(|err| Error::InvalidResultError(err.to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::deno_runtime::GetDenoRuntimeResult;

    struct GetEvalDenoRuntime;
    impl GetDenoRuntime for GetEvalDenoRuntime {
        fn get(self) -> GetDenoRuntimeResult {
            DenoRuntime::try_new(Default::default())
        }
    }

    #[tokio::test]
    async fn test_eval_expr() {
        assert_eq!(eval_expr(GetEvalDenoRuntime, "1+1").await, Ok(2));
    }
}
