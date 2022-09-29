struct User {
    name: String,
    age: u8,
}
impl User {
    fn message(&self) -> String {
        return format!("Hello i am {}, i am {}", self.name, self.age);
    }
    fn speak(&self) {
        println!("{}", self.message())
    }
}
fn main() {
    let user = User {
        name: String::from("Ahmed"),
        age: 10,
    };
    user.speak()
}
