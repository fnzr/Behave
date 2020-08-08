use crate::nodes::ChildrenNodes;
use crate::{enqueue_node, Behavior, BehaviorTree, Node, Status};

pub struct Sequence {
    children: ChildrenNodes,
    status: Status,
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
        if result == &Status::Success {
            if let Some(child) = self.children.next() {
                enqueue_node(bt, child, Some(self_rc));
            } else {
                self.status = Status::Success;
            }
        } else {
            self.status = result.clone();
        }
    }
}
