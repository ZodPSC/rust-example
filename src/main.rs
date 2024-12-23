use chrono::prelude::*;

fn main() {
    println!("Hello!");
    println!("Current datetime {}", Utc::now());

    let mut shopping_list: Vec<String> = Vec::new();

    loop {
        print_menu();

        let command = read_input();

        match command.trim() {
            "show" => show_shopping_list(&shopping_list),
            "add" => add_shopping_list(&mut shopping_list),
            "remove" => remove_shopping_list(&mut shopping_list),
            _ => break,
        }
    }
}

fn print_menu() {
    println!();
    println!("Enter command:");
    println!("'show' - show all shopping list");
    println!("'add' - add new shopping");
    println!("'remove' - remove shopping element");
    println!("other for exit");
    println!();
}

fn read_input() -> String {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).unwrap();
    buffer
}

fn show_shopping_list(list: &Vec<String>) {
    println!("\nShopping list:");
    let mut index: usize = 0;
    for position in list {
        println!("{}: {}", index, position);
        index = index + 1;
    }
}

fn add_shopping_list(list: &mut Vec<String>) {
    println!("\nAdding shopping:");
    let input = read_input();
    let position = input.trim().to_string();
    list.push(position)
}

fn remove_shopping_list(list: &mut Vec<String>) {
    println!("\nRemoving shopping:");
    let input = read_input();
    let position = input.trim().to_string();
    list.remove(position.parse::<usize>().unwrap());
    println!("\nRemoving shopping completed.");
}