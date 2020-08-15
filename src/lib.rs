use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;
//pub mod helpers;
//pub mod nodes;

pub struct Action {
    status: Status,
    update: Box<dyn FnMut() -> Status>,
    on_complete_cb: FnOnComplete,
}

impl Action {
    fn new<T>(update: Box<T>, on_complete_cb: FnOnComplete) -> Self
    where
        T: FnMut() -> Status + Copy + 'static,
    {
        Self {
            update,
            on_complete_cb,
            status: Status::Invalid,
        }
    }
}

impl Behavior for Action {
    fn status(&self) -> Status {
        self.status
    }

    fn update(&mut self, _: &mut VecDeque<Node>) -> Status {
        self.status = (self.update)();
        self.status.clone()
    }

    fn on_complete(&mut self, result: Status, events: &mut VecDeque<Node>) {
        if let Some(cb) = &mut self.on_complete_cb {
            cb(result, events)
        }
    }
}

pub struct Sequence {
    children: Vec<Node>,
    current_child: i16,
    status: Status,
    on_complete_cb: FnOnComplete,
}

impl Sequence {
    pub fn new(on_complete_cb: FnOnComplete) -> Self {
        Self {
            children: vec![],
            current_child: 0,
            status: Status::Invalid,
            on_complete_cb,
        }
    }
}

impl Behavior for Sequence {
    fn initialize(&mut self, events: &mut VecDeque<Node>) {
        self.current_child = 0;
        if let Some(child) = self.children.get(0) {
            events.push_back(child.clone());
            child.borrow_mut().initialize(events);
            self.status = Status::Running
        } else {
            self.status = Status::Failure
        }
    }

    fn on_complete(&mut self, result: Status, events: &mut VecDeque<Node>) {
        if let Some(cb) = &mut self.on_complete_cb {
            cb(result, events)
        }
    }

    fn child_complete(&mut self, result: Status, events: &mut VecDeque<Node>) {
        match result {
            Status::Success => {
                self.current_child += 1;
                if let Some(child) = self.children.get(self.current_child as usize) {
                    events.push_back(child.clone());
                    child.borrow_mut().initialize(events);
                    self.status = Status::Running
                } else {
                    self.on_complete(result, events);
                    self.status = Status::Success
                }
            }
            Status::Failure => {
                self.on_complete(result, events);
                self.status = Status::Failure
            }
            _ => panic!("Invalid result: {:?}", &result),
        };
    }

    fn status(&self) -> Status {
        self.status.clone()
    }
}

pub struct Tree {
    events: VecDeque<Node>,
    root: Node,
}

impl Tree {
    pub fn new(mut tree_builder: Box<NodeBuilder>) -> Self {
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

type FnOnComplete = Option<Box<dyn FnMut(Status, &mut VecDeque<Node>) + 'static>>;
type Node = Rc<RefCell<dyn Behavior>>;
type NodeBuilder = dyn FnMut(FnOnComplete) -> Node;

pub fn action<T>(update: T) -> Box<NodeBuilder>
where
    T: FnMut() -> Status + Copy + 'static,
{
    Box::new(move |on_complete| -> Node {
        Rc::new(RefCell::new(Action::new(Box::new(update), on_complete)))
    })
}

pub fn sequence(mut children_builder: Vec<Box<NodeBuilder>>) -> Box<NodeBuilder> {
    Box::new(move |on_complete| -> Node {
        let sequence = Rc::new(RefCell::new(Sequence::new(on_complete)));
        let mut children = Vec::with_capacity(children_builder.len());
        for child_builder in children_builder.iter_mut() {
            let seq = sequence.clone();
            children.push((child_builder)(Some(Box::new(move |status, events| {
                seq.borrow_mut().child_complete(status, events);
            }))));
        }
        sequence.borrow_mut().children = children;
        sequence
    })
}

pub trait Behavior {
    fn initialize(&mut self, events: &mut VecDeque<Node>) {}

    fn update(&mut self, events: &mut VecDeque<Node>) -> Status {
        self.status()
    }

    fn status(&self) -> Status;

    fn child_complete(&mut self, result: Status, events: &mut VecDeque<Node>) {}

    fn on_complete(&mut self, result: Status, events: &mut VecDeque<Node>);

    fn abort(&mut self) -> Status {
        Status::Aborted
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
mod tests {

    use super::*;

    #[test]
    fn test() {
        let tree_builder = sequence(vec![action(|| Status::Success), action(|| Status::Failure)]);
        let mut tree = Tree::new(tree_builder);
        println!("{:?}", tree.run());
    }
}
