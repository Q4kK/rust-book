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

    
}
