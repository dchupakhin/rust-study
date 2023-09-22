use crate::examples::matches::{Coin, RandomNumber};
pub use crate::examples::{enums, links, matches, structures, variables, derive};
use crate::examples::collections;

mod examples;

fn main() {
    enums::exm();
    links::exm(&String::from("Hello!"), &mut String::from("mut"));
    matches::dice_roll(8);
    matches::luck_number(RandomNumber::Integer(777));
    matches::luck_number(RandomNumber::Long(14));
    matches::value_in_cents(Coin::Penny);
    matches::value_in_cents(Coin::Nickel);
    matches::value_in_cents(Coin::Dime);
    matches::value_in_cents(Coin::Quarter);
    matches::plus_one(Some(9));
    matches::_if_let_exm(Some(7));
    structures::exm();
    variables::exm();
    derive::debug::exm();
    collections::vectors::exm();
    collections::strings::exm();
}
