use std::collections::btree_set::Difference;

fn main() {
    let x = 5; // <- variable is immutable here
    println!("The value of x is: {x}");
    x = 6; // <- so we can't modify it here
    println!("The value ox x is: {x}");

    let mut z = 5; // if we mark is a mutable
    println!("The value of z is: {z}");
    z = 6; // <- then we can modify the variable here
    println!("The value of z is: {z}");

    const KEEP_POSITIVE: i32 = 600; // this guy can't be modified ever

    let y = 5; // <- here we bind y to a value of 5
    let y = y + 1; // this works because y is immutable, so we instead bind not the value y to y + 1 but we assign a new value to y that equals y + 1;
                   // it's the difference between (x = x + 1) and (let x = x + 1)
                   // in the first example we set x to its new value which can't work for immutable objects (by definition, they can't be modified)
                   // in the second example, we *assign* the variable x to the new value which does work because we're not modifying the value inside the variable, we're modifying the variable itself and changing where the memory address is located

    let spaces = "    "; // this works because of ^, where the value isn't modified but rather the variable is
    let spaces = spaces.len();

    let mut spaces2 = "    ";
    spaces2 = spaces2.len(); // however this doesn't - we can't modify a variable's type. 

    let spaces3: &str = "    ";
    spaces3: usize = spaces3.len(); // a typed version of the one seen above where you can visually see the types clashing

    // this is why stuff like parse() exists
}

fn data_types() {
    let guess: u32 = "42".parse().expect("Not a number!"); // we need the typing here to tell parse() what to expect as the return result, otherwise parse could be trying to return either a signed or unsigned int of any kind, double, long, or any numerical data type
    
    let x = 2.0; // f64 is the rust default
    let y: f32 = 3.0; // f32 is explicitly typed

    // #Operations
    let sum = 5 + 10;

    let difference = 99.5 - 4.3;

    let product = 6 * 3;

    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // <- this should result in -1, -5 / 3 ~= -1.667, and since the digits are an integer the rest gets truncated off to the integer at the front, *NOT* rounded to the nearest integer.

    let remainder = 10 % 3;

    let c = 'z';
    let z: char = z; // <- just the previous example but explicitly typed

    // COMPOUND TYPES
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    let (f, g, h) = tup;
    // here, g == 6.4 and is assigned as a f64

    let tupl: (i32, f64, u8) = (500, 6.4, 1); // we can access the tuple element directly by just indexing the value of the thing we want
    let five_hundred = tupl.0;
    let six_point_four = tupl.1;
    let one = tupl.2;

    let array = [1, 2, 3, 4, 5]; // arrays are useful when you know the amount of items you need to store
    let three_arr = [3; 5];

    
}

fn functions() {
    println!("Running another function!");

    another_function();

    let x: i32 = 12;
    function_with_params(x);
}

fn function_with_params(x: i32) {
    println!("The value of x is {x}");
}

fn another_function() {
    println!("this is another function!");
}

fn expressions_and_scopes() {
    let y = (let x = 3); // this can't work becasue x doesn't return a value for y to bind to
    
    let a = {
        let b = 3;
        b + 1
    } // this does because the inner scop of this function returns a value of b + 1;

    // adding a semicolon at the end of a statement makes it not return a value, but removing the semicolon does

    //however, many functions have return values such as return_five();
}

fn control_flow() {
  // #if expressions  
  let number = 3;
  if number > 5 {
      println!("Condition was met!");
  } else {
      println!("Condition was false");
  }

  let condition = true;

  //inline if statements, rust doesn't have the other stuff
  let mut number = if condition { 5 } else { 6 };

  // can't do let number = condition ? 5 : 6;

  let mut counter = 6;

  let result = loop {
      counter += 1;

      if counter == 10 {
          break counter * 2;
      }
  };

  while number != 0 {
      println!("{number}");
      number -= 1;
  }

}
