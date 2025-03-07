// Struct
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

// Tuple Struct
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

// Unit-like struct
struct AlwaysEqual;

fn main() {
    let mut user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("8oPfK@example.com"),
        sign_in_count: 1,
    };

    user1.email = String::from("anotheremail@example.com");
    println!("user1.email: {}", user1.email);

    let new_user: User = build_user(
        String::from("hellow@example.com"),
        String::from("helloworld"),
    );
    println!("new_user.email: {}", new_user.username);

    let user2 = User {
        email: String::from("anotheremail@example.com"),
        ..user1
    };
    println!("user2.username: {}", user2.username);
    //borrow of moved value: `user1.username`
    //move occurs because `user1.username` has type `String`, which does not implement the `Copy` trait
    // println!("user1.username moved: {}", user1.username);

    // Color != Point
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    println!("black.0: {}", black.0);
    println!("origin.0: {}", origin.0);

    let subject = AlwaysEqual;
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}
