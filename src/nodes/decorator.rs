/*
use crate::{Behavior, BehaviorTree, Node, Status};

pub struct Repeater {
    pub node: Node,
    pub repeat_for: u32,
    pub current_loop: u32,
}

impl Behavior for Repeater {
    fn initialize(&mut self, bt: &mut BehaviorTree) -> Status {
        self.current_loop = 0;
        bt.events.push_back(self.node.clone());
        self.node.borrow_mut().initialize(bt);
        Status::Running
    }

    fn tick(&mut self, bt: &mut BehaviorTree) -> Status {
        let mut node = self.node.borrow_mut();
        match node.status {
            Status::Running => {
                bt.events.push_back(self.node.clone());
                Status::Running
            }
            child_status => {
                self.current_loop += 1;
                if self.current_loop < self.repeat_for {
                    bt.events.push_back(self.node.clone());
                    node.initialize(bt);
                    Status::Running
                } else {
                    child_status
                }
            }
        }
    }

    fn abort(&mut self, bt: &mut BehaviorTree) -> Status {
        self.node.borrow_mut().abort(bt).clone()
    }
}
*/
