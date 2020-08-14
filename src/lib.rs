use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;
//pub mod helpers;
//pub mod nodes;

pub struct Action {
    status: Status,
    update: Box<dyn FnMut() -> Status>,
}

impl Action {
    fn new(update: Box<dyn FnMut() -> Status>) -> Box<Self> {
        Box::new(Self {
            update,
            status: Status::Invalid,
        })
    }
}

impl Behavior for Action {
    fn status(&self) -> Status {
        self.status
    }

    fn index(&self) -> i16 {
        0
    }

    fn update(&mut self, _: &mut VecDeque<Event>) -> Status {
        self.status = (self.update)();
        self.status.clone()
    }
}

pub type Event = (i16, Option<i16>);

pub struct Sequence {
    children: Vec<i16>,
    current_child: i16,
    status: Status,
    index: i16,
}

impl Behavior for Sequence {
    fn initialize(&mut self, events: &mut VecDeque<Event>) {
        if let Some(child) = self.children.get(0) {
            events.push_back((child.clone(), Some(self.index)));
            self.status = Status::Running
        } else {
            self.status = Status::Failure
        }
    }

    fn child_complete(&mut self, result: Status, events: &mut VecDeque<Event>) {
        match result {
            Status::Success => {
                self.current_child += 1;
                if let Some(child) = self.children.get(self.current_child as usize) {
                    events.push_back((child.clone(), Some(self.index)));
                    Status::Running
                } else {
                    Status::Success
                }
            }
            Status::Failure => Status::Failure,
            _ => panic!("Invalid result: {:?}", &result),
        };
    }

    fn status(&self) -> Status {
        self.status.clone()
    }

    fn index(&self) -> i16 {
        self.index
    }
}

pub struct Tree {
    events: VecDeque<Event>,
    nodes: Vec<Box<dyn Behavior>>,
}

impl Tree {
    pub fn new(mut nodes: Vec<Box<dyn Behavior>>) -> Self {
        nodes.shrink_to_fit();
        Self {
            events: VecDeque::<Event>::new(),
            nodes,
        }
    }
    pub fn step(&mut self) -> bool {
        if let Some((node_key, opt_parent_key)) = self.events.pop_front() {
            let node = &mut self.nodes[node_key as usize];
            if node.status() == Status::Aborted {
                return true;
            } else if node.status() != Status::Running {
                node.initialize(&mut self.events);
            }
            let status = node.update(&mut self.events);
            if status == Status::Failure || status == Status::Success {
                if let Some(parent_key) = opt_parent_key {
                    let parent = &mut self.nodes[parent_key as usize];
                    parent.child_complete(status, &mut self.events);
                }
            } else if status == Status::Running {
                self.events.push_back((node_key, opt_parent_key));
            }
            true
        } else {
            false
        }
    }
}

pub struct TreeBuilder {
    nodes: Vec<Box<dyn Behavior>>,
}

impl TreeBuilder {
    pub fn new() -> Self {
        Self { nodes: vec![] }
    }

    pub fn builder(self) -> Tree {
        Tree::new(self.nodes)
    }

    pub fn action(&mut self, behavior: Box<dyn Behavior>) -> i16 {
        let index = self.nodes.len();
        self.nodes.push(behavior);
        index as i16
    }

    pub fn sequence(&mut self, children: Vec<i16>) -> i16 {
        let index = self.nodes.len() as i16;
        self.nodes.push(Box::new(Sequence {
            children: children,
            index,
            current_child: 0,
            status: Status::Invalid,
        }));
        index
    }
}

pub trait Behavior {
    fn initialize(&mut self, events: &mut VecDeque<Event>) {}

    fn update(&mut self, events: &mut VecDeque<Event>) -> Status {
        self.status()
    }

    fn status(&self) -> Status;

    fn index(&self) -> i16;

    fn child_complete(&mut self, result: Status, events: &mut VecDeque<Event>) {}

    fn abort(&mut self) -> Status {
        Status::Aborted
    }
}

#[derive(PartialEq, Copy, Clone, Debug)]
pub enum Status {
    Invalid,
    Delegated,
    Running,
    Success,
    Failure,
    Aborted,
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test() {
        let mut builder = TreeBuilder::new();
        let root = builder.sequence(vec![
            builder.action(Action::new(Box::new(|| Status::Success)))
        ]);
    }
}
