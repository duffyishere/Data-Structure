use std::marker::PhantomData;
use std::ptr::NonNull;

#[derive(Debug)]
pub struct SimpleLinkedList<T> {
    first: Option<NonNull<Node<T>>>,
    last: Option<NonNull<Node<T>>>,
    size: usize,
    marker: PhantomData<T>,
}

struct Node<T> {
    next: Option<NonNull<Node<T>>>,
    prev: Option<NonNull<Node<T>>>,
    element: T,
}

impl<T> Node<T> {
    fn new(element: T) -> Self {
        Node {next: None, prev: None, element}
    }

    fn into_element(self: Box<Self>) -> T {
        self.element
    }
}

// private methods
impl <T> SimpleLinkedList<T> {
    fn push_front_node(&mut self, mut node: Box<Node<T>>) {
        unsafe {
            node.next = self.first;
            node.prev = None;
            let node = Some(Box::leak(node).into());

            match self.first {
                None => self.last = node,
                Some(first) =>  (*first.as_ptr()).prev = node,
            }

            self.first = node;
            self.size += 1;
        }
    }


    fn pop_front_node(&mut self) -> Option<Box<Node<T>>> {
        self.first.map(|node| unsafe {
            let node = Box::from_raw(node.as_ptr());
            self.first = node.next;

            match self.first {
                None => self.last = None,
                Some(first) => (*first.as_ptr()).prev = None,
            }

            self.size -= 1;
            node
        })
    }

    fn push_back_node(&mut self, mut node: Box<Node<T>>) {
        unsafe {
            node.next = None;
            node.prev = self.last;
            let node = Some(Box::leak(node).into());

            match self.last {
                None => self.first = node,
                Some(last) => (*last.as_ptr()).next = node,
            }

            self.last = node;
            self.size += 1 ;
        }
    }

    fn pop_back_node(&mut self) -> Option<Box<Node<T>>>{
        self.last.map(|node| unsafe {
            let node = Box::from_raw(node.as_ptr());
            self.last = node.prev;

            match self.last {
                None => self.first = None,
                Some(last) => (*last.as_ptr()).next = None,
            }

            self.size -= 1;
            node
        })
    }
}

impl<T> SimpleLinkedList<T> {
    pub const fn new() -> Self {
        SimpleLinkedList { first: None, last: None, size: 0, marker: PhantomData }
    }

    pub fn push_front(&mut self, element:T) {
        self.push_front_node(Box::new(Node::new(element)))
    }

    pub fn pop_front(&mut self) -> Option<T> {
        self.pop_front_node().map(Node::into_element)
    }

    pub fn push_back(&mut self, element: T) {
        self.push_back_node(Box::new(Node::new(element)))
    }

    pub fn pop_back(&mut self) -> Option<T> {
        self.pop_back_node().map(Node::into_element)
    }
}