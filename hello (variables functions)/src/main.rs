fn main() {
    let name = String::from("hello world");
    hello(name);
}

fn hello(name: String) {
    println!("Hello, {}!", name);    
}