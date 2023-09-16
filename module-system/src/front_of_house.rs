// We can nest modules inside one another
// A parent module cannot access a child module unless it is made public
pub mod hosting;

pub mod serving {
  fn take_order() {}
  fn serve_order() {}
  fn take_payment() {}
}