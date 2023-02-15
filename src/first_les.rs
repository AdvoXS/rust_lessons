use std::cmp::Ordering;
use std::io;
use rand::Rng;

pub fn first_lesson() {
    let secret_number = rand::thread_rng().gen_range(1..=100);
    test_collections();
    loop {
        let mut str = String::new();
        print_type_of(&str);
        io::stdin().read_line(&mut str).expect("wer");
        let str: u32 = match str.trim().parse()
        {
            Ok(num) => num,
            Err(_) => {
                println!("Неверно введено значение. Введите число!");
                continue;
            }
        };

        print_type_of(&str);

        match str.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}

fn test_collections() {
    let tuple: (u32, &str, char) = (2, "2", '2');
    let arr = [3; 5];

    let el_tuple = tuple.2;
    let el_array = arr[0];

    println!("element of tuple: {el_tuple}, element of tuple: {el_array}")
}

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}