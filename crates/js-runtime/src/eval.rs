use super::deno_runtime::{DenoRuntime, GetDenoRuntime};
use super::result::{Error, Result};
use deno_core::v8::{Global, Value};
use deno_core::{serde_v8, v8, PollEventLoopOptions};

pub async fn eval_expr<T: serde::de::DeserializeOwned + Send + 'static>(
    get_deno_runtime: impl GetDenoRuntime + 'static,
    expression: impl Into<String>,
) -> Result<T> {
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
        assert_eq!(eval_expr(GetEvalDenoRuntime, "1+1").await, Ok(2)); // int
        assert_eq!(eval_expr(GetEvalDenoRuntime, "1.1+1.2").await, Ok(2.3)); // double
        assert_eq!(
            eval_expr(GetEvalDenoRuntime, "true && false").await,
            Ok(false)
        ); // bool
        assert_eq!(
            eval_expr(GetEvalDenoRuntime, "'hello'").await,
            Ok("hello".to_string())
        ); // string
        assert_eq!(eval_expr(GetEvalDenoRuntime, "1n+1n").await, Ok(2)); // bigint
        assert_eq!(
            eval_expr(GetEvalDenoRuntime, "1n+1n").await,
            Ok(serde_v8::BigInt::from(BigInt::from(2)))
        ); // bigint
        assert_eq!(
            eval_expr(GetEvalDenoRuntime, format!("{}n+{}n", u64::MAX, u64::MAX)).await,
            Ok(serde_v8::BigInt::from(BigInt::from(u64::MAX as u128 * 2)))
        ); // bigint

        assert_eq!(eval_expr(GetEvalDenoRuntime, "null").await, Ok(None::<i32>)); // null
        assert_eq!(eval_expr(GetEvalDenoRuntime, "null").await, Ok(())); // null
        assert_eq!(eval_expr(GetEvalDenoRuntime, "1+1").await, Ok(Some(2))); // optional int

        assert_eq!(eval_expr(GetEvalDenoRuntime, "").await, Ok(None::<i32>)); // undefined
        assert_eq!(
            eval_expr(GetEvalDenoRuntime, "undefined").await,
            Ok(None::<i32>)
        ); // undefined
        assert_eq!(eval_expr(GetEvalDenoRuntime, "").await, Ok(())); // undefined
        assert_eq!(
            eval_expr(GetEvalDenoRuntime, "undefined").await,
            Ok(())
        ); // undefined

        assert_eq!(
            eval_expr(GetEvalDenoRuntime, "Promise.resolve(2)").await,
            Ok(2)
        ); // Promise<int>

        // any value
        let val = eval_expr::<serde_v8::AnyValue>(GetEvalDenoRuntime, "1+1")
            .await
            .unwrap();
        assert!(matches!(val, serde_v8::AnyValue::Number(2f64)));
    }
}
