pub mod node;
pub mod iter;
pub mod helper;
pub mod list;
// pub mod slice;

pub use list::TreeList;
// pub type AVLTreeListSlice<'a, T> = slice::Slice<'a, T>;

#[cfg(test)]
mod tests {
    #[test]
    fn push() {
        let mut list = super::TreeList::new();
        for &x in &[6, 5, 4] {
            list.push_front(x);
        }
        for &x in &[7, 8, 9] {
            list.push_back(x);
        }
        for &x in &[3, 2, 1, 0] {
            list.push_front(x);
        }
        let mut res = vec![];
        for &x in list.iter() {
            res.push(x);
        }
        assert_eq!(res.len(), 10);
        for i in 0 .. 10 {
            assert_eq!(res[i], i);
        }
    }

    #[test]
    fn insert() {
        let mut list = super::TreeList::new();
        list.insert(0, 4);
        list.insert(0, 0);
        list.insert(1, 1);
        eprintln!("{:?}, len={}", list, list.len());
        list.insert(3, 6);
        list.insert(2, 2);
        list.insert(4, 5);
        list.insert(3, 3);
        assert_eq!(list.len(), 7);
        for (i, &x) in list.iter().enumerate() {
            assert_eq!(i, x);
        }
    }
}
