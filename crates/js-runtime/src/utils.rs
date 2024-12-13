use super::result::{Error, Result};
use futures::future::Either;
use std::future::Future;
use std::time::Duration;

pub fn with_timeout<T>(
    timeout_duration: Option<Duration>,
    fut: impl Future<Output = T>,
) -> impl Future<Output = Result<T>> {
    match timeout_duration {
        None => Either::Left(async { Ok(fut.await) }),
        Some(timeout_duration) => Either::Right(async move {
            tokio::time::timeout(timeout_duration, fut)
                .await
                .map_err(|_timeout_elapsed| Error::ExecuteError("timeout".to_string()))
        }),
    }
}
