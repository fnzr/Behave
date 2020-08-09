use crate::nodes::ChildrenNodes;
use crate::{Behavior, BehaviorTree, Node, Status};

pub struct Sequence {
    pub children: ChildrenNodes,
    pub status: Status,
}

impl Sequence {
    pub fn add_condition(&mut self, condition: Node) {
        self.children.nodes.insert(0, condition);
    }
}

impl Behavior for Sequence {
    fn initialize(&mut self, bt: &mut BehaviorTree, self_rc: Node) {
        self.children.reset();
        if let Some(child) = self.children.next() {
            bt.events.push_back((child.clone(), Some(self_rc.clone())));
            child.borrow_mut().initialize(bt, self_rc);
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

    fn on_child_complete(&mut self, result: &Status, bt: &mut BehaviorTree, self_rc: Node) {
        if result == &Status::Success {
            if let Some(child) = self.children.next() {
                bt.events.push_back((child.clone(), Some(self_rc.clone())));
                child.borrow_mut().initialize(bt, self_rc);
            } else {
                self.status = Status::Success;
            }
        } else {
            self.status = result.clone();
        }
    }
}
