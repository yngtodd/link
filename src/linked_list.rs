use pyo3::prelude::*;

#[pyclass]
pub struct List<T> {
    head: Link<T>,
}

type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
    elem: T,
    next: Link<T>,
}

#[pymethods]
impl<T> List<T> {
    #[new]
    pub fn new() -> Self {
        List { head: None }
    }

    /// Push an element to the list
    pub fn push(&mut self, elem: T) {
        let node = Box::new(Node {
            elem: elem,
            next: self.head.take(),
        });

        self.head = Some(node);
    }

    /// Pop the final element in the linked list
    ///
    /// Returns `None` if the list is empty, otherwise
    /// returns the final node's element and set the list's 
    /// head to the next node.
    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            node.elem
        })
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut cur_link = self.head.take();
        // If the link has a next node, replace that link with an empty  link,
        // and go to the next link. We will do this for all nodes, and the 
        // list will finally go out of scope here.
        while let Some(mut boxed_node) = cur_link {
            cur_link = boxed_node.next.take() 
        }
    }
}

#[pymodule]
fn link<T>(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_class::<List<T>>()?;
    Ok(())
}