fn string_manip() {
    let s1 = String::from("hello"); // <- here we define a string

    let len = calculate_length(&s1); // <- here we make a variable called `len` using the reference
                                     // to s1. Since the onwership is still with s1, when we return the value
                                     // from the function, len will own the returned value and s1 will
                                     // still own the string.

    println!("The length of {s1} is {len}"); // <- that's what allows us to print both values out here
                                             // without copying either

    let mut s = String::from("hi");

    let r1 = &s; // no problem
    let r2 = &s; // no problem

    println!("{r1} and {r2}");
    // variables r1 and r2 are out of scope after this point

    let r3 = &mut s; // no problem
    println!("{r3}");
}

fn calculate_length(s1: &str) -> usize {
    s1.len()
} // <- here, s1 goes out of scope but since it doesn't own the thing it refers to
  // it is not dropped after returning the value

fn mutable_references() {
    let mut s = String::from("Hello");

    change(&mut s);
}

fn change(str: &mut String) {
    str.push_str(", world");
}

/*
this works because a mutable reference is allowed to change the thing it's referencing, but
*only* the thing it's referencing.
The biggest restriction with mutable references is that you can't have any other references to the variable.
*/

fn main_dangle() {
    let ref_to_nothing = dangle(); // here we try to define a variable as a string reference
}

fn dangle() -> &String {
    // lifetime starts here
    let s = String::from("hello"); // here the string is created
    &s // here we return a reference to the string, but after this line the string it's
       // referring to goies out of scope, and the memory dissapears
}

fn no_dangle() -> String {
    let s = String::from("hello");
    s
}
