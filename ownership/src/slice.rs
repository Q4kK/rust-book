use std::io::Bytes;

fn main() {
    let mut s = String::from("hello world");

    let word = first_word(&s); // word will get the value of 5

    // do stuff with word here

    s.clear() // now there's no string, so what can we use 5 for?
              // nothing!
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        // define a loop that iterates through each byte of the string
        if item == b' ' {
            // if the item is equal to the byte literal syntax of a space, return
            return i;
        }
    }

    s.len() // otherwise, return the length of the string
}

fn slices() {
    let s = String::from("hello world");

    let hello = &s[0..5];
    let world = &s[6..11]; /* <- these are string slices, and they're really useful. Rather than
                           making a reference to the whole string and mainpulating from there,
                           slices are referencing a portion of the string, specified by the [..] bit.
                           */
}

// now if we rewrite first_word to return the string slice

fn first_word_slice(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
