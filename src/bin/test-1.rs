use tree_list::*;

fn main() {
    let mut list = TreeList::new();
    list.push_back(1);
    list.push_back(2);
    list.push_back(3);
    list.push_back(4);
    list.push_front(0);
    println!("{:?}", list.len());
    println!("{:?}", list);
}