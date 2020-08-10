use crate::nodes::{on_parallel_child_complete, Behavior, ChildrenNodes, Node, Status};
use crate::BehaviorTree;
#[derive(PartialEq)]
pub enum ParallelPolicy {
    One,
    OneDelayed,
    All,
}

pub struct Parallel {
    pub children: ChildrenNodes,
    pub status: Status,
    pub success_policy: ParallelPolicy,
    pub failure_policy: ParallelPolicy,
    pub success_count: usize,
    pub failure_count: usize,
}

impl Behavior for Parallel {
    fn status(&self) -> &Status {
        &self.status
    }

    fn tick(&mut self) -> &Status {
        &self.status
    }

    fn initialize(&mut self, bt: &mut BehaviorTree, rc: Node) {
        self.children.reset();
        self.success_count = 0;
        self.failure_count = 0;
        if let Some(child) = self.children.next() {
            bt.events.push_back((child.clone(), Some(rc.clone())));
            child.borrow_mut().initialize(bt, rc);
            self.status = Status::Running;
        } else {
            self.status = Status::Failure;
        }
    }

    fn on_child_complete(&mut self, status: &Status, bt: &mut BehaviorTree, rc: Node) {
        on_parallel_child_complete(self, status, bt, rc);
        if self.status != Status::Running {
            self.terminate()
        }
    }

    fn terminate(&mut self) {}

    fn abort(&mut self) {
        for node_rc in self.children.nodes.iter() {
            let mut node = node_rc.borrow_mut();
            if node.status() == &Status::Running {
                node.abort();
            }
        }
        self.status = Status::Aborted
    }
}
