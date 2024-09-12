mod coin_match;
mod caluculation;

use coin_match::{Coin, UsState, value_in_cents};
use caluculation::caluculate;

fn main() {
    let coin1 = Coin::Quarter(UsState::Alaska);
    let value1 = value_in_cents(coin1);
    println!("The coin's value is: {} cents", value1);

    let coin2 = Coin::Penny;
    let value2 = value_in_cents(coin2);
    println!("The coin's value is: {} cents", value2);

    let coin3 = Coin::Nickel;
    let value3 = value_in_cents(coin3);
    println!("The coin's value is: {} cents", value3);

    let coin4 = Coin::Dime;
    let value4 = value_in_cents(coin4);
    println!("The coin's value is: {} cents", value4);

    // 四則演算
    caluculate();
}
