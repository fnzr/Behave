use crate::nodes::{Behavior, ChildrenNodes, Node, Status};
use crate::{enqueue_node, BehaviorTree};

pub struct Selector {
    children: ChildrenNodes,
    status: Status,
}

impl Selector {}

impl Behavior for Selector {
    fn initialize(&mut self, bt: &mut BehaviorTree, self_rc: Node) {
        self.children.reset();
        if let Some(child) = self.children.next() {
            enqueue_node(bt, child, Some(self_rc));
            self.status = Status::Running;
        } else {
            self.status = Status::Invalid;
        }
    }

    fn tick(&mut self) -> &Status {
        &self.status
    }

    fn status(&self) -> &Status {
        &self.status
    }

    fn on_child_complete(&mut self, result: &Status, bt: &mut BehaviorTree, self_rc: Node) {
        if result != &Status::Success {
            if let Some(child) = self.children.next() {
                enqueue_node(bt, child, Some(self_rc));
            } else {
                self.status = result.clone();
            }
        } else {
            self.status = Status::Success;
        }
    }
}
