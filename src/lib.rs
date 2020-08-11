use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;
pub mod helpers;
pub mod nodes;

//pub type Node = Rc<RefCell<NodeWrapper>>;
pub type Node = NodeWrapper;

pub trait Behavior {
    fn initialize(&mut self, bt: &mut BehaviorTree) -> Status;

    fn tick(&mut self, bt: &mut BehaviorTree) -> Status;

    fn terminate(&mut self) {}

    fn abort(&mut self, _: &mut BehaviorTree) -> Status {
        self.terminate();
        Status::Aborted
    }

    fn on_child_complete(&mut self, status: &Status, bt: &mut BehaviorTree) -> Status {
        status.clone()
    }
}

pub struct NodeWrapper {
    status: Status,
    behavior: Box<dyn Behavior>,
    parent_index: Option<u16>,
}

impl NodeWrapper {
    pub fn new(behavior: Box<dyn Behavior>, parent_index: Option<u16>) -> Self {
        Self {
            behavior,
            status: Status::Invalid,
            parent_index,
        }
    }
}

impl NodeWrapper {
    fn initialize(&mut self, bt: &mut BehaviorTree) {
        self.status = self.behavior.initialize(bt)
    }

    fn tick(&mut self, bt: &mut BehaviorTree) -> &Status {
        self.status = self.behavior.tick(bt);
        if self.status == Status::Success || self.status == Status::Failure {
            if let Some(index) = self.parent_index {
                let mut parent = &bt.nodes[index as usize];
                parent.on_child_complete(&self.status, bt);
            }
        }
        &self.status
    }

    fn abort(&mut self, bt: &mut BehaviorTree) -> &Status {
        self.status = self.behavior.abort(bt);
        &self.status
    }

    fn on_child_complete(&mut self, status: &Status, bt: &mut BehaviorTree) {
        self.status = self.behavior.on_child_complete(status, bt)
    }
}

pub struct TreeBuilder {
    next_index: u16,
    nodes: Vec<Node>,
}

impl TreeBuilder {
    pub fn new() -> Self {
        Self {
            next_index: 0,
            nodes: vec![],
        }
    }

    pub fn builder(mut self) -> BehaviorTree {
        self.nodes.shrink_to_fit();
        BehaviorTree::new(self.nodes)
    }

    pub fn action(mut self, behavior: Box<dyn Behavior>) -> Self {
        self.nodes.push(NodeWrapper::new(behavior, None));
        self.next_index += 1;
        self
    }

    pub fn sequence(mut self, children: Vec<u16>) -> Self {
        for child_rc in children.iter() {
            self.nodes[child_rc.clone() as usize].parent_index = Some(self.next_index);
        }
        self.nodes.push(NodeWrapper::new(
            Box::new(nodes::Sequence {
                children: children,
                current_child: 0,
            }),
            None,
        ));
        self.next_index += 1;
        self
    }
}

pub struct BehaviorTree {
    events: VecDeque<u16>,
    nodes: Vec<Node>,
}

impl BehaviorTree {
    pub fn new(nodes: Vec<Node>) -> Self {
        Self {
            events: VecDeque::<u16>::new(),
            nodes,
        }
    }

    pub fn start(&mut self) {
        self.events.clear();
        let mut root = self.nodes.last().unwrap().clone();
        root.initialize(self);
        self.events.push_back(self.nodes.len() as u16);
    }

    pub fn run(&mut self) -> Status {
        self.start();
        while self.step() {}
        self.nodes.last().unwrap().status
    }

    pub fn step(&mut self) -> bool {
        if let Some(index) = self.events.pop_front() {
            let mut node = self.nodes.get(index as usize).unwrap().clone();
            if node.status == Status::Aborted {
                return true;
            }
            let status = node.tick(self);
            if status == &Status::Running {
                //drop(node);
                self.events.push_back(index);
            }
            true
        } else {
            false
        }
    }
}

#[derive(PartialEq, Copy, Clone, Debug)]
pub enum Status {
    Invalid,
    Running,
    Success,
    Failure,
    Aborted,
}

#[cfg(test)]
mod tests {}
