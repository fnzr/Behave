extern crate event_behavior_tree;

use event_behavior_tree::helpers::*;
use event_behavior_tree::*;

pub fn succeed() -> Node {
    pure_action(|| Status::Success)
}

pub fn fail() -> Node {
    pure_action(|| Status::Failure)
}

pub fn panic() -> Node {
    pure_action(|| {
        assert!(false);
        Status::Failure
    })
}
