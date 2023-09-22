/// -(2 n - 1 ) до 2 n - 1 - 1 включительно
/// целочисленные знаковые: i8, i16, i32, i64, i128, isize
/// let a: i32 = 10

/// от 0 до 2 n - 1
/// целочисленные беззнаковые: u8, u16, u32, u64, u128, usize
/// let b: u8 = 6;

/// с плавающей точкой: f32, f64
/// let c: f32 = 0.5 ;
/// let c: f32 = 5f32;

/// символ: char
/// let d: char = 'g';
/// строка: String
/// let s: String = String::new;

/// булевое: bool
/// let x: bool = true;

/// переменные изначально иммутабельны
/// let mut my_var = 10; изменяемая переменная

fn variables_example() {
    let my_float_64 = 10f64;
    let my_float = 10.0;
    let my_float_32: f32 = 10.0;
    let my_bool = true;
    let my_char = 'c';
    my_char = 'd';

    // изменяемая переменная
    let mut my_mutable_variable = 10;
    my_mutable_variable = 5;
    println!("{}", my_char);

    // тип константы должен быть указан
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

    // tuple (type0, type1, typ2 .. typeN)
    let tup = (1, 2f32, 'c', "s");
    let tup2: (i32, char, str) = (3, 'g', "string");

    // array [type; N]
    let arr = [1, 2, 3, 4, 5];
    let arr2: [char; 3] = ['a', 'b', 'c'];
    let arr3 = [0; 7]; // [0, 0, 0, 0, 0, 0, 0]
}