pub type DenoRuntime = deno_core::JsRuntime;

pub type GetDenoRuntimeResult = std::result::Result<DenoRuntime, GetDenoRuntimeError>;
pub type GetDenoRuntimeError = anyhow::Error;

pub trait GetDenoRuntime: Send {
    fn get(self) -> GetDenoRuntimeResult;
}

#[cfg(test)]
pub mod for_test {
    use super::{DenoRuntime, GetDenoRuntime, GetDenoRuntimeResult};
    use deno_core::RuntimeOptions;
    use deno_web::{BlobStore, TimersPermission};
    use std::sync::Arc;

    pub struct GetTestDenoRuntime;
    impl GetDenoRuntime for GetTestDenoRuntime {
        fn get(self) -> GetDenoRuntimeResult {
            DenoRuntime::try_new(RuntimeOptions {
                is_main: true,
                extensions: vec![
                    deno_console::deno_console::init_ops_and_esm(),
                    deno_webidl::deno_webidl::init_ops_and_esm(),
                    deno_url::deno_url::init_ops_and_esm(),
                    deno_web::deno_web::init_ops_and_esm::<AllowAllPermissions>(
                        Arc::new(BlobStore::default()),
                        None,
                    ),
                    test_runtime::init_ops_and_esm(),
                ],
                ..Default::default()
            })
        }
    }

    struct AllowAllPermissions {}
    impl TimersPermission for AllowAllPermissions {
        #[inline(always)]
        fn allow_hrtime(&mut self) -> bool {
            true
        }
    }

    deno_core::extension!(
        test_runtime,
        esm_entry_point = "ext:test_runtime/98_global_scope_for_test.js",
        esm = [
          dir "js",
          "98_global_scope_for_test.js",
        ],
    );
}
