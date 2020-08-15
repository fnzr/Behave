use crate::{Behavior, FnOnComplete, Node, Status};
use std::collections::VecDeque;

pub struct Selector {
    pub children: Vec<Node>,
    pub current_child: i16,
    pub status: Status,
    pub on_complete_cb: FnOnComplete,
}

impl Selector {
    pub fn new(on_complete_cb: FnOnComplete) -> Self {
        Self {
            children: vec![],
            current_child: 0,
            status: Status::Invalid,
            on_complete_cb,
        }
    }
}

impl Behavior for Selector {
    fn initialize(&mut self, events: &mut VecDeque<Node>) {
        self.current_child = 0;
        if let Some(child) = self.children.get(0) {
            self.status = Status::Running;
            events.push_back(child.clone());
            child.borrow_mut().initialize(events);
        } else {
            self.status = Status::Failure
        }
    }

    fn on_complete(&mut self, result: Status, events: &mut VecDeque<Node>) {
        self.status = result.clone();
        if let Some(cb) = &mut self.on_complete_cb {
            cb(result, events)
        }
    }

    fn child_complete(&mut self, result: Status, events: &mut VecDeque<Node>) {
        match result {
            Status::Success => {
                self.on_complete(result, events);
            }
            Status::Failure => {
                self.current_child += 1;
                if let Some(child) = self.children.get(self.current_child as usize) {
                    events.push_back(child.clone());
                    child.borrow_mut().initialize(events);
                } else {
                    self.on_complete(result, events);
                }
            }
            _ => panic!("Invalid result: {:?}", &result),
        };
    }

    fn status(&self) -> Status {
        self.status.clone()
    }
}
