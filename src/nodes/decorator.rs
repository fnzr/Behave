use crate::{Behavior, BehaviorTree, Node, Status};

pub struct Decorator {
    status: Status,
    child: Node,
    decoration: fn(&mut Decorator, Node) -> Status,
}

impl Behavior for Decorator {
    fn initialize(&mut self, bt: &mut BehaviorTree, self_rc: Node) {
        self.status = Status::Running
    }

    fn status(&self) -> &Status {
        &self.status
    }

    fn tick(&mut self) -> &Status {
        self.status = (self.decoration)(self, self.child);
        &self.status
    }
}
