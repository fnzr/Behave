use crate::{Behavior, BehaviorTree, Node, Status};
pub struct Action {
    status: Status,
    action: fn(),
}

impl Behavior for Action {
    fn initialize(&mut self, bt: &mut BehaviorTree, self_rc: Node) {
        self.status = Status::Running
    }

    fn status(&self) -> &Status {
        &self.status
    }

    fn tick(&mut self) -> &Status {
        (self.action)();
        &self.status
    }
}
