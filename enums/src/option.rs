pub fn main() {
    let some_number = Some(5);
    let some_char = Some('e');
    let absent_number: Option<i32> = None;

    let x: i8 = 5;
    let y: Option<i8> = Some(5);

    let sum = x + y;
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    /*
    the match construct is exhautive, meaning it requires all the possibilities to be defined.
    If we were to exclude the `none` case, then the compiler would throw an error.
    */
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn dice_rolling() {
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        other => move_player(other), /*
                                     other isn't a keyword, but rather a default that includes every possibility other
                                     than the ones explicitly defined. This variable could be named `get_a_grip` or something.
                                     this HAS to go last since the match arms are evaluated in order, so the catch-all pattern
                                     needs to go at the end.
                                      */
    }
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => reroll(), /*
                       this is rust's catch-all operator when we DON'T want to use the value specified.
                       It MATCHES ANY VALUE but does not bind to that value.
                       because we want to just roll again and not consume the value, the _ operator is used
                       to simply try again ignoring any and all other cases.
                       Again, we place it at the end since the match arms are evaluated in order.
                       */
    }
}

fn reroll() {}

fn move_player(other: u8) {}

fn remove_fancy_hat() {}

fn add_fancy_hat() {}
