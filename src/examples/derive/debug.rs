/// * `#[derive(Debug)]` позволяет выводить структуры в виде строкового отображения
/// * `println!("{:?}", ms)`  неформатированный вывод MyStruct { name: "Dima", age: 39 }
/// * `println!("{:#?}", ms)` форматированный вывод MyStruct {
///                                                     name: "Dima",
///                                                     age: 39,
///                                                 }
/// * `dbg!()` печатает в stderr номер файла и строки, где происходит вызов макроса dbg!,
/// вместе с результирующим значением этого выражения и возвращает владение на значение.
#[derive(Debug)]
struct MyStruct {
    _name: String,
    _age: u8,
}

pub fn exm() {
    let ms = MyStruct {
        _name: String::from("Dima"),
        _age: 39,
    };
    println!("{:?}", ms);
    println!("{:#?}", ms);
    dbg!(ms);
}