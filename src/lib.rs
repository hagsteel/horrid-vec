pub mod horrible_vec;

pub use horrible_vec::HorridVec;

#[derive(Debug, Clone)]
struct Person {
    name: String,
    age: u32,
}

impl Person {
    fn new(name: &str) -> Self {
        Self {
            name: name.into(),
            age: 123,
        }
    }
}

impl Drop for Person {
    fn drop(&mut self) {
        eprintln!("dropping {}", self.name);
    }
}

pub unsafe fn unsafe_main() {
    let person_0 = Person::new("Florp");
    let person_1 = Person::new("The index guy");
    let person_2 = Person::new("Kittens");

    let mut vec = HorridVec::with_capacity(3);
    vec.push(person_0);
    vec.push(person_1);
    vec.push(person_2);

    for val in vec.into_iter() {
        // eprintln!("{:?}", val);
    }
}
