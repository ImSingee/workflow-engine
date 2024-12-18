use workflow_engine::Action;

pub enum Actions {
    Add(AddAction),
}

pub struct AddAction;

impl Action for AddAction {
    type Input = ();
    type Output = ();
    type Error = ();

    async fn execute(&self, input: Self::Input) -> Result<Self::Output, Self::Error> {
        todo!()
    }
}
