pub trait Action {
    type Input;
    type Output;
    type Error;

    async fn execute(&self, input: Self::Input) -> Result<Self::Output, Self::Error>;
}
//
// pub(crate) struct ActionBridge<T>(T);
// pub(crate) trait ActionBridgeTrait: workflow_engine_core::Action {}
//
// impl<T: Action> workflow_engine_core::Action for ActionBridge<T> {
//     type Input = T::Input;
//     type Output = T::Output;
//     type Error = T::Error;
//
//     async fn execute(&self, input: Self::Input) -> Result<Self::Output, Self::Error> {
//         self.execute(input).await
//     }
// }
//
// impl<T: Action> ActionBridgeTrait for ActionBridge<T> {}
