use crate::{Behavior, BehaviorTree, Node, Status};
pub struct Action {
    pub status: Status,
    pub action: fn() -> Status,
}

impl Behavior for Action {
    fn initialize(&mut self, bt: &mut BehaviorTree, self_rc: Node) {
        self.status = Status::Running
    }

    fn status(&self) -> &Status {
        &self.status
    }

    fn tick(&mut self) -> &Status {
        self.status = (self.action)();
        &self.status
    }

    fn abort(&mut self) {
        self.status = Status::Aborted
    }
}
