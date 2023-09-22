/// match сравнивает значение с различными шаблонами и затем выполняет код
/// в зависимости от того, какой из шаблонов совпал
///

pub enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

pub enum RandomNumber {
    Integer(i32),
    Long(i64),
}

pub fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

pub fn luck_number(number: RandomNumber) -> bool {
    match number {
        RandomNumber::Integer(777) => true,
        RandomNumber::Integer(11) => true,
        RandomNumber::Long(777) => true,
        RandomNumber::Long(11) => true,
        _ => false,
    }
}

pub fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

pub fn dice_roll(x: i32) {
    match x {
        1 => {
            println!("crit miss")
        }
        20 => {
            println!("crit")
        }
        other => {
            println!("{other}")
        }
    }
}

/// if let обрабатывает только один шаблон
pub fn _if_let_exm(x: Option<u8>) -> bool {
    if let Some(mut z) = x {
        z += 1;
        println!("{z}");
        true;
    }
    false
}
