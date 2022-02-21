pub mod node;
pub mod iter;
// pub mod slice;

// pub type AVLTreeListSlice<'a, T> = slice::Slice<'a, T>;

use std::{ops::*, pin::Pin, iter::FromIterator, fmt::Debug};

use node::*;

pub struct TreeList<T> {
    root: Option<PinnedNode<T>>,
}

impl<T> TreeList<T> {
    pub fn new() -> Self {
        Self {
            root: None,
        }
    }

    pub fn raw(root: Option<PinnedNode<T>>) -> Self {
        Self {
            root,
        }
    }

    pub fn len(&self) -> usize {
        self.root.as_ref().map(|node| node.len()).unwrap_or(0)
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn clear(&mut self) {
        self.root = None;
    }

    /// Inserts element at the given index.
    /// # Panics
    /// Panics if index is out of bounds (i.e. greater than the length of the list).
    /// # Complexity
    /// O(log(len))
    pub fn insert(&mut self, index: usize, element: T) {
        let node = Node::pin(element);
        if let Some(root) = self.root.take() {
            let (left, right) = root.split_at(index);
            self.root = Node::merge(Node::merge(left, Some(node)), right);
        } else {
            self.root = Some(node);
        }
    }
    pub fn push_front(&mut self, element: T) {
        let node = Node::pin(element);
        self.root = Node::merge(Some(node), self.root.take());
    }
    pub fn push_back(&mut self, element: T) {
        let node = Node::pin(element);
        self.root = Node::merge(self.root.take(), Some(node));
    }

    /// Moves all the elements of `other` into `Self`, leaving `other` empty.
    /// # Complexity
    /// O(log(len + other.len()))
    pub fn append(&mut self, other: &mut Self) {
        self.root = Node::merge(self.root.take(), other.root.take());
    }

    /// Removes the element at the specified position in the list.
    /// Returns the element that was removed.
    /// # Complexity
    /// O(log(len))
    pub fn remove(&mut self, index: usize) -> Option<T> {
        let (left, right) = self.root.take()?.split_at(index);
        if let Some(right) = right {
            let (node, right) = right.split_at(1);
            self.root = Node::merge(left, right);
            return Some(unsafe { Pin::into_inner_unchecked(node?) }.into_data());
        } else {
            self.root = left;
            return None;
        }
    }
    pub fn pop_front(&mut self) -> Option<T> {
        let (left, right) = self.root.take()?.split_at(1);
        self.root = right;
        Some(unsafe { Pin::into_inner_unchecked(left?) }.into_data())
    }
    pub fn pop_back(&mut self) -> Option<T> {
        let root = self.root.take()?;
        let index = root.len() - 1;
        let (left, right) = root.split_at(index);
        self.root = left;
        Some(unsafe { Pin::into_inner_unchecked(right?) }.into_data())
    }

    /// Splits the list into two at the given index.
    /// Returns the second list.
    /// # Panics
    /// Panics if the index is out of bounds.
    /// # Complexity
    /// O(log(len))
    pub fn split_off(&mut self, at: usize) -> Self {
        assert!(at <= self.len());
        if let Some(root) = self.root.take() {
            let (left, right) = root.split_at(at);
            self.root = left;
            Self::raw(right)
        } else {
            Self::new()
        }
    }

    pub fn iter(&self) -> iter::Iter<'_, T> {
        iter::Iter::new(self.root.as_ref().map(|node| node.as_ref().get_ref()))
    }

    // pub fn splice<R, I>(&mut self, range: R, replace_with: I) -> Self
    // where
    //     R: RangeBounds<usize>,
    //     I: IntoIterator<Item = T>,
    // {
    //     use Bound::*;
    //     let l = match range.start_bound() {
    //         Included(&n) => n + 1,
    //         Excluded(&n) => n,
    //         Unbounded => 0,
    //     };
    //     let r = match range.end_bound() {
    //         Included(&n) => n + 1,
    //         Excluded(&n) => n,
    //         Unbounded => self.len(),
    //     };
    //     if let Some(root) = self.root.take() {
    //         let (left, right) = root.split_at(l);
    //         let (center, right) = right.split_at(r - l);

    //     } else {
    //         Self::new()
    //     }
    // }
}

impl<T> FromIterator<T> for TreeList<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        Self::raw(Node::from_iter(iter))
    }
}

impl<T> Index<usize> for TreeList<T> {
    type Output = T;

    fn index(&self, index: usize) -> &T {
        assert!(index < self.len());
        self.root.as_ref().unwrap().at(index).data()
    }
}

impl<T> IndexMut<usize> for TreeList<T> {
    fn index_mut(&mut self, index: usize) -> &mut T {
        assert!(index < self.len());
        unsafe { self.root.as_mut().unwrap().as_mut().get_unchecked_mut() }.at_mut(index).data_mut()
    }
}

impl<T: Debug> Debug for TreeList<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("TreeList {")?;
        let mut first = true;
        for element in self.iter() {
            if !first {
                f.write_str(", ")?;
            }
            first = false;
            f.write_fmt(format_args!("{:?}", element))?;
        }
        f.write_str("}")?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
