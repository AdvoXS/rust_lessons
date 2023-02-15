mod struct_les;
mod enum_les6;
mod first_les;

use std::ptr::null;
use crate::enum_les6::{enum_with_construct, if_let_test, matcher, test_option};
use crate::first_les::first_lesson;
use crate::struct_les::get_persons;

fn main() {
    if_let_test();
    println!("{}",test_option());
}

fn test_enum_les() {
    enum_with_construct();
}

fn test_struct_les() {
    let persons = get_persons();
    persons.0.say_full_hello();
    persons.1.say_full_hello();
    persons.2.say_full_hello();
    println!("Иван и Марья живут в одном месте? {}!", if persons.0.is_equals_place(&persons.1) { "Да" } else { "Нет" })
}

fn test_first_les() {
    first_lesson();
}


