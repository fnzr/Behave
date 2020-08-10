use crate::nodes::{Behavior, Node, Status};
use crate::BehaviorTree;

pub struct Selector {
    pub children: Vec<Node>,
    pub current_child: u16,
}

impl Selector {}

impl Behavior for Selector {
    fn initialize(&mut self, bt: &mut BehaviorTree) -> Status {
        self.current_child = 0;
        if let Some(child_rc) = self.children.get(0) {
            child_rc.borrow_mut().initialize(bt);
            bt.events.push_back(child_rc.clone());
            Status::Running
        } else {
            Status::Failure
        }
    }

    fn tick(&mut self, bt: &mut BehaviorTree) -> Status {
        let child_rc = self.children.get(self.current_child as usize).unwrap();
        let child = child_rc.borrow();
        match child.status {
            Status::Running => {
                bt.events.push_back(child_rc.clone());
                Status::Running
            }
            Status::Success => Status::Success,
            result_status => {
                self.current_child += 1;
                if let Some(next_child_rc) = self.children.get(self.current_child as usize) {
                    next_child_rc.borrow_mut().initialize(bt);
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
        if let Some(child_rc) = self.children.get(self.current_child as usize) {
            let mut child = child_rc.borrow_mut();
            if child.status == Status::Running {
                status = child.abort(bt).clone();
            }
        }
        status
    }
}
