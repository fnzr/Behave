use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

#[derive(PartialEq, Copy, Clone, Debug)]
pub enum Status {
    Invalid,
    Running,
    Success,
    Failure,
}

pub enum Node {
    Branch(Rc<RefCell<dyn C>>),
    Leaf(Rc<RefCell<dyn B>>),
}
pub struct Action {}
pub struct Sequence {
    children: Vec<Node>,
    current_child: usize,
    status: Status,
}

impl Sequence {
    fn new(children: Vec<Node>) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Sequence {
            children,
            current_child: 0,
            status: Status::Invalid,
        }))
    }
}

impl C for Sequence {
    fn initialize(
        &mut self,
        bt: &mut BehaviorTree,
        self_rc: Rc<RefCell<dyn C>>,
    ) {
        if let Some(child) = self.children.get(0) {
            enqueue_node(bt, child, self_rc);
            self.status = Status::Running;
            self.current_child = 0;
        } else {
            self.status = Status::Invalid;
        }
    }

    fn status(&self) -> &Status {        
        &self.status
    }

    fn on_child_complete(
        &mut self,
        result: &Status,
        bt: &mut BehaviorTree,
        self_rc: Rc<RefCell<dyn C>>,
    ) {        
        if result == &Status::Success {
            self.current_child += 1;
            if let Some(child) = self.children.get_mut(self.current_child) {
                enqueue_node(bt, child, self_rc);
            } else {
                self.status = Status::Success;
            }
        } else {
            self.status = result.clone();
        }
    }
}

pub trait B {
    fn tick(&mut self) -> &Status;
    fn initialize(&mut self) {}
    fn terminate(&mut self) {}
}

pub trait C {
    fn initialize(
        &mut self,
        bt: &mut BehaviorTree,
        self_rc: Rc<RefCell<dyn C>>,
    );

    fn terminate(&mut self) {}

    fn on_child_complete(
        &mut self,
        status: &Status,
        bt: &mut BehaviorTree,
        self_rc: Rc<RefCell<dyn C>>,
    );

    fn status(&self) -> &Status;
}

fn enqueue_node(
    bt: &mut BehaviorTree,
    node: &Node,
    parent_rc: Rc<RefCell<dyn C>>,
) {
    match node {
        Node::Branch(b) => {
            let mut tmp = b.borrow_mut();
            tmp.initialize(bt, b.clone());
            bt.events.push_back((Node::Branch(b.clone()), parent_rc));
        }
        Node::Leaf(l) => {
            let mut tmp = l.borrow_mut();
            tmp.initialize();
            bt.events.push_back((Node::Leaf(l.clone()), parent_rc));
        }
    }
}

pub struct BehaviorTree {
    events: VecDeque<(Node, Rc<RefCell<dyn C>>)>,
    fake_root: Rc<RefCell<FakeRoot>>,
}

pub struct FakeRoot {
    status: Status,
}

impl C for FakeRoot {
    fn initialize(
        &mut self,
        bt: &mut BehaviorTree,
        self_rc: Rc<RefCell<dyn C>>,
    ) {
    }

    fn on_child_complete(
        &mut self,
        status: &Status,
        bt: &mut BehaviorTree,
        self_rc: Rc<RefCell<dyn C>>,
    ) {
        //println!("Fake complete");
        self.status = status.clone();
    }

    fn status(&self) -> &Status {
        &self.status
    }
}

impl BehaviorTree {
    pub fn new() -> Self {
        Self {
            events: VecDeque::new(),
            fake_root: Rc::new(RefCell::new(FakeRoot {
                status: Status::Invalid,
            })),
        }
    }
    pub fn start(&mut self, root: &mut Node) {
        enqueue_node(self, root, self.fake_root.clone());
    }

    pub fn step(&mut self) -> bool {
        if let Some((node, parent)) = self.events.pop_front() {
            let (result, new_node) = match node {
                Node::Branch(b) => {
                    let tmp = b.borrow_mut();
                    
                    (tmp.status().clone(), Node::Branch(b.clone()))                    
                }
                Node::Leaf(l) => {
                    let mut tmp = l.borrow_mut();                    
                    (tmp.tick().clone(), Node::Leaf(l.clone()))
                }
            };
            if result == Status::Running {                
                self.events.push_back((new_node, parent));
            } else {
                parent.borrow_mut().on_child_complete(
                    &result,
                    self,
                    parent.clone()
                );
            }
            true
        }
        else {
            false
        }        
    }
}

pub struct PrintEven {
    x: Rc<RefCell<i32>>
}

impl B for PrintEven {
    fn tick(&mut self) -> &Status {
        if *self.x.borrow() % 2 == 0 {
            println!("Even");
        }
        else {
            println!("Skip");
        }
        &Status::Success
    }
}

pub struct PrintOdd {
    x: Rc<RefCell<i32>>
}

impl B for PrintOdd {
    fn tick(&mut self) -> &Status {
        if *self.x.borrow() % 2 != 0 {
            println!("Odd");
        }
        else {
            println!("Skip");
        }
        &Status::Success
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]

    fn test() {
        let x = Rc::new(RefCell::new(1));
        //let state = Rc::new(RefCell::new(&x));
        let a = Node::Leaf(Rc::new(RefCell::new(PrintOdd { x:x.clone() })));
        let b = Node::Leaf(Rc::new(RefCell::new(PrintEven { x:x.clone() })));
        let children = vec![a, b];
        //let root = Rc::new(RefCell::new(Sequence::<&u32>::new(children)));
        let mut bt = BehaviorTree::new();
        let mut root = Node::Branch(Sequence::new(children));
        bt.start(&mut root);
        loop {
            while bt.step() {                
            }            
            let c = x.clone();
                let mut v = c.borrow_mut();
                *v += 1;
            //let y = rf.borrow();
            //println!("{:?}", y);
            bt.start(&mut root);
        }
    }
}
