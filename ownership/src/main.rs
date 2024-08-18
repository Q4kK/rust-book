mod refs_and_borrowing;
mod slice;
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

    // let's try some more

    let x = 5;
    let y = x;

    /*
    in this case, we bind a value of 5 to x (defaulted to i32) and bind the same value to y
    however, if we try it with strings:
    */

    let s1 = String::from("Hello");
    let s2 = s1;

    /*
    in this case, s1 is creating a new string with three components; A pointer to the stack, a length of the
    string, and the capacity that the adress needs

    s2 is not binding to the *value* of s1, but rather binding to the same pointer that s1 uses to get the address on the stack.

    The reasoning behind this is that since strings are a variable size based on the length (unlike integers
    which are already allocated by typing, say i8 -> -127 - 128) rust copying the whole entire string to a
    different point on the heap could be very costly for performance.

    There's another problem. s1 and s2 both have the same pointer to memory.
    In this case, if we try to free the same memory twice using *drop*, we'll get a double free error which
    can cause a program to run arbitrarily. In order to get around this, when we bind s2 to s1, s1 is no
    longer valid and the variable cannot be used. The pointer remains, but the value of s1 is no longer
    bound to the vairable and is instead bound to s2.

    Therefore, if we do want to completely copy a string over, we can do what's known as a clone (using
    clone()).
    */

    {
        let s1 = String::from("Hello");
        let s2 = s1.clone();
        println!("s1 = {s1}, s2 = {s2}");
    }

    /*
    In this case, clone is fully copying all the data from s1 into s2, not including the pointer. The
    allocater takes another amount of free space and frees up another block of memory for the new s2.
    */

    /*
    Going back to the first example, x doesn't move into y because it implements the copy trait. Every type
    has either the copy trait or the drop trait implemented, never both.
    Since x is an integer, it is trivially copied over to y and both are accessable on the stack.

    The types that implement copy are all integers, boolean, floating points, chars, and Tuples if they ONLY
    implement types that also implement copy. I.e, (i32, u32, bool) and not (i32, f64, string)
    */

    {
        let s = String::from("Hello"); // <- here we define s

        takes_ownership(s); // <- here we use s in the function call, and after this line s is no
                            // longer valid

        let x = 5; // <- here we define x and set it to 5

        makes_copy(x); // <- here, since i32 implements copy we copy the value into the function
                       // and x is still able to be used until the scope ends
    }
}

fn takes_ownership(s: String) {
    println!("{s}");
}

fn makes_copy(x: i32) {
    println!("{x}");
}

fn returnvalues() {
    let s1 = some_func(); //define s1, s2, s3
    let s2 = String::from("hello"); // here we define s2
    let s3 = takes_and_gives(s2); // here we define s3 by moving the value of s2 to the function
                                  // takes_and_gives, which passes the string to s3.
} // here, s3 goes out of scope and is dropped. s1 goes out of scope and is dropped.
  // s2 has been moved to takes_and_gives, although the value was returned to s3.

fn some_func() -> String {
    // here we give the some_string value to s1
    let some_string = String::from("hello");
    some_string
}

fn takes_and_gives(s: String) -> String {
    s
}
