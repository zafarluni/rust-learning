#[derive(Debug)]
enum MenuChoice {
    MainMenu,
    Start,
    Quit,
}

fn get_choice(input: &str) -> Result<MenuChoice, String> {
    match input {
        "mainmenu" => Ok(MenuChoice::MainMenu),
        "start" => Ok(MenuChoice::Start),
        "quit" => Ok(MenuChoice::Quit),
        _ => Err("Menu choice not found".to_owned()),
    }
}

pub fn use_fn_with_result_return_type() {
    println!("Choice : {:?}", get_choice("mainmenu"));
    println!("Choice : {:?}", get_choice("start"));
    println!("Choice : {:?}", get_choice("hello"));
    println!("Choice : {:?}", get_choice("quit"));
}
