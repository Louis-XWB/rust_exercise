pub mod garden;

use garden::vegetables::Asparagus;
use rust_exercise::back_of_house;
use rust_exercise::front_of_house::hosting;

pub fn test_module() {
    let plant = Asparagus {};
    println!("plant: {:?}", plant);

    hosting::add_to_waitlist();
    back_of_house::fix_incorrect_order();
}
