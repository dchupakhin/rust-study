use crate::doc::link;

mod doc;

fn main() {
    link::exm_link(&String::from("Hello!"), &mut String::from("mut"));
}
