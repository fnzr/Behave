use crate::nodes::*;
use crate::{Behavior, Node, NodeWrapper, Status};
use std::cell::RefCell;
use std::rc::Rc;

pub fn action(node: Box<dyn Behavior>) -> Node {
    Rc::new(RefCell::new(NodeWrapper {
        behavior: node,
        status: Status::Invalid,
    }))
}

pub fn repeater(node: Node, repeat_for: u32) -> Node {
    Rc::new(RefCell::new(NodeWrapper {
        behavior: Box::new(Repeater {
            node,
            repeat_for,
            current_loop: 0,
        }),
        status: Status::Invalid,
    }))
}

pub fn pure_action(action: Box<dyn FnMut() -> Status>) -> Node {
    Rc::new(RefCell::new(NodeWrapper {
        behavior: Box::new(Action { action }),
        status: Status::Invalid,
    }))
}

pub fn sequence(nodes: Vec<Node>) -> Node {
    Rc::new(RefCell::new(NodeWrapper {
        behavior: Box::new(Sequence {
            children: nodes,
            current_child: 0,
        }),
        status: Status::Invalid,
    }))
}

pub fn selector(nodes: Vec<Node>) -> Node {
    Rc::new(RefCell::new(NodeWrapper {
        behavior: Box::new(Selector {
            children: nodes,
            current_child: 0,
        }),
        status: Status::Invalid,
    }))
}
/*
pub fn parallel(
    success_policy: nodes::ParallelPolicy,
    failure_policy: nodes::ParallelPolicy,
    nodes: Vec<Node>,
) -> Node {
    Rc::new(RefCell::new(nodes::Parallel {
        children: nodes::ChildrenNodes::new(nodes),
        status: Status::Invalid,
        success_policy,
        failure_policy,
        success_count: 0,
        failure_count: 0,
    }))
}
*/
