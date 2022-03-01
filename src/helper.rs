use super::node::*;

pub trait Helper: Sized {
  type Data;

  fn update(_node: &mut Node<Self>) {}
  fn push(_node: &mut Node<Self>) {}
}

pub struct ListHelper<T>(std::marker::PhantomData<T>);
impl<T> Helper for ListHelper<T> {
  type Data = T;

  fn update(_node: &mut Node<Self>) {

  }
}