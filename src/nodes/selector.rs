use crate::nodes::{Behavior, ChildrenNodes, Node, Status};
use crate::BehaviorTree;

pub struct Selector {
    pub children: ChildrenNodes,
    pub status: Status,
}

impl Selector {}

impl Behavior for Selector {
    fn initialize(&mut self, bt: &mut BehaviorTree, rc: Node) {
        self.children.reset();
        if let Some(child) = self.children.next() {
            bt.events.push_back((child.clone(), Some(rc.clone())));
            child.borrow_mut().initialize(bt, rc);
            self.status = Status::Running;
        } else {
            self.status = Status::Failure;
        }
    }

    fn tick(&mut self) -> &Status {
        &self.status
    }

    fn status(&self) -> &Status {
        &self.status
    }

    fn abort(&mut self) {
        if let Some(child_rc) = self.children.get() {
            let mut child = child_rc.borrow_mut();
            if child.status() == &Status::Running {
                child.abort();
            }
        }
        self.status = Status::Aborted;
    }

    fn on_child_complete(&mut self, result: &Status, bt: &mut BehaviorTree, rc: Node) {
        if result != &Status::Success {
            if let Some(child) = self.children.next() {
                bt.events.push_back((child.clone(), Some(rc.clone())));
                child.borrow_mut().initialize(bt, rc);
            } else {
                self.status = result.clone();
            }
        } else {
            self.status = Status::Success;
        }
    }
}
