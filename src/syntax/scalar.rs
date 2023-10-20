pub fn test_scalar() {
    //simple example
    let mut x: i32 = 6; //mutable variable binding
    print!("{x}"); //Macro for printing, like println! but without newline
    while x != 1 {
        //No parenthesis around expression
        if x % 2 == 0 {
            x = x / 2;
        } else {
            x = 3 * x + 1;
        }
        print!(" -> {x}");
    }

    //Scalar types
    let x = 5; //immutable variable binding
    let mut y = 5; //mutable variable binding
    y = 6;
    println!("x: {x}, y: {y}");

    //Integer types
    let x: i8 = 5; //signed 8-bit integer
    let x: u8 = 5; //unsigned 8-bit integer
    let x: i16 = 5; //signed 16-bit integer
    let x: u16 = 5; //unsigned 16-bit integer
    let x: i32 = 5; //signed 32-bit integer
    let x: u32 = 5; //unsigned 32-bit integer
    let x: i64 = 5; //signed 64-bit integer
    let x: u64 = 5; //unsigned 64-bit integer
    let x: isize = 5; //signed pointer-sized integer
    let x: usize = 5; //unsigned pointer-sized integer

    //Floating-point types
    let x: f32 = 5.0; //32-bit floating-point number
    let x: f64 = 5.0; //64-bit floating-point number

    //Numeric operations
    let sum = 5 + 10; //addition
    let difference = 95.5 - 4.3; //subtraction
    let product = 4 * 30; //multiplication
    let quotient = 56.7 / 32.2; //division
    let remainder = 43 % 5; //remainder

    //Boolean type
    let t = true;
    let f: bool = false; //with explicit type annotation

    //Character type
    let c = 'z';
    let z = 'ℤ';
    let _heart_eyed_cat = '😻';

    // Examples of non-decimal integer representations
    let hex = 0xff; // Hexadecimal format
    let octal = 0o77; // Octal format
    let binary = 0b1111_0000; // Binary format
    let byte = b'A'; // Byte format (u8 only)
}
