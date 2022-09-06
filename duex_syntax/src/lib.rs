mod state;

use std::{
    borrow::BorrowMut,
    cell::RefCell,
    rc::{Rc, Weak},
};

use duex_parser::token::Token;
use state::SyntaxState;

#[derive(Clone)]
pub struct Node {
    parent: Option<Weak<RefCell<Node>>>,
    first_child: Option<Rc<RefCell<Node>>>,
    sibling: Option<Rc<RefCell<Node>>>,

    state: SyntaxState,
}

impl Node {
    pub fn new() -> Node {
        Node {
            parent: None,
            first_child: None,
            sibling: None,

            state: SyntaxState::new(),
        }
    }

    pub fn get_state(&self) -> SyntaxState {
        self.state.clone()
    }

    pub fn set_state(&mut self, token: &Token, scope_name: &str) {
        self.state.token = token.clone();
        self.state.scope_name = scope_name.to_owned();
    }

    pub fn get_first_child(&self) -> Option<Rc<RefCell<Node>>> {
        self.first_child.clone()
    }

    pub fn set_first_child(&mut self, child: &Rc<RefCell<Node>>) {
        *self.first_child.borrow_mut() = Some(Rc::clone(child));
    }

    pub fn get_sibling(&self) -> Option<Rc<RefCell<Node>>> {
        self.sibling.clone()
    }

    pub fn set_sibling(&mut self, sibling: &Rc<RefCell<Node>>) {
        *self.sibling.borrow_mut() = Some(Rc::clone(sibling));
    }

    pub fn get_parent(&self) -> Option<Weak<RefCell<Node>>> {
        self.parent.clone()
    }

    pub fn set_parent(&mut self, parent: &Rc<RefCell<Node>>) {
        *self.parent.borrow_mut() = Some(Rc::downgrade(parent));
    }
}

#[cfg(test)]
mod tests {
    use duex_parser::token::Token;

    use super::Node;
    use std::{cell::RefCell, rc::Rc};

    #[test]
    fn test_syntax_tree() {
        let root = Rc::new(RefCell::new(Node::new()));
        let child = Rc::new(RefCell::new(Node::new()));

        root.borrow_mut().set_first_child(&child);
        child.borrow_mut().set_parent(&root);

        let mut state = root
            .borrow()
            .get_first_child()
            .unwrap()
            .borrow()
            .get_state();

        println!("{}", state);

        child.borrow_mut().set_state(&Token::Keyword, "let");

        state = root
            .borrow()
            .get_first_child()
            .unwrap()
            .borrow()
            .get_state();

        println!("{}", state);
    }
}
