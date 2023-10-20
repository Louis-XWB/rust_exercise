pub fn test_array() {
    let mut arr: [i32; 6] = [10, 20, 30, 40, 50, 60];
    arr[1] = 21;
    println!("arr[1]: {}", arr[1]); //Array type
    println!("arr: {:?}", arr); //Array type

    let array = [10, 20, 30];
    println!("array: {array:?}");
    for n in array {
        println!("{n}")
    }

    let temperature = 14;
    let adjustment  = if temperature<15 {10}else{20};

    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {}", element);
    }

    println!();
    print!("Iterating over range:");
    for i in 0..3 {
        print!(" {}", array[i]);
    }
    println!();
}
