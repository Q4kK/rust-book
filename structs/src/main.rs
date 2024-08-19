mod examplestructs;
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

struct AlwaysEqual; // <- here we can define a struct with no type and initialize it
                    // pretty cool

struct User {
    active: bool,
    // email: &str,
    // username: &str,
    sign_in_count: u64,
    /*
    here the User struct is expecting a lifetime parameter because
    the reference needs to be valid for the lifetime of the struct. If the
    reference goes out of scope but the struct stays in scope, that would cause
    an empty pointer and some really bad issues.
    */
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        // &self is a shorthand for (self: &self) and allows us to reference the
        // struct with the same name type
        self.width * self.height
    }
}

fn main() {
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    let subject = AlwaysEqual;

    // examplestructs::main();
    examplestructs::main2();
    examplestructs::print_struct();

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("the area of the rect1 with impl is {}", rect1.area());
}
