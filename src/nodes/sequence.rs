use crate::{Behavior, BehaviorTree, Node, Status};

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
            let mut node = &bt.nodes[child.clone() as usize];
            node.initialize(bt);
            Status::Running
        } else {
            Status::Invalid
        }
    }

    fn tick(&mut self, bt: &mut BehaviorTree) -> Status {
        let child_index = self.children.get(self.current_child as usize).unwrap();
        let mut child = bt.nodes[child_index.clone() as usize];
        match child.status {
            Status::Success => {
                self.current_child += 1;
                if let Some(next_child) = self.children.get(self.current_child as usize) {
                    bt.events.push_back(next_child.clone());
                    let mut node = bt.nodes[next_child.clone() as usize];
                    node.initialize(bt);
                    Status::Running
                } else {
                    Status::Success
                }
            }
            result_status => result_status,
        }
    }

    fn abort(&mut self, bt: &mut BehaviorTree) -> Status {
        let mut status = Status::Aborted;
        if let Some(child_rc) = self.children.get(self.current_child as usize) {
            let mut child = &bt.nodes[child_rc.clone() as usize];
            if child.status == Status::Running {
                status = child.abort(bt).clone()
            }
        }
        status
    }
}
