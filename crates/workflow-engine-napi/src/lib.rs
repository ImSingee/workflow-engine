#![deny(clippy::all)]

use anyhow::{anyhow, Result};
use napi::bindgen_prelude::*;
use napi_derive::napi;
use workflow_engine_core;

#[napi]
pub fn plus_100(input: u32) -> u32 {
    workflow_engine_core::add(100, input)
}

#[napi]
pub async fn async_plus_100(input: BigInt) -> Result<u64> {
    let (_, val, success) = input.get_u64();

    if !success {
        return Err(anyhow!("cannot convert input value to u64"));
    }

    Ok(workflow_engine_core::async_add(100, val).await)
}
