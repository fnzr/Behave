use crate::nodes::*;
use crate::{Behavior, CustomBehavior, Node, NodeBuilder, Status};
use std::cell::RefCell;
use std::rc::Rc;

pub fn action<T>(update: T) -> Box<NodeBuilder>
where
    T: FnMut() -> Status + Copy + 'static,
{
    Box::new(move |on_complete| -> Node {
        Rc::new(RefCell::new(Action::new(Box::new(update), on_complete)))
    })
}

pub fn custom<B>(behavior: Rc<RefCell<B>>) -> Box<NodeBuilder>
where
    B: CustomBehavior + 'static,
{
    Box::new(move |on_complete| -> Node {
        behavior.borrow_mut().set_on_complete(on_complete);
        behavior
    })
}

pub fn sequence(children_builder: Vec<Box<NodeBuilder>>) -> Box<NodeBuilder> {
    Box::new(move |on_complete| -> Node {
        let sequence = Rc::new(RefCell::new(Sequence::new(on_complete)));
        let mut children = Vec::with_capacity(children_builder.len());
        for child_builder in children_builder.into_iter() {
            let seq = sequence.clone();
            children.push((child_builder)(Some(Box::new(move |status, events| {
                seq.borrow_mut().child_complete(status, events);
            }))));
        }
        sequence.borrow_mut().children = children;
        sequence
    })
}

pub fn selector(children_builder: Vec<Box<NodeBuilder>>) -> Box<NodeBuilder> {
    Box::new(move |on_complete| -> Node {
        let selector = Rc::new(RefCell::new(Selector::new(on_complete)));
        let mut children = Vec::with_capacity(children_builder.len());
        for child_builder in children_builder.into_iter() {
            let sel = selector.clone();
            children.push((child_builder)(Some(Box::new(move |status, events| {
                sel.borrow_mut().child_complete(status, events);
            }))));
        }
        selector.borrow_mut().children = children;
        selector
    })
}
