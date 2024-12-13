pub type DenoRuntime = deno_core::JsRuntime;

pub type GetDenoRuntimeResult = std::result::Result<DenoRuntime, GetDenoRuntimeError>;
pub type GetDenoRuntimeError = anyhow::Error;

pub trait GetDenoRuntime: Send {
    fn get(self) -> GetDenoRuntimeResult;
}
