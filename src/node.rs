use std::{pin::Pin};

pub type PinnedNode<T> = Pin<Box<Node<T>>>;

pub struct Node<T> {
    data: T,
    len: usize,
    height: usize,
    // parent: Option<(bool, NonNull<Node<T>>)>,
    children: [Option<PinnedNode<T>>; 2]
}

impl<T> Node<T> {
    pub fn new(data: T) -> Self {
        Node {
            data,
            len: 1,
            height: 1,
            // parent: None,
            children: [None, None]
        }
    }

    pub fn from_iter<I: IntoIterator<Item = T>>(data: I) -> Option<Pin<Box<Self>>> {
        let mut node = None;
        for element in data {
            node = Node::merge(node, Some(Node::pin(element)));
        }
        node
    }

    pub fn pin(data: T) -> Pin<Box<Self>> {
        Box::pin(Self::new(data))
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn data(&self) -> &T {
        &self.data
    }

    pub fn data_mut(&mut self) -> &mut T {
        &mut self.data
    }

    pub fn into_data(self) -> T {
        self.data
    }

    pub fn child(&self, dir: bool) -> Option<&Node<T>> {
        if let Some(child) = &self.children[dir as usize] {
            Some(child.as_ref().get_ref())
        } else {
            None
        }
    }

    pub fn leftmost<P: Fn(&T) -> bool>(&self, predicate: P) -> Option<usize> {
        let mut left_len = 0;
        let result = predicate(&self.data);
        if let Some(left) = &self.children[0] {
            left_len = left.len();
            if result {
                return left.leftmost(predicate).or(Some(left_len));
            }
        }
        if result {
            return Some(left_len);
        }
        if let Some(right) = &self.children[1] {
            return Some(left_len + 1 + right.leftmost(predicate)?);
        }
        None
    }

    pub fn at(&self, index: usize) -> &Self {
        let mut left_len = 0;
        if let Some(left) = &self.children[0] {
            left_len = left.len();
            if index < left_len {
                return left.at(index);
            }
        }
        if index == left_len {
            return self;
        }
        self.children[1].as_ref().unwrap().at(index - left_len - 1)
    }

    pub fn at_mut(&mut self, index: usize) -> &mut Self {
        let left_len = self.children[0].as_ref().map(|node| node.len()).unwrap_or(0);
        if index < left_len {
            unsafe { self.children[0].as_mut().unwrap().as_mut().get_unchecked_mut() }.at_mut(index)
        } else if index == left_len {
            self
        } else {
            unsafe { self.children[1].as_mut().unwrap().as_mut().get_unchecked_mut() }.at_mut(index - left_len - 1)
        }
    }

    // pub fn parent(&self) -> Option<&Self> {
    //     if let Some((_, parent)) = &self.parent {
    //         Some(unsafe { parent.as_ref() })
    //     } else {
    //         None
    //     }
    // }

    pub fn rotate(mut self: Pin<Box<Self>>, dir: bool) -> Pin<Box<Self>> {
        let self_mut = unsafe { self.as_mut().get_unchecked_mut() };
        if let Some(mut child) = self_mut.children[!dir as usize].take() {
            let child_mut = unsafe { child.as_mut().get_unchecked_mut() };
            self_mut.children[!dir as usize] = child_mut.children[dir as usize].take();
            self_mut.update();
            child_mut.children[dir as usize] = Some(self);
            child_mut.update();
            child
        } else {
            self
        }
    }

    pub fn balance(self: Pin<Box<Self>>) -> Pin<Box<Self>> {
        let lh = self.children[0].as_ref().map(|child| child.height).unwrap_or(0);
        let rh = self.children[1].as_ref().map(|child| child.height).unwrap_or(0);
        if lh + 2 < rh {
            self.rotate(false)
        } else if rh + 2 < lh {
            self.rotate(true)
        } else {
            self
        }
    }

    pub fn update(&mut self) {
        let mut len = 1;
        let mut height = 1;
        for child in &self.children {
            if let Some(child) = child {
                len += child.len;
                height = height.max(child.height + 1);
            }
        }
        self.len = len;
        self.height = height;
    }

    pub fn merge(left: Option<Pin<Box<Self>>>, right: Option<Pin<Box<Self>>>) -> Option<Pin<Box<Self>>> {
        if let Some(left) = left {
            if let Some(right) = right {
                Some(left.append(right))
            } else {
                Some(left)
            }
        } else {
            right
        }
    }

    pub fn append(mut self: Pin<Box<Self>>, other: Pin<Box<Self>>) -> Pin<Box<Self>> {
        let self_mut = unsafe { self.as_mut().get_unchecked_mut() };
        let right = &mut self_mut.children[1];
        *right = Some(if let Some(child) = right.take() {
            child.append(other)
        } else {
            other
        });
        self_mut.update();
        self.balance()
    }

    pub fn split_at(mut self: Pin<Box<Self>>, at: usize) -> (Option<Pin<Box<Self>>>, Option<Pin<Box<Self>>>) {
        assert!(at <= self.len);
        if at == 0 {
            (None, Some(self))
        } else if at == self.len {
            (Some(self), None)
        } else {
            let mut left_len = 0;
            if let Some(left_child) = &self.children[0] {
                left_len = left_child.len;
                let self_mut = unsafe { self.as_mut().get_unchecked_mut() };
                if at <= left_len {
                    let (left, right) = self_mut.children[0].take().unwrap().split_at(at);
                    self_mut.children[0] = right;
                    self_mut.update();
                    return (left, Some(self.balance()));
                }
            }
            let self_mut = unsafe { self.as_mut().get_unchecked_mut() };
            let (left, right) = self_mut.children[1].take().unwrap().split_at(at - left_len - 1);
            self_mut.children[1] = left;
            self_mut.update();
            (Some(self.balance()), right)
        }
    }
}

// impl<T> Drop for Node<T> {
//     fn drop(&mut self) {
//         if let Some((dir, mut parent)) = self.parent {
//             unsafe { parent.as_mut() }.children[dir as usize] = None;
//         }
//     }
// }
