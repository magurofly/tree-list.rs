use tree_list::*;

fn main() {
  let mut list = [1, 3, 5].iter().copied().collect::<TreeList<_>>();
  list.insert_sorted(2);
  list.insert_sorted(3);
  list.insert_sorted(4);
  println!("{:?}", list);
}