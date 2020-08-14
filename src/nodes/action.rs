use crate::{Behavior, BehaviorTree, Status, Tree};
use std::collections::VecDeque;
pub struct Action {
    pub action: Box<dyn FnMut() -> Status>,
}

impl Behavior for Action {
    fn initialize(&mut self, bt: &mut BehaviorTree) -> Status {
        Status::Running
    }

    fn tick(&mut self, bt: &mut BehaviorTree) -> Status {
        (self.action)()
    }

    fn abort(&mut self, _bt: &mut BehaviorTree) -> Status {
        Status::Aborted
    }
}
