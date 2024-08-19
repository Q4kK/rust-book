pub fn main() {
    let rect1 = (30, 50);

    println!("The area of the rectangle is {} square pixels", area(rect1));
}

fn area(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}
/*
this part is good, but it could be better. None of the tuple's variables are named,
and having a name to know which part to use would be great. This is where we can
refactor for structs.
*/
#[derive(Debug)]
struct Rectangle {
    width: u64,
    height: u64,
}

pub fn main2() {
    let rectangle = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the sctruct rectangle is {} square pixels.",
        area2(&rectangle),
    );
}

fn area2(r1: &Rectangle) -> u64 {
    r1.width * r1.height
} // I made this function originally with a recangle parameter like r1: Rectangle
  // but using a reference is better since the main2() function can continue to use
  // rectangle after I print it out.

pub fn print_struct() {
    let r1 = Rectangle {
        width: 30,
        height: 50,
    };
    println!("The rectangle contains {:#?}", r1); // This doesn't work normally, we need to impliment the Debug trait onto the Rectangle struct.
}
