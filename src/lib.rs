use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;
pub mod helpers;
pub mod nodes;

pub struct ActionNode {
    status: Status,
    tick: Box<dyn FnMut() -> Status>,
}

#[derive(Copy, Clone, Debug)]
pub enum NodeType {
    Sequence,
    Action,
}

pub type NodeKey = (NodeType, u16);

pub type EventEntry = (NodeKey, Option<NodeKey>);

pub type Node = Rc<RefCell<NodeWrapper>>;
//pub type Node = NodeWrapper;
//type Tree = (Vec<NodeWrapper>, VecDeque<u16>);

pub struct Seq {
    children: Vec<NodeKey>,
    current_child: u16,
    status: Status,
}

impl Seq {
    pub fn initialize(&mut self, events: &mut VecDeque<EventEntry>, self_key: NodeKey) {
        if let Some(child) = self.children.get(0) {
            events.push_back((child.clone(), Some(self_key)));
            self.status = Status::Running;
        } else {
            self.status = Status::Failure;
        }
    }

    pub fn on_child_complete(
        &mut self,
        result: Status,
        events: &mut VecDeque<EventEntry>,
        self_key: NodeKey,
    ) -> Status {
        match result {
            Status::Success => {
                self.current_child += 1;
                if let Some(child) = self.children.get(self.current_child as usize) {
                    events.push_back((child.clone(), Some(self_key)));
                    Status::Running
                } else {
                    Status::Success
                }
            }
            Status::Failure => Status::Failure,
            _ => panic!("Invalid result: {:?}", &result),
        }
    }
}

pub struct Tree {
    actions: Vec<ActionNode>,
    sequences: Vec<Seq>,
    events: VecDeque<EventEntry>,
    bt: BehaviorTree,
}

impl Tree {
    pub fn step(&mut self) {
        if let Some((node_key, opt_parent_key)) = self.events.pop_front() {
            let (node_type, node_index) = &node_key;
            let result = match node_type {
                NodeType::Sequence => {
                    let seq = self.sequences[node_index.clone() as usize];
                    if seq.status != Status::Running {
                        seq.initialize(&mut self.events, node_key)
                    }
                    seq.status
                }
                NodeType::Action => {
                    let act = self.actions[node_index.clone() as usize];
                    (act.tick)()
                }
            };
            if result == Status::Success || result == Status::Failure {
                if let Some(parent_key) = opt_parent_key {
                    let (parent_type, parent_index) = &parent_key;
                    match parent_type {
                        NodeType::Sequence => {
                            let seq = self.sequences[parent_index.clone() as usize];
                            seq.on_child_complete(result, &mut self.events, parent_key);
                        }
                        _ => panic!("Unsuported parent type: {:?}", &parent_type),
                    }
                }
            } else {
                self.events.push_back((node_key, opt_parent_key));
            }
        }
    }

    pub fn action(&mut self, action: ActionNode) -> NodeKey {
        let key = self.actions.len();
        self.actions.push(action);
        (NodeType::Action, key as u16)
    }

    pub fn sequence(&mut self, children: Vec<NodeKey>) -> NodeKey {
        let key = self.sequences.len();
        self.sequences.push(Seq {
            children,
            current_child: 0,
            status: Status::Invalid,
        });
        (NodeType::Sequence, key as u16)
    }
}

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
        //root.initialize(self);
        //self.events.push_back(self.nodes.len() as u16);
    }

    pub fn run(&mut self) -> Status {
        //self.start();
        //while self.step() {}
        //self.nodes.last().unwrap().status
        Status::Invalid
    }

    pub fn node(&self, index: &u16) -> Node {
        self.nodes[index.clone() as usize].clone()
    }

    pub fn step(&mut self, tree: &mut Tree) -> bool {
        if let Some(index) = self.events.pop_front() {
            let node_rc = self.nodes[index as usize].clone();
            let mut node = node_rc.borrow_mut();
            if node.status == Status::Invalid {
                node.initialize(self);
            }
            if node.status == Status::Aborted {
                return true;
            }
            let status = node.tick(self);
            if status == &Status::Running {
                //drop(node);
                tree.1.push_back(index);
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
    Delegated,
    Running,
    Success,
    Failure,
    Aborted,
}

#[cfg(test)]
mod tests {}
