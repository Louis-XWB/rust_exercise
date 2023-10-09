//struct
// struct Rectangle {
//     width: u32,
//     height: u32,
// }

// impl Rectangle {
//     fn area(&self) -> u32 {
//         self.width * self.height
//     }

//     fn inc_width(&mut self, delta: u32) {
//         self.width += delta;
//     }
// }

fn main() {
    // println! is a macro that prints text to the console
    //println!("Hello, world!");
    //print!("Test print!");

    //simple example
    // let mut x: i32 = 6; //mutable variable binding
    // print!("{x}"); //Macro for printing, like println! but without newline
    // while x != 1 {
    //     //No parenthesis around expression
    //     if x % 2 == 0 {
    //         x = x / 2;
    //     } else {
    //         x = 3 * x + 1;
    //     }
    //     print!(" -> {x}");
    // }

    // println!();

    //Scalar types
    // let x = 5; //immutable variable binding
    // let mut y = 5; //mutable variable binding
    // y = 6;
    // println!("x: {x}, y: {y}");

    //Integer types
    // let x: i8 = 5; //signed 8-bit integer
    // let x: u8 = 5; //unsigned 8-bit integer
    // let x: i16 = 5; //signed 16-bit integer
    // let x: u16 = 5; //unsigned 16-bit integer
    // let x: i32 = 5; //signed 32-bit integer
    // let x: u32 = 5; //unsigned 32-bit integer
    // let x: i64 = 5; //signed 64-bit integer
    // let x: u64 = 5; //unsigned 64-bit integer
    // let x: isize = 5; //signed pointer-sized integer
    // let x: usize = 5; //unsigned pointer-sized integer

    //Floating-point types
    // let x: f32 = 5.0; //32-bit floating-point number
    // let x: f64 = 5.0; //64-bit floating-point number

    //Numeric operations
    // let sum = 5 + 10; //addition
    // let difference = 95.5 - 4.3; //subtraction
    // let product = 4 * 30; //multiplication
    // let quotient = 56.7 / 32.2; //division
    // let remainder = 43 % 5; //remainder

    //Boolean type
    // let t = true;
    // let f: bool = false; //with explicit type annotation

    //Character type
    // let c = 'z';
    // let z = 'ℤ';
    // let _heart_eyed_cat = '😻';

    //Compound types
    // let mut arr: [i32; 6] = [10, 20, 30, 40, 50, 60];
    // arr[1] = 21;
    // println!("arr[1]: {}", arr[1]); //Array type
    // println!("arr: {:?}", arr); //Array type

    // //Tuple type
    // let tup: (i32, f64, u8) = (500, 6.4, 1);
    // let (_x, _y, _z) = tup; //destructuring
    // println!("The value of y is: {}", _y);
    // println!("The value of y is: {}", tup.1); //accessing tuple element

    //References and borrowing
    // let mut x: i32 = 10;
    // let ref_x = &mut x;
    // *ref_x = 20;
    // println!("x: {x}");
    // println!("ref_x: {ref_x}");
    // let mut x: i32 = 10;
    // let ref_x = &mut x;
    // *ref_x = 20;
    // // println!("x: {}", x);
    // println!("ref_x: {}", ref_x);

    //error: borrowed value does not live long enough
    // let ref_x: &i32;
    // {
    //     let xx: i32 = 10;
    //     ref_x = &xx;
    // }
    // println!("ref_x: {ref_x}");

    // let ref_x: &i32;
    // {
    //     let xx: i32 = 10;
    //     ref_x = &xx;
    //     println!("ref_x: {}", ref_x);
    // }

    // let xx: i32 = 10;
    // {
    //     let ref_x: &i32;
    //     ref_x = &xx;
    //     println!("ref_x: {}", ref_x);
    // }

    //Slices
    // let mut a: [i32; 6] = [10, 20, 30, 40, 50, 60];
    // println!("a: {a:?}");

    // a[3] = 33;
    // let s: &[i32] = &a[1..4];
    // //a[3] = 44;
    // println!("s: {s:?}");

    //String vs str
    //&str an immutable reference to a string slice.
    //String a mutable string buffer.
    // let s1 = "World";
    // println!("s1: {s1}");

    // let mut s2 = String::from("Hello ");
    // println!("s2: {s2}");
    // s2.push_str(s1);
    // println!("s2: {s2}");

    // let s3: &str = &s2[6..];
    // println!("s3: {s3}");

    // print_fizzbuzz_to(21);

    // let mut rect = Rectangle {
    //     width: 30,
    //     height: 50,
    // };

    // println!("rect area: {}", rect.area());
    // rect.inc_width(20);
    // println!("rect area: {}", rect.area());

    // println!("coin toss: {}", pick_one("heads", "tails"));
    // println!("cash prize: {}", pick_one(500, 1000));

    let x: i8 = 15;
    let y: i16 = 1000;

    println!("{x} * {y} = {}", multiply(x.into(), y));
}

//function Rustdoc
// fn is_divisible_by(lhs: u32, rhs: u32) -> bool {
//     if rhs == 0 {
//         return false;
//     } else {
//         return true;
//     }
// }

// fn is_divisible(n: u32, divisor: u32) -> bool {
//     if divisor == 0 {
//         return false;
//     }
//     n % divisor == 0
// }

// fn fizzbuzz(n: u32) -> String {
//     let fizz = if is_divisible(n, 3) { "fizz" } else { "" };
//     let buzz = if is_divisible(n, 5) { "buzz" } else { "" };
//     if fizz.is_empty() && buzz.is_empty() {
//         return format!("{n}");
//     }
//     format!("{fizz}{buzz}")
// }

// fn print_fizzbuzz_to(n: u32) {
//     for i in 1..=n {
//         println!("{}",fizzbuzz(n))
//     }
// }

// Each function has a single implementation:
// Always takes a fixed number of parameters.
// Always takes a single set of parameter types.
// Default values are not supported:
// All call sites have the same number of arguments.
// Macros are sometimes used as an alternative.

// fn pick_one<T>(a: T, b: T) -> T {
//     if std::process::id() % 2 == 0 { a } else { b }
// }

fn multiply(x: i16, y: i16) -> i16 {
    x * y
}
