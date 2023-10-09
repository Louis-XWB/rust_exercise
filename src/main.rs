fn main() {
    // println! is a macro that prints text to the console
    //println!("Hello, world!");
    //print!("Test print!");

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

    println!();
}
