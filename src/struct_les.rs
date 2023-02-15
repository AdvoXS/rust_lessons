#[derive(Debug, Clone)]
struct Address {
    latitude: f32,
    longitude: f32,
    region: String,
    city: String,
    street: String,
    house: u8,
}

#[derive(Debug, Clone)]
pub struct Person {
    age: i8,
    first_name: String,
    second_name: String,
    middle_name: String,
    address: Address,
}

impl Person {

}

impl Person { //возможно двойное определение структуры
fn default_init() -> Self { // "конструктор" по умолчанию
    Person {
        age: 0,
        first_name: "none".to_string(),
        second_name: "none".to_string(),
        middle_name: "none".to_string(),
        address: Address {
            latitude: 0.0,
            longitude: 0.0,
            region: "none".to_string(),
            city: "none".to_string(),
            street: "none".to_string(),
            house: 0,
        },
    }
}
    fn say_hello(&self) {
        println!("Hello, {}!", self.first_name);
    }

    pub(crate) fn say_full_hello(&self) {
        print!("Hello, {} {} {}! ", self.second_name, self.first_name, self.middle_name);
        println!("You are from: city={}, street={}, house={}! ", self.address.city, self.address.street, self.address.house);
    }

    pub(crate) fn is_equals_place(&self, another_person: &Person) -> bool {
        self.address.latitude == another_person.address.latitude
            && self.address.longitude == another_person.address.longitude
    }
}


pub fn get_persons() -> (Person, Person, Person) {
    let person_ivan = Person {
        age: 12,
        first_name: "Ivan".to_string(),
        second_name: "T".to_string(),
        middle_name: "R".to_string(),
        address: Address {
            latitude: 0.0,
            longitude: 0.0,
            region: "Moscow oblast".to_string(),
            city: "Moscow".to_string(),
            street: "Tekst".to_string(),
            house: 4,
        },
    };

    let person_maria = Person {
        first_name: "Maria".to_string(),
        second_name: "T".to_string(),
        middle_name: "H".to_string(),
        ..person_ivan.clone()
    };

    let default_person = Person::default_init();
    (person_ivan, person_maria, default_person)
}