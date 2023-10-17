pub mod hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}

mod serving {
  fn take_order() {}

  fn serve_order() {}

  fn take_payment() {}
}