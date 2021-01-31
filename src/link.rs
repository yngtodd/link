use std::mem;
use pyo3::prelude::*;

#[pyclass]
pub struct List {
    head: Link,
}

enum Link {
    Empty,
    More(Box<Node>),
}

struct Node {
    elem: i32,
    next: Link,
}

#[pymethods]
impl List {
    #[new]
    pub fn new() -> Self {
        List { head: Link::Empty }
    }

    /// Push an element to the list
    pub fn push(&mut self, elem: i32) {
        let node = Box::new(Node {
            elem: elem,
            next: mem::replace(&mut self.head, Link::Empty),
        });

        self.head = Link::More(node);
    }

    /// Pop the final element in the linked list
    ///
    /// Returns `None` if the list is empty, otherwise
    /// returns the final node's element and set the list's 
    /// head to the next node.
    pub fn pop(&mut self) -> Option<i32> {
        match mem::replace(&mut self.head, Link::Empty) {
            Link::Empty => None,
            Link::More(node) => {
                self.head = node.next;
                Some(node.elem)
            }
        }
    }
}

impl Drop for List {
    fn drop(&mut self) {
        let mut cur_link = mem::replace(&mut self.head, Link::Empty);
        // If the link has a next node, replace that link with an empty  link,
        // and go to the next link. We will do this for all nodes, and the 
        // list will finally go out of scope here.
        while let Link::More(mut boxed_node) = cur_link {
            cur_link = mem::replace(&mut boxed_node.next, Link::Empty)
        }
    }
}