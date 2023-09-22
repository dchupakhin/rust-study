use crate::matches::Coin;

/// Вектор Vec<T>
/// Векторы позволяют хранить более одного значения в единой структуре данных,
/// хранящей элементы в памяти один за другим.
/// Векторы могут хранить данные только одного типа.

pub fn exm() {
    // создание нового вектора
    let mut v: Vec<i32> = Vec::new();
    let _v2: Vec<&str> = Vec::with_capacity(2);

    // добавление элементов
    v.push(00);
    v.push(11);
    v.push(22);

    // получение по индексу
    let _el2 = &v[2];

    // метод get(index: i32) возвращает Option
    let el1 = v.get(1);
    if let Some(x) = el1 {
        println!("{x}");
    }

    // макрос для созданя вектора
    let _v3 = vec![1, 2, 3]; // [1, 2, 3]
    let _v4 = vec![""; 5]; // ["", "", "", "", ""]

    // взятие последнего значения с удалением из вектора
    let _top = v.pop();

    // итерирование по вектору
    for i in &v {
        println!("{i}")
    }

    // итерирование по вектору с изменением элементов
    for i in &mut v {
        *i += 50;
        println!("{i}")
    }

    let _coin_vec = vec![Coin::Penny, Coin::Quarter, Coin::Dime, Coin::Nickel];
}