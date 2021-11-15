#[derive(Debug)]
struct User {
    user_id: i32,
    name: String,
}

fn find_user(name: &str) -> Option<i32> {
    let name = name.to_lowercase();

    match name.as_str() {
        "sam" => Some(1),
        "matt" => Some(5),
        "katie" => Some(9),
        _ => None,
    }
}

pub fn find_user_with_cloures() {
    let user_name = "sam";
    let user1 = find_user(user_name).map(|user_id| User {
        user_id,
        name: user_name.to_owned(),
    });

    println!("User: {:?}", user1);

    let user_name = "matt";
    let user1 = find_user(user_name).map(|user_id| User {
        user_id,
        name: user_name.to_owned(),
    });

    println!("User: {:?}", user1);

    let user_name = "zafar";
    let user1 = find_user(user_name).map(|user_id| User {
        user_id,
        name: user_name.to_owned(),
    });

    println!("User: {:?}", user1);
}

pub fn test_combanitorics() {
    let vect = vec![1, 2, 3, 4, 5];
    let result: Vec<_> = vect.iter().map(|x| x * 2).filter(|x| x > &6).collect();
    println!("values: {:?}", result);
}
