
pub mod syntax;

use syntax::array;
use syntax::enums;
use syntax::function;
use syntax::map;
use syntax::references_borrowing;
use syntax::scalar;
use syntax::slice;
use syntax::string;
use syntax::structs;
use syntax::tuple;
use syntax::vector;

pub mod garden;
use garden::vegetables::Asparagus;
use rust_exercise::back_of_house;
use rust_exercise::front_of_house::hosting;

fn main() {
    //Syntax
    {
        scalar::test_scalar();
        tuple::test_tuple();
        array::test_array();
        references_borrowing::test_references_borrowing();
        slice::test_slice();
        structs::test_structs();
        enums::test_enums();
        function::test_fn();
        vector::test_vector();
        string::test_string();
        map::test_map();
    }

    //module
    {
        let plant = Asparagus {};
        println!("plant: {:?}", plant);
    }

    //libs
    {
        hosting::add_to_waitlist();
        back_of_house::fix_incorrect_order();
    }
}
