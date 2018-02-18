use std::cell::RefCell;
use std::rc::{Rc, Weak};

#[derive(Debug)]
pub struct Node<'a> {
    pub name: &'a str,
    pub parents: RefCell<Vec<Weak<Node<'a>>>>,
    pub children: RefCell<Vec<Rc<Node<'a>>>>,
}

trait INode<'a> {
    fn add_child(&self, child: Rc<Node<'a>>);
}

impl<'a> Node<'a> {
    pub fn new(name: &str) -> Node {
        Node {
            name: name,
            parents: RefCell::new(vec![]),
            children: RefCell::new(vec![]),
        }
    }
}

impl<'a> INode<'a> for Rc<Node<'a>> {
    fn add_child(&self, child: Rc<Node<'a>>) {
        child.parents.borrow_mut().push(Rc::downgrade(&self));
        self.children.borrow_mut().push(child);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_works() {
        let node = Node::new("test");
        assert_eq!(node.name, "test");
        assert_eq!(node.parents.borrow().len(), 0);
        assert_eq!(node.children.borrow().len(), 0);
    }

    #[test]
    fn add_child_works() {
        let node = Rc::new(Node::new("test"));
        let child = Rc::new(Node::new("child"));
        node.add_child(Rc::clone(&child));
        assert_eq!(node.name, "test");
        assert_eq!(node.children.borrow().len(), 1);
        assert_eq!(node.children.borrow()[0].name, child.name);
        let children = node.children.borrow();
        assert_eq!(children[0].parents.borrow().len(), 1);
        assert_eq!(children[0].children.borrow().len(), 0);
        assert_eq!(node.parents.borrow().len(), 0);
    }
}
