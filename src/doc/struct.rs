/// создание: MyStruct {
///             name: String::from("Dfgg"),
///             age: 34
///           }
struct MyStruct {
    /// открытое поле
    pub name: String,
    /// приватное поле
    age: u8,
}

/// Методы структуры.
impl MyStruct {
     /// метод экземпляра
     /// * `&self` всегда первым параметром. ссылка на экземпляр
     /// вызывается ms.get_name()
    fn get_name(&self) -> &String {
        &(self.name)
    }

    fn get_age(&self) -> u8 {
        self.age
    }

     /// ассоциированная функция
     /// вызывается MyStruct::print(ms)
    fn print(ms: MyStruct) {
        println!("MyStruct {{name: {}; age: {}}}", ms.get_name(), ms.get_age());
    }

    /// конструктор
    fn new(name: &str, age: u8) -> Self {
        Self {
            name: String::from(name),
            age,
        }
    }
}

/// структура на основе кортежа, без именованных полей
/// создание: Color(1, 23, 43, 0.5)
struct Color(i32, i32, i32, f32);

impl Color {
    fn print(&self) {
        println!("Red:{} Green:{} Blue:{} Opacity:{}", self.0, self.1, self.2, self.3)
    }
}

pub fn struct_exm() {
    let ms = MyStruct::new("Dima", 39);
    /// обращение к открытому полю
    let _name = &ms.name;
    /// вызов метода экземпляра
    let _age = ms.get_age();
    /// вызов метода структуры
    MyStruct::print(ms);

    let some_color = Color(1, 1, 1, 0.5);
    some_color.print();
}

