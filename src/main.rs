mod models;
mod handlers;
mod admin;

use std::path::Path;
use std::fs;
use std::io::{self, Write};
use console::style;
use handlers::{user_handler, restaurant_handler};
use admin::admin_handler;
use models::{User, Restaurant, Order};

fn main() {

    init_data_files();

    println!("\n{}", style("=== Welcome to SnapFood CLI ===").bold().cyan());

    loop {
        println!("\n{}", style("Main Menu:").bold().green());
        println!("1. Login");
        println!("2. Register");
        println!("3. Exit");

        let choice = get_input("Enter your choice: ");

        match choice.as_str() {
            "1" => login(),
            "2" => register(),
            "3" => {
                println!("{}", style("Goodbye!").bold().yellow());
                break;
            }
            _ => println!("{}", style("Invalid choice!").bold().red()),
        }
    }
}

fn init_data_files() {
    let data_dir = Path::new("data");
    if !data_dir.exists() {
        fs::create_dir(data_dir).expect("Failed to create data directory");
    }

    
    let users_path = data_dir.join("users.json");
    if !users_path.exists() {
        let admin = User {
            username: "admin".to_string(),
            password: "admin123".to_string(),
            role: "admin".to_string(),
            restaurant: None,
            orders: Vec::new(),
        };
        fs::write(&users_path, serde_json::to_string_pretty(&vec![admin]).unwrap())
            .expect("Failed to create users.json");
    }

    
    let files = ["restaurants.json", "orders.json"];
    for file in files.iter() {
        let path = data_dir.join(file);
        if !path.exists() {
            fs::write(&path, "[]").expect(&format!("Failed to create {}", file));
        }
    }
}
fn login() {
    println!("\n{}", style("Login").bold().green());
    let username = get_input("Username: ");
    let password = get_input("Password: ");

    let users: Vec<User> = load_data("users.json");
    if let Some(user) = users.iter().find(|u| u.username == username && u.password == password) {
        println!("\n{} {}", style("Welcome,").bold().green(), style(&username).bold().blue());
        
        match user.role.as_str() {
            "user" => user_handler(&username),
            "restaurant_owner" => restaurant_handler(&username),
            "admin" => admin_handler(),
            _ => println!("{}", style("Unknown role!").bold().red()),
        }
    } else {
        println!("{}", style("Invalid username or password!").bold().red());
    }
}

fn register() {
    println!("\n{}", style("Register").bold().green());
    let username = get_input("Username: ");
    let password = get_input("Password: ");
    let role = get_role_input();

    let mut users: Vec<User> = load_data("users.json");
    
    if users.iter().any(|u| u.username == username) {
        println!("{}", style("Username already exists!").bold().red());
        return;
    }

    let restaurant_name = if role == "restaurant_owner" {
        let name = get_input("Restaurant name: ");
        
        
        let mut restaurants: Vec<Restaurant> = load_data("restaurants.json");
        restaurants.push(Restaurant {
            name: name.clone(),
            owner: username.clone(),
            category: get_input("Restaurant category (e.g., Fastfood, Iranian): "),
            menu: Vec::new(),
        });
        save_data("restaurants.json", &restaurants);
        
        Some(name)
    } else {
        None
    };

    let new_user = User {
        username,
        password,
        role,
        restaurant: restaurant_name,
        orders: Vec::new(),
    };

    users.push(new_user);
    save_data("users.json", &users);

    println!("{}", style("Registration successful!").bold().green());
}

fn get_role_input() -> String {
    loop {
        println!("\nSelect role:");
        println!("1. User");
        println!("2. Restaurant Owner");
        println!("3. Admin (requires special permission)");

        let choice = get_input("Enter role choice: ");

        match choice.as_str() {
            "1" => return "user".to_string(),
            "2" => return "restaurant_owner".to_string(),
            "3" => {
                let admin_code = get_input("Enter admin registration code: ");
                if admin_code == "admin123" {
                    return "admin".to_string();
                } else {
                    println!("{}", style("Invalid admin code!").bold().red());
                }
            }
            _ => println!("{}", style("Invalid choice!").bold().red()),
        }
    }
}

fn get_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn load_data<T: serde::de::DeserializeOwned>(filename: &str) -> Vec<T> {
    let path = format!("data/{}", filename);
    let data = fs::read_to_string(path).unwrap_or_else(|_| "[]".to_string());
    serde_json::from_str(&data).unwrap_or_else(|_| Vec::new())
}

fn save_data<T: serde::Serialize>(filename: &str, data: &T) {
    let path = format!("data/{}", filename);
    let json = serde_json::to_string_pretty(data).unwrap();
    fs::write(path, json).unwrap();
}