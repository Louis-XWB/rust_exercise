pub fn test_fn() {
    print_fizzbuzz_to(21);

    println!("coin toss: {}", pick_one("heads", "tails"));
    println!("cash prize: {}", pick_one(500, 1000));

    let x: i8 = 15;
    let y: i16 = 1000;

    println!("{x} * {y} = {}", multiply(x.into(), y));

    println!("is_child: {}", is_child(19));
}

//function Rustdoc
fn is_divisible_by(lhs: u32, rhs: u32) -> bool {
    if rhs == 0 {
        return false;
    } else {
        return true;
    }
}

fn is_divisible(n: u32, divisor: u32) -> bool {
    if divisor == 0 {
        return false;
    }
    n % divisor == 0
}

fn fizzbuzz(n: u32) -> String {
    let fizz = if is_divisible(n, 3) { "fizz" } else { "" };
    let buzz = if is_divisible(n, 5) { "buzz" } else { "" };
    if fizz.is_empty() && buzz.is_empty() {
        return format!("{n}");
    }
    format!("{fizz}{buzz}")
}

fn print_fizzbuzz_to(n: u32) {
    for i in 1..=n {
        println!("{}", fizzbuzz(n))
    }
}

// Each function has a single implementation:
// Always takes a fixed number of parameters.
// Always takes a single set of parameter types.
// Default values are not supported:
// All call sites have the same number of arguments.
// Macros are sometimes used as an alternative.
fn pick_one<T>(a: T, b: T) -> T {
    if std::process::id() % 2 == 0 {
        a
    } else {
        b
    }
}

fn multiply(x: i16, y: i16) -> i16 {
    x * y
}

fn is_child(age: i32) -> bool {
    if (age < 18) {
        return true;
    }
    false
}
