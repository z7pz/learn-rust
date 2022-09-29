mod my_mod {
    pub struct User {
        pub name: String,
        pub age: u8
    }
     impl User {
        // private by default
        fn message(&self) -> String {
            return format!("{}: i am {}", self.name, self.age);
        }
        pub fn speak(&self) {
            println!("{}", self.message());
        } 
    }
}

fn main() {
    let user = my_mod::User {name: String::from("Ahmed"), age: 10};
    user.speak();
}
