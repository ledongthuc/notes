struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

struct color (i32, i32, i32);

struct unit_like_struct;

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn square(size: u32) -> Rectangle {
        return Rectangle{
            width: size,
            height: size,
        }
    }
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn set_width(&mut self, width: &u32) {
        self.width = *width;
    }
    fn can_hold(&self, another_rect: &Rectangle) -> bool {
        self.width > another_rect.width && self.height > another_rect.height
    }
}

fn main() {
    let mut user1 = User {
        email: String::from("ledongthuc@gmail.com"),
        username: String::from("ledongthuc"),
        active: true,
        sign_in_count: 1,
    };
    println!("Hello, {:?}", user1.email);

    user1.email = String::from("ledongthuc1@gmail.com");
    println!("Hello, {:?}", user1.email);

    let user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };
    println!("Hello, user2: {} with status {}", user2.email, user2.active);

    let tupe_struct = color(32, 50, 11);
    println!("Tupe struct: {:?}", tupe_struct.0);

    let subject = unit_like_struct;

    let mut rect = Rectangle {
        width: 30,
        height: 50,
    };
    println!("rect: {:?} with are {}", rect, rect.area());
    rect.set_width(&70);
    println!("rect: {:?} with are {}", rect, rect.area());

    let another_rect = Rectangle {
        width: 10,
        height: 20,
    };
    println!("rect1 {:?} can hold rect2 {:?}: {}", rect, another_rect, rect.can_hold(&another_rect));

    let square1 = Rectangle::square(10);
    println!("square1: {:?}", square1);
}
