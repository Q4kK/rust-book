mod if_let;
mod option;
// structs are nice for collecting similar data that we might want to keep
// in the same spot, like a shape or an ip adress, but it requires a type
// and also all the stuff inside the struct needs to be defined. [0]

enum IpAddrKind {
    V4,
    V6,
} // enums solve this problem by
  // a) allowing us to define a variable that can be one of many possible types
  // b) allowing functions to take in the Ip type without needing to specify v4 or v6 [1]

struct IpAddr {
    kind: IpAddrKind,
    address: String,
} // we can use structs with the enum in order to get some cool results, but it'd be even cooler if we could do it in one line [2]

enum IpName {
    V4(String),
    V6(String),
}

fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6; // [1]

    // we can use the "generic" (the type isn't but the input is) function below on either variable: [1]
    route(four);
    route(six);

    let home = IpName::V4(String::from("127.0.0.1"));
    let loobpack = IpName::V6(String::from("::1")); // [2]

    let m = Message::Write(String::from("Hello"));
    m.call();

    option::main();
}

fn route(_ip: IpAddrKind) {
    // here we define a function that can take in either a v4 or v6 but we don't know which one it'll take in by compile time
    // so we classify the type as the ip enum that lets the program choose from our ip enum at runtime [1]
}

enum Message {
    // [3]
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

// to write the same as [3] in struct form
struct QuitMessage;
struct MoveMessage {
    x: i32,
    y: i32,
}
struct WriteMessage(String);
struct ChangeColorMessage(i32, i32, i32);

// and man, that's a lot more work for the same stuff

impl Message {
    fn call(&self) {
        // do stuff
    }
}
