
pub fn test_structs() {
    let mut rect = Rectangle {
        width: 30,
        height: 50,
    };

    println!("rect area: {}", rect.area());
    rect.inc_width(20);
    println!("rect area: {}", rect.area());

    struct User {
        active: bool,
        username: String,
        email: String,
        sign_in_count: u64,
    }

    let user1 = User {
        active: true,
        username: String::from("user1"),
        email: String::from("email@email.com"),
        sign_in_count: 1,
    };

    let active = user1.active;
    let username = user1.username;
    let email = user1.email;
    let sign_in_count = user1.sign_in_count;
    // user1.active = false;

    let username = String::from("test");
    let mut user2 = User {
        active: user1.active,
        username,
        email: String::from("email2@gmail.com"),
        sign_in_count: 1,
    };

    user2.active = false;

    let user3 = User {
        active: false,
        ..user2
    };

    let customWidth = 10;

    let mut rectangle = Rectangle {
        width: dbg!(10 * customWidth),
        height: 20,
    };

    println!("area: {}", area(&rectangle));
    println!("rectangle: {:?}", rectangle);

    dbg!(&rectangle);

    println!("area: {}", rectangle.area());
    rectangle.inc_width(200);
    println!("area: {}", rectangle.area());

    let square = Rectangle::square(10);
    println!("square: {:?}", square);

    let isRightWidth = Rectangle::isRightWidth(10);
    println!("isRightWidth: {}", isRightWidth);
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.height * self.width
    }

    fn inc_width(&mut self, delta: u32) {
        self.width += delta;
    }

    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }

    fn isRightWidth(width: u32) -> bool {
        width > 5
    }
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
