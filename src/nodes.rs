use crate::{Behavior, BehaviorTree, Node, Status};
pub mod action;
//pub mod active_selector;
pub mod decorator;
//pub mod monitor;
//pub mod parallel;
pub mod selector;
pub mod sequence;

pub use action::*;
//pub use active_selector::*;
pub use decorator::*;
//pub use monitor::*;
//pub use parallel::*;
pub use selector::*;
pub use sequence::*;
pub struct ChildrenNodes {
    nodes: Vec<Node>,
    next_child: usize,
}

impl ChildrenNodes {
    pub fn new(nodes: Vec<Node>) -> Self {
        Self {
            nodes,
            next_child: 0,
        }
    }

    pub fn next(&mut self) -> Option<&Node> {
        let node = self.nodes.get(self.next_child);
        self.next_child += 1;
        node
    }

    pub fn get(&mut self) -> Option<&Node> {
        self.nodes.get(self.next_child)
    }

    pub fn reset(&mut self) {
        self.next_child = 0
    }
}
/*
pub fn on_parallel_child_complete(
    parent: &mut Parallel,
    status: &Status,
    bt: &mut BehaviorTree,
    parent_rc: Node,
) {
    match status {
        &Status::Success => parent.success_count += 1,
        _ => parent.failure_count += 1,
    }
    if parent.failure_policy == ParallelPolicy::One && parent.failure_count > 0 {
        parent.status = Status::Failure;
    } else if parent.success_policy == ParallelPolicy::One && parent.success_count > 0 {
        parent.status = Status::Success
    } else if let Some(child) = parent.children.next() {
        bt.events
            .push_back((child.clone(), Some(parent_rc.clone())));
        child.borrow_mut().initialize(bt, parent_rc);
    } else {
        let len = parent.children.nodes.len();
        if (parent.failure_policy == ParallelPolicy::OneDelayed && parent.failure_count > 0)
            || (parent.failure_policy == ParallelPolicy::All && parent.failure_count == len)
        {
            parent.status = Status::Failure;
        } else if (parent.success_policy == ParallelPolicy::OneDelayed && parent.success_count > 0)
            || (parent.success_policy == ParallelPolicy::All && parent.success_count == len)
        {
            parent.status = Status::Success;
        }
    }
}
*/
