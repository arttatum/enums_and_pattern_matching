#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

impl Coin {
    fn value_in_cents(&self) -> u8 {
        match self {
            Coin::Penny => 1,
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter => 25,
        }
    }
}

fn main() {
    let coins = [Coin::Penny, Coin::Nickel, Coin::Dime, Coin::Quarter];
    
    for coin in coins {
        println!("{:?} has a value of {} cents", coin, coin.value_in_cents());
        print_luck_for_penny(&coin);
        print_penny_or_other(&coin);
    }
}

// The 'discard' symbol (_) indicates we don't wish to use the value.
fn print_luck_for_penny(coin: &Coin) {
    match coin {
        Coin::Penny => println!("Luck you!"),
        _ => println!("Not so lucky :(")
    }
}

// We can use the value in a 'catch all' arm like so
fn print_penny_or_other(coin: &Coin) {
    match coin {
        Coin::Penny => println!("It's a {:?}, lucky you!", Coin::Penny),
        other_coin => println!("It's a {:?}, hard luck.", other_coin)
    }
}

// Rust's compiler requires us to handle all cases.
// If we only wish to take action for certain values, we can inform the compiler that 
// in all other cases, we really don't want to do anything.
fn print_penny_or_do_nothing(coin: &Coin) {
    match coin {
        Coin::Penny => println!("Do a celebration dance!"),
        _ => () 
    }
}

// The same logic can be executed using the if_let construct
fn print_penny_or_do_nothing_if_let(coin: &Coin) {
 if let Coin:Penny = coin {
    Coin::Penny => println!("Do a celebration dance!");
 }   
}

fn printpenny_or_other_if_let(coin: &Coin) {
    if let Coin::Penny = coin {
        println!("It's a {:?}, lucky you!", Coin::Penny);
    } else {
        println!("It's a {:?}, hard luck.", coin);
    }
}