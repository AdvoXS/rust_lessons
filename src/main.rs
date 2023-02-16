mod struct_les;
mod enum_les6;
mod first_les;
mod exceptions_les;
mod generic_trait_les;
mod traits_les;

use std::error::Error;
use std::fs::File;
use std::ptr::null;
use crate::enum_les6::{enum_with_construct, if_let_test, matcher, test_option};
use crate::exceptions_les::{last_char_of_first_line, test_less_propagation_errors, test_match_exc, test_propagation_errors};
use crate::first_les::first_lesson;
use crate::generic_trait_les::{test_generic_struct, test_largest_generic};
use crate::struct_les::get_persons;
use crate::traits_les::test;

fn main() {
    let text = test_less_propagation_errors().expect("!");
    println!("{}", last_char_of_first_line(text.as_str()).expect("Not found line"));
    test_generic_struct();
    test();
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


