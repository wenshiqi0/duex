mod state;
pub mod token;
pub mod node;

use std::{cell::RefCell, rc::Rc};

use crate::node::Node;

pub struct NodeBuilder;

impl NodeBuilder {
    pub fn init() -> Rc<RefCell<Node>> {
        let mut first_node = Node::new();
        first_node.set_state(&token::Token::Start, "");
        Rc::new(RefCell::new(first_node))
    }

    pub fn build() -> Rc<RefCell<Node>> {
        Rc::new(RefCell::new(Node::new()))
    }
}
