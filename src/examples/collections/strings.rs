/// строки реализованы в виде набора байтов
/// &str    String

pub fn exm() {
    // создание строки
    let mut _s1 = String::new();
    let mut _s2 = "test data".to_string();
    let _s3 = String::from("test string from");

    // конкатенация
    _s1.push_str("data test");
    _s1.push(' ');
    _s1.push('c');
    _s2 += "test data";

    let _s4 = format!("{_s1}-{_s2}-{_s3}");

    // строка как коллекция символов
    for c in _s1.chars() {
        println!("{c}");
    }

    // строка как коллекция байт
    for b in _s1.bytes() {
        println!("{b}");
    }
}