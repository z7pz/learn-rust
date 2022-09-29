mod dir;
mod test;
fn main() {
    let test_message = test::test();
    println!("{}", test_message);
    
    let dir_hello_message = dir::hello::hello();
    println!("{}", dir_hello_message);

    let dir_idk_message = dir::idk::idk();
    println!("{}", dir_idk_message);

    let dir_message = dir::idk();
    println!("{}", dir_message);
}
