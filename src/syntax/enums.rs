pub fn test_enums() {
    #[derive(Debug)]
    enum IpAddrKind {
        V4,
        V6,
    }

    let ipV4 = IpAddrKind::V4;
    let ipV6 = IpAddrKind::V6;

    println!("ipV4: {:?}", ipV4);
    println!("ipV6: {:?}", ipV6);

    fn router(ip_kind: IpAddrKind) {
        println!("ip_kind: {:?}", ip_kind);
    }
    router(ipV4);
    router(ipV6);

    struct IpAddr {
        kind: IpAddrKind,
        address: String,
    }

    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    enum IpAddr2 {
        V4(String),
        V6(String),
    }

    let home2 = IpAddr2::V4(String::from("127.0.0.1"));
    let loopback2 = IpAddr2::V6(String::from("::1"));

    enum IpAddr3 {
        V4(u8, u8, u8, u8),
        V6(String),
    }

    let home3 = IpAddr3::V4(127, 0, 0, 1);
    let loopback3 = IpAddr3::V6(String::from("::1"));

    struct Ipv4Addr {
        // --snip--
    }

    struct Ipv6Addr {
        // --snip--
    }

    enum IpAddr4 {
        V4(Ipv4Addr),
        V6(Ipv6Addr),
    }

    #[derive(Debug)]
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }

    impl Message {
        fn call(&self) {
            // method body would be defined here
        }
    }

    let m = Message::Write(String::from("hello"));
    m.call();

    let my_move = Message::Move { x: 10, y: 20 };
    println!("my_move: {:?}", my_move);

    struct QuitMessage; // unit struct
    struct MoveMessage {
        x: i32,
        y: i32,
    }
    struct WriteMessage(String); // tuple struct
    struct ChangeColorMessage(i32, i32, i32); // tuple struct

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

    enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter,
    }

    fn value_in_cents(coin: Coin) -> u8 {
        match coin {
            Coin::Penny => {
                println!("Lucky penny!");
                1
            }
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter => 25,
        }
    }

    let coin = Coin::Penny;
    println!("coin: {}", value_in_cents(coin));

    #[derive(Debug)]
    enum UsState {
        Alabama,
        Alaska,
    }

    enum Coin2 {
        Penny,
        Nickel,
        Dime,
        Quarter(UsState),
    }

    fn value_in_cents2(coin: Coin2) -> u8 {
        match coin {
            Coin2::Penny => {
                println!("Lucky penny!");
                1
            }
            Coin2::Nickel => 5,
            Coin2::Dime => 10,
            Coin2::Quarter(state) => {
                println!("State quarter from {:?}!", state);
                25
            }
        }
    }

    let coin = Coin2::Quarter(UsState::Alaska);
    println!("coin: {}", value_in_cents2(coin));
}
