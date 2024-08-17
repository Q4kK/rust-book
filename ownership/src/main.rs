fn main() {
    // OWNERSHIP RULES

    /*
    1) Each value in rust has an owner
    2) There can only be one owner at a time
    3) when the owner goes out of scope, the value will be dropped.
    */

    // let's try some

    // Here, s is not valid because it isn't in scope
    {
        let s = "hello"; // <- we can start accessing s after this line

        // do stuff
    } // <- this scope is now over and s is no longer valid

    {
        let mut s = String::from("hello");
        s.push_str(", world!");
        println!("{s}"); // this will print hello world
    }
}
