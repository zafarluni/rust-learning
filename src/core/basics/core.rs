pub fn ranges() {
    for num in 1..4 {
        println!("{}", &num);
    }

    println!("-------------------------");

    for ch in 'a'..='z' {
        println!("{}", ch);
    }
}

pub fn if_let() {
    enum Color {
        Red,
        Blue,
        Green,
    }

    let x = Color::Green;

    if let Color::Red = x {
        println!("It's red");
    } else {
        println!("It's not red");
    }
}

pub fn while_let(){
    let mut data = Some(3);

    while let Some(i) = data {
        println!("loop");
        data = None;
    }

    let vec = vec![1,2,3];

    let mut number_iter = vec.iter();

    while let Some(x) = number_iter.next() {
        println!("Number: {}", x);
    }
    
}
