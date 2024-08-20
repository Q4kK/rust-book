fn main() {
    let config_max = Some(3u8);
    match config_max {
        Some(max) => println!("The maximum is configured to be {max}"), // if the value exists, we print it
        _ => (), // anything else we send to the shadow realm
    } // this is a good way to do it, but there's a better way.

    if let Some(max) = config_max {
        println!("The maximum is configured to be {max}");
    } // ok, lets figure this out

    /*
    the let Some(max) = config_max part simply defines the Some value as whatever config_max is.
    Then, it prints the line.
    However, the `if let` part of that statement only executes the code if the Some(max) value evaluates
    to true, which it won't if Some(max) is equal to None.
    This is a more concise way of writing out our first match statement, but without needing to explicitly
    define the `None` or other behavior.
    The problem with if let is that you don't get to use the exauhstive behavior.
    For example:
    */

    // let other_behavior = Some(3);
    if let Some(behavior) = config_max {
        println!("This is the behavior!");
    // } else if Some(other_behavior) = other_behavior { // <- this doesn't work, you can only define 2 behaviors with if let and else.
    //     println!("Okay.")
    } else {
        println!("Nope.")
    }

    /*
    Not only is this completely verbose, it also doesn't work as intended and not as robustly as a match statement.
    */
}
