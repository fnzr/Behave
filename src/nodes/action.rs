use crate::{Behavior, BehaviorTree, Status};
pub struct Action {
    pub action: Box<dyn FnMut() -> Status>,
}

impl Behavior for Action {
    fn initialize(&mut self, _bt: &mut BehaviorTree) -> Status {
        Status::Running
    }

    fn tick(&mut self, _bt: &mut BehaviorTree) -> Status {
        (self.action)()
    }

    fn abort(&mut self, _bt: &mut BehaviorTree) -> Status {
        Status::Aborted
    }
}
