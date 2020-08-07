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

pub enum Node<T> {
    Branch(Rc<RefCell<dyn C<State = T>>>),
    Leaf(Rc<RefCell<dyn B<State = T>>>),
}
pub struct Action {}
pub struct Sequence<T> {
    children: Vec<Node<T>>,
    current_child: usize,
    status: Status,
}

impl<T> Sequence<T> {
    fn new(children: Vec<Node<T>>) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Sequence {
            children,
            current_child: 0,
            status: Status::Invalid,
        }))
    }
}

impl<T> C for Sequence<T> {
    type State = T;    

    fn initialize(
        &mut self,
        bt: &mut BehaviorTree<Self::State>,
        self_rc: Rc<RefCell<dyn C<State = Self::State>>>,
        state: Rc<RefCell<Self::State>>,
    ) {
        if let Some(child) = self.children.get(0) {
            enqueue_node(bt, child, self_rc, state);
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
        bt: &mut BehaviorTree<Self::State>,
        self_rc: Rc<RefCell<dyn C<State = Self::State>>>,
        state: Rc<RefCell<T>>,
    ) {        
        if result == &Status::Success {
            self.current_child += 1;
            if let Some(child) = self.children.get_mut(self.current_child) {
                enqueue_node(bt, child, self_rc, state);
            } else {
                self.status = Status::Success;
            }
        } else {
            self.status = result.clone();
        }
    }
}

pub trait B {
    type State;
    fn tick(&mut self, state: Rc<RefCell<Self::State>>) -> &Status;
    fn initialize(&mut self, state: Rc<RefCell<Self::State>>) {}
    fn terminate(&mut self, state: Rc<RefCell<Self::State>>) {}
}

pub trait C {
    type State;    

    fn initialize(
        &mut self,
        bt: &mut BehaviorTree<Self::State>,
        self_rc: Rc<RefCell<dyn C<State = Self::State>>>,
        state: Rc<RefCell<Self::State>>,
    );

    fn terminate(&mut self) {}

    fn on_child_complete(
        &mut self,
        status: &Status,
        bt: &mut BehaviorTree<Self::State>,
        self_rc: Rc<RefCell<dyn C<State = Self::State>>>,
        state: Rc<RefCell<Self::State>>,
    );

    fn status(&self) -> &Status;
}

fn enqueue_node<T>(
    bt: &mut BehaviorTree<T>,
    node: &Node<T>,
    parent_rc: Rc<RefCell<dyn C<State = T>>>,
    state: Rc<RefCell<T>>,
) {
    match node {
        Node::Branch(b) => {
            let mut tmp = b.borrow_mut();
            tmp.initialize(bt, b.clone(), state);
            bt.events.push_back((Node::Branch(b.clone()), parent_rc));
        }
        Node::Leaf(l) => {
            let mut tmp = l.borrow_mut();
            tmp.initialize(state);
            bt.events.push_back((Node::Leaf(l.clone()), parent_rc));
        }
    }
}

pub struct BehaviorTree<T> {
    state: Rc<RefCell<T>>,
    events: VecDeque<(Node<T>, Rc<RefCell<dyn C<State = T>>>)>,
    fake_root: Rc<RefCell<FakeRoot<T>>>,
}

pub struct FakeRoot<T> {
    status: Status,
    phantom: std::marker::PhantomData<T>,
}

impl<T> C for FakeRoot<T> {
    type State = T;

    fn initialize(
        &mut self,
        bt: &mut BehaviorTree<Self::State>,
        self_rc: Rc<RefCell<dyn C<State = Self::State>>>,
        state: Rc<RefCell<Self::State>>,
    ) {
    }

    fn on_child_complete(
        &mut self,
        status: &Status,
        bt: &mut BehaviorTree<Self::State>,
        self_rc: Rc<RefCell<dyn C<State = Self::State>>>,
        state: Rc<RefCell<Self::State>>,
    ) {
        //println!("Fake complete");
        self.status = status.clone();
    }

    fn status(&self) -> &Status {
        &self.status
    }
}

impl<T: 'static> BehaviorTree<T> {
    pub fn new(state: Rc<RefCell<T>>) -> Self {
        Self {
            state,
            events: VecDeque::new(),
            fake_root: Rc::new(RefCell::new(FakeRoot {
                status: Status::Invalid,
                phantom: std::marker::PhantomData,
            })),
        }
    }
    pub fn start(&mut self, root: &mut Node<T>) {
        enqueue_node(self, root, self.fake_root.clone(), self.state.clone());
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
                    (tmp.tick(self.state.clone()).clone(), Node::Leaf(l.clone()))
                }
            };
            if result == Status::Running {                
                self.events.push_back((new_node, parent));
            } else {
                parent.borrow_mut().on_child_complete(
                    &result,
                    self,
                    parent.clone(),
                    self.state.clone(),
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
    x: i32,
}

impl B for PrintEven {
    type State = u32;

    fn tick(&mut self, state: Rc<RefCell<Self::State>>) -> &Status {
        if *state.borrow() % 2 == 0 {
            println!("Even");
        }
        &Status::Success
    }
}

pub struct PrintOdd {
    x: i32,
}

impl B for PrintOdd {
    type State = u32;

    fn tick(&mut self, state: Rc<RefCell<Self::State>>) -> &Status {
        if *state.borrow() % 2 != 0 {
            println!("Odd");
        }
        &Status::Success
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]

    fn test() {
        let x = 1;
        //let state = Rc::new(RefCell::new(&x));
        let a = Node::Leaf(Rc::new(RefCell::new(PrintOdd { x: 1 })));
        let b = Node::Leaf(Rc::new(RefCell::new(PrintEven { x: 1 })));
        let children = vec![a, b];
        //let root = Rc::new(RefCell::new(Sequence::<&u32>::new(children)));
        let rf = Rc::new(RefCell::new(x));
        let mut bt = BehaviorTree::new(rf.clone());
        let mut root = Node::Branch(Sequence::new(children));
        bt.start(&mut root);
        loop {
            while bt.step() {}
            let mut m = rf.borrow_mut();
                *m += 1;
            //let y = rf.borrow();
            //println!("{:?}", y);
            bt.start(&mut root);
        }
    }
}
