use super::deno_runtime::{DenoRuntime, GetDenoRuntime};
use super::result::{Error, Result};
use super::utils::with_timeout;
use deno_core::v8::{Global, Value};
use deno_core::{serde_v8, v8, PollEventLoopOptions};
use std::time::Duration;

#[derive(Default, Clone)]
pub struct EvalOptions {
    timeout: Option<Duration>,
}

pub async fn eval_expr<T: serde::de::DeserializeOwned + Send + 'static>(
    get_deno_runtime: impl GetDenoRuntime + 'static,
    expression: impl Into<String>,
    options: EvalOptions,
) -> Result<T> {
    let expression = expression.into();

    let task = tokio::task::spawn_blocking(move || {
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
    });

    let task = with_timeout(options.timeout, task);

    let val = task.await???;

    Ok(val)
}

pub async fn from_v8_value<T: serde::de::DeserializeOwned>(
    runtime: &mut DenoRuntime,
    val: Global<Value>,
) -> Result<T> {
    let promise = runtime.resolve(val);
    let val = runtime
        .with_event_loop_promise(promise, PollEventLoopOptions::default())
        .await
        .map_err(Error::from_deno_execute_script_error)?;

    let scope = &mut runtime.handle_scope();
    let local = v8::Local::new(scope, val);

    serde_v8::from_v8::<T>(scope, local).map_err(|err| Error::InvalidResultError(err.to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::deno_runtime::GetDenoRuntimeResult;
    use num_bigint::BigInt;

    struct GetEvalDenoRuntime;
    impl GetDenoRuntime for GetEvalDenoRuntime {
        fn get(self) -> GetDenoRuntimeResult {
            DenoRuntime::try_new(Default::default())
        }
    }

    #[tokio::test]
    async fn test_eval_expr() {
        let default_eval_options: EvalOptions = Default::default();

        assert_eq!(
            eval_expr(GetEvalDenoRuntime, "1+1", default_eval_options.clone()).await,
            Ok(2)
        ); // int
        assert_eq!(
            eval_expr(GetEvalDenoRuntime, "1.1+1.2", default_eval_options.clone()).await,
            Ok(2.3)
        ); // double
        assert_eq!(
            eval_expr(
                GetEvalDenoRuntime,
                "true && false",
                default_eval_options.clone()
            )
            .await,
            Ok(false)
        ); // bool
        assert_eq!(
            eval_expr(GetEvalDenoRuntime, "'hello'", default_eval_options.clone()).await,
            Ok("hello".to_string())
        ); // string
        assert_eq!(
            eval_expr(GetEvalDenoRuntime, "1n+1n", default_eval_options.clone()).await,
            Ok(2)
        ); // bigint
        assert_eq!(
            eval_expr(GetEvalDenoRuntime, "1n+1n", default_eval_options.clone()).await,
            Ok(serde_v8::BigInt::from(BigInt::from(2)))
        ); // bigint
        assert_eq!(
            eval_expr(
                GetEvalDenoRuntime,
                format!("{}n+{}n", u64::MAX, u64::MAX),
                default_eval_options.clone()
            )
            .await,
            Ok(serde_v8::BigInt::from(BigInt::from(u64::MAX as u128 * 2)))
        ); // bigint

        assert_eq!(
            eval_expr(GetEvalDenoRuntime, "null", default_eval_options.clone()).await,
            Ok(None::<i32>)
        ); // null
        assert_eq!(
            eval_expr(GetEvalDenoRuntime, "null", default_eval_options.clone()).await,
            Ok(())
        ); // null
        assert_eq!(
            eval_expr(GetEvalDenoRuntime, "1+1", default_eval_options.clone()).await,
            Ok(Some(2))
        ); // optional int

        assert_eq!(
            eval_expr(GetEvalDenoRuntime, "", default_eval_options.clone()).await,
            Ok(None::<i32>)
        ); // undefined
        assert_eq!(
            eval_expr(
                GetEvalDenoRuntime,
                "undefined",
                default_eval_options.clone()
            )
            .await,
            Ok(None::<i32>)
        ); // undefined
        assert_eq!(
            eval_expr(GetEvalDenoRuntime, "", default_eval_options.clone()).await,
            Ok(())
        ); // undefined
        assert_eq!(
            eval_expr(
                GetEvalDenoRuntime,
                "undefined",
                default_eval_options.clone()
            )
            .await,
            Ok(())
        ); // undefined

        assert_eq!(
            eval_expr(
                GetEvalDenoRuntime,
                "Promise.resolve(2)",
                default_eval_options.clone()
            )
            .await,
            Ok(2)
        ); // Promise<int>

        // any value
        let val = eval_expr::<serde_v8::AnyValue>(
            GetEvalDenoRuntime,
            "1+1",
            default_eval_options.clone(),
        )
        .await
        .unwrap();
        assert!(matches!(val, serde_v8::AnyValue::Number(2f64)));
    }

    #[tokio::test]
    async fn test_eval_expr_not_timeout() {
        eval_expr(
            GetEvalDenoRuntime,
            "Promise.resolve(2)",
            EvalOptions {
                timeout: Some(Duration::from_millis(100)),
            },
        )
        .await
        .unwrap()
    }

    #[tokio::test]
    #[should_panic(expected = "timeout")]
    async fn test_eval_expr_timeout() {
        eval_expr(
            GetEvalDenoRuntime,
            "Promise.resolve(2)",
            EvalOptions {
                timeout: Some(Duration::from_millis(1)),
            },
        )
        .await
        .unwrap()
    }
}
