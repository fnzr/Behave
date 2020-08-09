use crate::{Behavior, Node, Status};
pub mod action;
pub mod decorator;
pub mod parallel;
pub mod selector;
pub mod sequence;

pub use action::*;
pub use decorator::*;
pub use parallel::*;
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

pub fn is_terminated(status: &Status) -> bool {
    status != &Status::Running
}
