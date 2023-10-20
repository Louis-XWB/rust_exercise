pub fn test_tuple() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (_x, _y, _z) = tup; //destructuring
    println!("The value of y is: {}", _y);
    println!("The value of y is: {}", tup.1); //accessing tuple element
}
