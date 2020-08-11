use crate::nodes::{Behavior, Node, Status};
use crate::BehaviorTree;
/*
pub struct Selector {
    pub children: Vec<u16>,
    pub current_child: u16,
}

impl Selector {}

impl Behavior for Selector {
    fn initialize(&mut self, bt: &mut BehaviorTree) -> Status {
        self.current_child = 0;
        if let Some(child_index) = self.children.get(0) {
            bt.nodes[child_index.clone() as usize].initialize(bt);
            bt.events.push_back(child_index.clone());
            Status::Running
        } else {
            Status::Failure
        }
    }

    fn tick(&mut self, bt: &mut BehaviorTree) -> Status {
        let child_index = self.children.get(self.current_child as usize).unwrap();
        let child = &bt.nodes[child_index.clone() as usize];
        match child.status {
            Status::Success => Status::Success,
            result_status => {
                self.current_child += 1;
                if let Some(next_child_rc) = self.children.get(self.current_child as usize) {
                    let mut next_node = &bt.nodes[next_child_rc.clone() as usize];
                    next_node.initialize(bt);
                    bt.events.push_back(next_child_rc.clone());
                    Status::Running
                } else {
                    result_status.clone()
                }
            }
        }
    }

    fn abort(&mut self, bt: &mut BehaviorTree) -> Status {
        let mut status = Status::Aborted;
        if let Some(child_index) = self.children.get(self.current_child as usize) {
            let mut child = bt.nodes[child_index.clone() as usize];
            if child.status == Status::Running {
                status = child.abort(bt).clone();
            }
        }
        status
    }
}
*/
