#[derive(Debug)]
struct Person {
    name: String,
    age: Option<u32>,
}

pub fn use_option_with_struct_values() {
    let p1 = Person {
        name: "Zafar".to_owned(),
        age: Some(38),
    };

    let p2 = Person {
        name: "Sajjad".to_owned(),
        age: None,
    };

    match_person(&p1);
    match_person(&p2);
}

fn match_person(p: &Person) {
    println!("\nWelcome Mr. {} ", p.name);
    match p.age {
        Some(a) => println!("Beautiful {} years.", a),
        None => println!("No age data available."),
    }
}
