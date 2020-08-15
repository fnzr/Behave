use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;
pub mod helpers;
pub mod nodes;

pub type FnOnComplete = Option<Box<dyn FnMut(Status, &mut VecDeque<Node>) + 'static>>;
pub type Node = Rc<RefCell<dyn Behavior>>;
type NodeBuilder = dyn FnOnce(FnOnComplete) -> Node;
pub struct Tree {
    events: VecDeque<Node>,
    root: Node,
}

impl Tree {
    pub fn new(tree_builder: Box<NodeBuilder>) -> Self {
        Self {
            events: VecDeque::<Node>::new(),
            root: tree_builder(None),
        }
    }

    pub fn run(&mut self) -> Status {
        self.events.clear();
        self.events.push_back(self.root.clone());
        self.root.borrow_mut().initialize(&mut self.events);
        while self.step() {}
        self.root.borrow().status()
    }

    pub fn step(&mut self) -> bool {
        if let Some(node_rc) = self.events.pop_front() {
            let mut node = node_rc.borrow_mut();
            if node.status() == Status::Aborted {
                return true;
            }
            let status = node.update(&mut self.events);
            if status == Status::Failure || status == Status::Success {
                node.on_complete(status, &mut self.events);
            } else if status == Status::Running {
                drop(node);
                self.events.push_back(node_rc);
            }
            true
        } else {
            false
        }
    }
}

pub trait Behavior {
    fn initialize(&mut self, _: &mut VecDeque<Node>) {}

    fn update(&mut self, _: &mut VecDeque<Node>) -> Status {
        self.status()
    }

    fn status(&self) -> Status;

    fn child_complete(&mut self, _: Status, _: &mut VecDeque<Node>) {}

    fn on_complete(&mut self, result: Status, events: &mut VecDeque<Node>);

    fn abort(&mut self) -> Status {
        Status::Aborted
    }
}

pub trait CustomBehavior: Behavior {
    fn set_on_complete(&mut self, on_complete: FnOnComplete);
}

#[derive(PartialEq, Copy, Clone, Debug)]
pub enum Status {
    Invalid,
    Running,
    Success,
    Failure,
    Aborted,
}
