use tree_list::*;

fn main() {
  let list = [1, 2, 3, 4, 5, 6, 7, 8, 9].iter().copied().collect::<TreeList<_>>();
  println!("{:?}", list);
}