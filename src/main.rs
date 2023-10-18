pub mod garden;

use crate::garden::vegetables::Asparagus;
use rust_exercise::back_of_house;
use rust_exercise::front_of_house::hosting;
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

    // let x: i8 = 15;
    // let y: i16 = 1000;

    // println!("{x} * {y} = {}", multiply(x.into(), y));

    // let array = [10, 20, 30];
    // println!("array: {array:?}");
    // for n in array {
    //     println!("{n}")
    // }

    // let temperature = 14;
    // let adjustment  = if temperature<15 {10}else{20}

    // let a = [10, 20, 30, 40, 50];

    // for element in a {
    //     println!("the value is: {}", element);
    // }

    // println!();
    // print!("Iterating over range:");
    // for i in 0..3 {
    //     print!(" {}", array[i]);
    // }
    // println!();

    // Examples of non-decimal integer representations
    // let hex = 0xff; // Hexadecimal format
    // let octal = 0o77; // Octal format
    // let binary = 0b1111_0000; // Binary format
    // let byte = b'A'; // Byte format (u8 only)

    // This will cause a panic if the index is out of bounds
    // An array of integers

    // println!("is_child: {}", is_child(19));

    // struct User {
    //     active: bool,
    //     username: String,
    //     email: String,
    //     sign_in_count: u64,
    // }

    // let user1 = User {
    //     active: true,
    //     username: String::from("user1"),
    //     email: String::from("email@email.com"),
    //     sign_in_count: 1,
    // };

    // let active = user1.active;
    // let username = user1.username;
    // let email = user1.email;
    // let sign_in_count = user1.sign_in_count;
    // // user1.active = false;

    // let username = String::from("test");
    // let mut user2 = User {
    //     active: user1.active,
    //     username,
    //     email: String::from("email2@gmail.com"),
    //     sign_in_count: 1,
    // };

    // user2.active = false;

    // let user3 = User {
    //     active: false,
    //     ..user2
    // };

    // let customWidth = 10;

    // let mut rectangle = Rectangle {
    //     width: dbg!(10 * customWidth),
    //     height: 20,
    // };

    // println!("area: {}", area(&rectangle));
    // println!("rectangle: {:?}", rectangle);

    // dbg!(&rectangle);

    // println!("area: {}", rectangle.area());
    // rectangle.inc_width(200);
    // println!("area: {}", rectangle.area());

    // let square = Rectangle::square(10);
    // println!("square: {:?}", square);

    // let isRightWidth = Rectangle::isRightWidth(10);
    // println!("isRightWidth: {}", isRightWidth);

    //enum
    // #[derive(Debug)]
    // enum IpAddrKind {
    //     V4,
    //     V6,
    // }

    // let ipV4 = IpAddrKind::V4;
    // let ipV6 = IpAddrKind::V6;

    // println!("ipV4: {:?}", ipV4);
    // println!("ipV6: {:?}", ipV6);

    // fn router(ip_kind: IpAddrKind) {
    //     println!("ip_kind: {:?}", ip_kind);
    // }
    // router(ipV4);
    // router(ipV6);

    // struct IpAddr {
    //     kind: IpAddrKind,
    //     address: String,
    // }

    // let home = IpAddr {
    //     kind: IpAddrKind::V4,
    //     address: String::from("127.0.0.1"),
    // };

    // let loopback = IpAddr {
    //     kind: IpAddrKind::V6,
    //     address: String::from("::1"),
    // };

    // enum IpAddr2 {
    //     V4(String),
    //     V6(String),
    // }

    // let home2 = IpAddr2::V4(String::from("127.0.0.1"));
    // let loopback2 = IpAddr2::V6(String::from("::1"));

    // enum IpAddr3 {
    //     V4(u8, u8, u8, u8),
    //     V6(String),
    // }

    // let home3 = IpAddr3::V4(127, 0, 0, 1);
    // let loopback3 = IpAddr3::V6(String::from("::1"));

    // struct Ipv4Addr {
    //     // --snip--
    // }

    // struct Ipv6Addr {
    //     // --snip--
    // }

    // enum IpAddr4 {
    //     V4(Ipv4Addr),
    //     V6(Ipv6Addr),
    // }

    // #[derive(Debug)]
    // enum Message {
    //     Quit,
    //     Move { x: i32, y: i32 },
    //     Write(String),
    //     ChangeColor(i32, i32, i32),
    // }

    // impl Message {
    //     fn call(&self) {
    //         // method body would be defined here
    //     }
    // }

    // let m = Message::Write(String::from("hello"));
    // m.call();

    // let my_move = Message::Move { x: 10, y: 20 };
    // println!("my_move: {:?}", my_move);

    // struct QuitMessage; // unit struct
    // struct MoveMessage {
    //     x: i32,
    //     y: i32,
    // }
    // struct WriteMessage(String); // tuple struct
    // struct ChangeColorMessage(i32, i32, i32); // tuple struct

    // impl MessageImpl for QuitMessage {
    //     fn call(&self) {
    //         println!("QuitMessage");
    //     }
    // }

    // impl MessageImpl for MoveMessage {
    //     fn call(&self) {
    //         println!("MoveMessage");
    //     }

    //     fn move_message(&self) {
    //         println!("MoveMessage");
    //     }
    // }

    // let some_number = Some(5);
    // let some_string = Some("a string");
    // let absent_number: Option<i32> = None;

    // let x: i8 = 5;
    // let y: Option<i8> = Some(5);
    // // let sum = x + y;

    // let sum = x + y.unwrap();

    // enum Coin {
    //     Penny,
    //     Nickel,
    //     Dime,
    //     Quarter,
    // }

    // fn value_in_cents(coin: Coin) -> u8 {
    //     match coin {
    //         Coin::Penny => {
    //             println!("Lucky penny!");
    //             1
    //         }
    //         Coin::Nickel => 5,
    //         Coin::Dime => 10,
    //         Coin::Quarter => 25,
    //     }
    // }

    // let coin = Coin::Penny;
    // println!("coin: {}", value_in_cents(coin));

    // #[derive(Debug)]
    // enum UsState {
    //     Alabama,
    //     Alaska,
    // }

    // enum Coin2 {
    //     Penny,
    //     Nickel,
    //     Dime,
    //     Quarter(UsState),
    // }

    // fn value_in_cents2(coin: Coin2) -> u8 {
    //     match coin {
    //         Coin2::Penny => {
    //             println!("Lucky penny!");
    //             1
    //         }
    //         Coin2::Nickel => 5,
    //         Coin2::Dime => 10,
    //         Coin2::Quarter(state) => {
    //             println!("State quarter from {:?}!", state);
    //             25
    //         }
    //     }
    // }

    // let coin = Coin2::Quarter(UsState::Alaska);
    // println!("coin: {}", value_in_cents2(coin));

    // fn plus_one(x: Option<i32>) -> Option<i32> {
    //     match x {
    //         None => None,
    //         Some(i) => Some(i + 1),
    //     }
    // }
    // let five = Some(5);
    // let six = plus_one(five);
    // let none = plus_one(None);
    // println!("five: {:?}", five);

    // fn add_fancy_hat() {}
    // fn remove_fancy_hat() {}
    // fn move_player(num_spaces: u8) {}

    // let dice_roll = 9;
    // match dice_roll {
    //     1 => add_fancy_hat(),
    //     2 => remove_fancy_hat(),
    //     3 => move_player(4),
    //     4 => move_player(3),
    //     5 => {
    //         add_fancy_hat();
    //         move_player(3);
    //     }
    //     6 => {
    //         remove_fancy_hat();
    //         move_player(2);
    //     }
    //     _ => (),
    // }

    // let config_max = Some(3u8);
    // match config_max {
    //     Some(max) => println!("Max is {}", max),
    //     None => println!("No max provided"),
    // }

    // match config_max {
    //     Some(max) => println!("Max is {}", max),
    //     _ => println!("No max provided"),
    // }

    // if let Some(max) = config_max {
    //     println!("Max is {}", max);
    // } else {
    //     println!("No max provided");
    // }

    //crate、module
    // let plant = Asparagus{};
    // println!("plant: {:?}", plant);

    // hosting::add_to_waitlist();
    // back_of_house::fix_incorrect_order();

    //vector
    // let v: Vec<i32> = Vec::new();
    // println!("v: {:?}", v);

    // let v = vec![1, 2, 3];
    // println!("v: {:?}", v);

    // let mut v2: Vec<i32> = Vec::new();
    // v2.push(5);
    // v2.push(6);
    // v2.push(7);
    // println!("v2: {:?}", v2);

    // let v3 = vec![1, 2, 3, 4, 5];
    // let third: &i32 = &v3[2];
    // println!("third: {}", third);

    // let third: Option<&i32> = v.get(2);
    // println!("third: {:?}", third);
    // match third {
    //     Some(third) => println!("third: {}", third),
    //     None => println!("third: None"),
    // }

    // let v4 = vec![1, 2, 3, 4, 5];
    // // let does_not_exist = &v4[100];
    // // println!("does_not_exist: {}", does_not_exist);
    // let does_not_exist = v.get(100);
    // println!("does_not_exist: {:?}", does_not_exist);

    // let mut v5 = vec![1, 2, 3, 4, 5];
    // let first = &v5[0];
    // // v5.push(6);
    // println!("first: {}", first);

    // let v6 = vec![100, 32, 57];
    // for i in &v6 {
    //     println!("{}", i);
    // }

    // let mut v7 = vec![100, 32, 57];
    // for i in &mut v7 {
    //     *i += 50;
    // }
    // println!("v7: {:?}", v7);

    // enum SpreadsheetCell {
    //     Int(i32),
    //     Float(f64),
    //     Text(String),
    // }

    // let row = vec![
    //     SpreadsheetCell::Int(3),
    //     SpreadsheetCell::Float(10.12),
    //     SpreadsheetCell::Text(String::from("blue")),
    // ];

    // {
    //     let v8 = vec![1, 2, 3, 4];
    //     // do stuff with v
    // }// <- v goes out of scope and is freed here


    //String
    let mut s = String::new();
    let data = "initial contents";
    let s = data.to_string();
    let s = "initial contents".to_string();
    let s = String::from("initial contents");

    let mut s = String::from("foo");
    s.push_str("bar");
    println!("s: {}", s);

    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    s1.push('s');
    println!("s1: {}", s1);

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used
    println!("s3: {}", s3);

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    // let s = s1 + "-" + &s2 + "-" + &s3;
    let s = format!("{}-{}-{}", s1, s2, s3);
    println!("s: {}", s);

    let hello = "Здравствуйте";
    let s = &hello[0..4];
    println!("s: {}", s);

    for c in hello.chars() {
        println!("{}", c);
    }
    
}

// #[derive(Debug)]
// struct Rectangle {
//     width: u32,
//     height: u32,
// }

// impl Rectangle {
//     fn area(&self) -> u32 {
//         self.height * self.width
//     }

//     fn inc_width(&mut self, delta: u32) {
//         self.width += delta;
//     }

//     fn square(size: u32) -> Rectangle {
//         Rectangle {
//             width: size,
//             height: size,
//         }
//     }

//     fn isRightWidth(width: u32) -> bool {
//         width > 5
//     }
// }

// fn area(rectangle: &Rectangle) -> u32 {
//     rectangle.width * rectangle.height
// }

// fn is_child(age: i32) -> bool {
//     if (age < 18) {
//         return true;
//     }
//     false
// }

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

// fn multiply(x: i16, y: i16) -> i16 {
//     x * y
// }
