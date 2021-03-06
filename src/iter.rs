use super::*;

pub struct Iter<'a, T> {
    stack: Vec<&'a Node<T>>,
}
impl<'a, T> Iter<'a, T> {
    pub fn new(root: Option<&'a Node<T>>) -> Self {
        let mut this = Self { stack: vec![] };
        this.add(root);
        this
    }

    fn add(&mut self, mut node: Option<&'a Node<T>>) {
        while let Some(child) = node {
            self.stack.push(child);
            node = child.child(false);
        }
    }
}
impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        let node = self.stack.pop()?;
        let element = node.data();
        self.add(node.child(true));
        Some(element)
    }
}