use crate::{Behavior, BehaviorTree, Node, Status, Tree};
use std::collections::VecDeque;

pub struct Sequence {
    pub children: Vec<u16>,
    pub current_child: u16,
}

impl Sequence {
    //pub fn add_condition(&mut self, condition: Node) {
    //  self.children.insert(0, condition);
    //}
}

impl Behavior for Sequence {
    fn initialize(&mut self, bt: &mut BehaviorTree) -> Status {
        self.current_child = 0;
        if let Some(child) = self.children.get(0) {
            bt.events.push_back(child.clone());
            bt.node(child).borrow_mut().initialize(bt);
            Status::Running
        } else {
            Status::Invalid
        }
    }

    fn tick(&mut self, bt: &mut BehaviorTree) -> Status {
        let child_index = self.children.get(self.current_child as usize).unwrap();
        match bt.node(child_index).borrow().status {
            Status::Success => {
                self.current_child += 1;
                if let Some(next_child) = self.children.get(self.current_child as usize) {
                    bt.events.push_back(next_child.clone());
                    let next_rc = bt.node(next_child);
                    let mut node = next_rc.borrow_mut();
                    node.initialize(bt);
                    let u: u16 = 1;
                    Status::Running
                } else {
                    Status::Success
                }
            }
            result_status => result_status,
        }
    }

    fn abort(&mut self, bt: &mut BehaviorTree) -> Status {
        /*
        let mut status = Status::Aborted;
        if let Some(index) = self.children.get(self.current_child as usize) {
            let mut child = bt.node(index).borrow_mut();
            if child.status == Status::Running {
                status = child.abort(bt).clone()
            }
        }
        */
        Status::Aborted
    }
}
