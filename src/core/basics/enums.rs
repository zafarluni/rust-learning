#[derive(Debug)]
enum PromoDiscount {
    NewUser,
    Holiday(String),
}

enum Discount {
    Percent(f64),
    Flat(i32),
    Promo(PromoDiscount),
    Custom(String)
}

pub fn complex_enum() {
    let dis = Discount::Promo(PromoDiscount::Holiday("Welcome...".to_owned()));

    match dis {
        Discount::Percent(p) => println!("{} percent discount.",p),
        Discount::Flat(f) => println!("Flat {} usd.",f),
        Discount::Promo(promo) => println!("Holiday: {:?}", promo),
        Discount::Custom(c) => println!("Custom discount {}",c),        
    }
}