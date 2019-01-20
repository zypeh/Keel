use toolshed::CopyCell;
use std::ops::Deref;
use std::fmt::{self, Debug};

/// `Node` is a specialized `Cell` that holds a reference to T instead of T.
/// `Node` has defined lifetime and implements `Defer<Target = T>` for convenience.
#[derive(Clone, Copy)]
pub struct Node<'ast, T: 'ast> {
    value: CopyCell<&'ast NodeValue<T>>
}

#[derive(Clone, Copy, PartialEq)]
pub struct NodeValue<T> {
    pub start: u32,
    pub end: u32,
    pub value: T,
}

impl<T> NodeValue<T> {
    #[inline]
    pub fn new(start: u32, end: u32, value: T) -> Self {
        NodeValue {
            start,
            end,
            value,
        }
    }
}

impl<'ast, T: 'ast> Node<'ast, T> {
    #[inline]
    pub fn new(ptr: &'ast NodeValue<T>) -> Self {
        Node {
            value: CopyCell::new(ptr)
        }
    }
    
    #[inline]
    pub fn set(&self, ptr: &'ast NodeValue<T>) {
        self.value.set(ptr)
    }

    #[inline]
    pub fn get(&mut self) -> &'ast NodeValue<T> {
        self.value.get()
    }
}

impl<'ast, T: 'ast> Deref for Node<'ast, T> {
    type Target = NodeValue<T>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        self.value.get()
    }
}

impl<'ast, T: 'ast + PartialEq> PartialEq for Node<'ast, T> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.deref().eq(other.deref())
    }
}

impl<'ast, T: 'ast + Debug> Debug for Node<'ast, T> {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        Debug::fmt(self.deref(), f)
    }
}

impl<T: Debug> Debug for NodeValue<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}:{}) ", self.start, self.end)?;

        Debug::fmt(&self.value, f)
    }
}

#[cfg(test)]
mod node {
    use super::*;

    #[test]
    fn node_test() {
        let foo = NodeValue::new(0, 0, "foo");
        let bar = NodeValue::new(0, 0, "bar");

        let foo_ptr = Node::new(&foo);
        let bar_ptr = foo_ptr.clone();

        assert_eq!(*foo_ptr, NodeValue::new(0, 0, "foo"));
        assert_eq!(*bar_ptr, NodeValue::new(0, 0, "foo"));

        bar_ptr.set(&bar);

        assert_eq!(*foo_ptr, NodeValue::new(0, 0, "foo"));
        assert_eq!(*bar_ptr, NodeValue::new(0, 0, "bar"));
    }
}
