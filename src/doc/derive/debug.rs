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
    pub name: String,
    age: u8,
}

fn debug_exm() {
    let ms = MyStruct {
        name: String::from("Dima"),
        age: 39,
    };
    println!("{:?}", ms);
    println!("{:#?}", ms);
    dbg!(ms);
}