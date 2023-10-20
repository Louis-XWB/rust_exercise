pub fn test_option() {
    let some_number = Some(5);
    let some_string = Some("a string");
    let absent_number: Option<i32> = None;

    let x: i8 = 5;
    let y: Option<i8> = Some(5);
    // let sum = x + y;

    let sum = x + y.unwrap();

    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            None => None,
            Some(i) => Some(i + 1),
        }
    }
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    println!("five: {:?}", five);

    fn add_fancy_hat() {}
    fn remove_fancy_hat() {}
    fn move_player(num_spaces: u8) {}

    let dice_roll = 9;
    match dice_roll {
        1 => add_fancy_hat(),
        2 => remove_fancy_hat(),
        3 => move_player(4),
        4 => move_player(3),
        5 => {
            add_fancy_hat();
            move_player(3);
        }
        6 => {
            remove_fancy_hat();
            move_player(2);
        }
        _ => (),
    }

    let config_max = Some(3u8);
    match config_max {
        Some(max) => println!("Max is {}", max),
        None => println!("No max provided"),
    }

    match config_max {
        Some(max) => println!("Max is {}", max),
        _ => println!("No max provided"),
    }

    if let Some(max) = config_max {
        println!("Max is {}", max);
    } else {
        println!("No max provided");
    }
}
