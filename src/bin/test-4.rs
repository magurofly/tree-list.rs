use tree_list::*;

fn main() {
  let mut list = "hello, world".chars().collect::<TreeList<_>>();
  println!("{:?}", list.splice(2..=3, Some('x')));
  println!("{:?}", list);
}