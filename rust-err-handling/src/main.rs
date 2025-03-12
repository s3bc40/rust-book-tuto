use std::fs::File;

fn main() {
    // panic!("crash and burn");
    let v = vec![1, 2, 3];
    // v[99]; // panic

    let greeting_file_result = File::open("hello.txt");
    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {error:?}"),
    };
}
