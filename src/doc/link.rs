/// * `&String` - ссылка на чтение.
/// * `&mut String` - ссылка на запись.
/// Одновременно в области действия может быть только одна ссылка на запись или несколько ссылок на
/// чтение.
pub fn exm_link(s: &String, ms: &mut String) {
    println!("{s}");
    ms.push_str("another str");
}
