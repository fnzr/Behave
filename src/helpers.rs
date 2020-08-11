use crate::nodes::*;
use crate::{Behavior, Node, NodeWrapper, Status};
use std::cell::RefCell;
use std::rc::Rc;

/*
pub fn action(node: Box<dyn Behavior>) -> Node {
    Rc::new(RefCell::new(NodeWrapper::new(node)))
}

pub fn repeater(node: Node, repeat_for: u32) -> Node {
    Rc::new(RefCell::new(NodeWrapper::new(Box::new(Repeater {
        node,
        repeat_for,
        current_loop: 0,
    }))))
}

pub fn pure_action(action: Box<dyn FnMut() -> Status>) -> Node {
    Rc::new(RefCell::new(NodeWrapper::new(Box::new(Action { action }))))
}

pub fn sequence(nodes: Vec<Node>) -> Node {
    let seq = NodeWrapper::new(Box::new(Sequence {
        children: vec![],
        current_child: 0,
    }));

    let seq_clone = seq.clone();
}

pub fn selector(nodes: Vec<Node>) -> Node {
    Rc::new(RefCell::new(NodeWrapper::new(Box::new(Selector {
        children: nodes,
        current_child: 0,
    }))))
}
*/
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
