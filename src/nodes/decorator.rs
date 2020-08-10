use crate::{Behavior, BehaviorTree, Node, Status};

pub struct Decorator {
    pub status: Status,
    pub child: Node,
    pub decoration: fn(&mut Decorator, Node) -> Status,
}

impl Behavior for Decorator {
    fn initialize(&mut self, bt: &mut BehaviorTree, rc: Node) {
        self.status = Status::Running;
        self.child.borrow_mut().initialize(bt, self.child.clone());
    }

    fn status(&self) -> &Status {
        &self.status
    }

    fn tick(&mut self) -> &Status {
        self.status = (self.decoration)(self, self.child.clone());
        &self.status
    }

    fn abort(&mut self) {
        self.child.borrow_mut().abort();
        self.status = Status::Aborted;
    }
}
