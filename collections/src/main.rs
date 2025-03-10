fn main() {
    let v: Vec<i32> = Vec::new();
    let v = vec![1, 2, 3];

    // Updating a Vector
    let mut v = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    // Reading Elements of Vectors
    let v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2];
    println!("The third element is {third}");

    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element."),
    }

    // let does_not_exist = &v[100]; // This will cause a panic
    let does_not_exist = v.get(100); // This will return None

    // Can not have mutable and immutable references in the same scope
    // let mut v = vec![1, 2, 3, 4, 5];
    // let first = &v[0];
    // v.push(6); // This will cause an error
    // println!("The first element is: {first}");

    // Iterating over the Values in a Vector
    let v = vec![100, 32, 57];
    for i in &v {
        println!("{}", i);
    }

    // To change the values in a mutable reference
    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
    }
    println!("{:#?}", v); // pretty print

    // Using an Enum to Store Multiple Types
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    // STRINGS
    let mut s = String::new();

    let data = "initial contents";
    let s = data.to_string();
    let s = "initial contents".to_string();
    let s = String::from("initial contents");

    // Updating a String
    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s2 is {}", s2);
    println!("s1 is {}", s1);

    let mut s = String::from("lo");
    s.push('l');
    println!("{}", s);
}
