use console::style;
use crate::{get_input, load_data, save_data};
use crate::models::{User, Restaurant, Order};

pub fn admin_handler() {
    println!("\n{}", style("Admin Panel").bold().green());
    
    loop {
        println!("\n{}", style("Options:").bold().green());
        println!("1. View All Users");
        println!("2. View All Restaurants");
        println!("3. View All Orders");
        println!("4. Block/Unblock User");
        println!("5. Delete Restaurant");
        println!("6. Logout");

        let choice = get_input("Enter your choice: ");

        match choice.as_str() {
            "1" => view_all_users(),
            "2" => view_all_restaurants(),
            "3" => view_all_orders(),
            "4" => block_user(),
            "5" => delete_restaurant(),
            "6" => break,
            _ => println!("{}", style("Invalid choice!").bold().red()),
        }
    }
}

fn view_all_users() {
    let users: Vec<User> = load_data("users.json");
    
    println!("\n{}", style("All Users:").bold().green());
    for user in users {
        println!("\n{}: {}", style("Username").bold().green(), user.username);
        println!("{}: {}", style("Role").bold().green(), user.role);
        if let Some(restaurant) = user.restaurant {
            println!("{}: {}", style("Restaurant").bold().green(), restaurant);
        }
        println!("{}: {}", style("Order Count").bold().green(), user.orders.len());
    }
}

fn view_all_restaurants() {
    let restaurants: Vec<Restaurant> = load_data("restaurants.json");
    
    println!("\n{}", style("All Restaurants:").bold().green());
    for restaurant in restaurants {
        println!("\n{}: {}", style("Name").bold().green(), restaurant.name);
        println!("{}: {}", style("Owner").bold().green(), restaurant.owner);
        println!("{}: {}", style("Category").bold().green(), restaurant.category);
        println!("{}: {}", style("Menu Items").bold().green(), restaurant.menu.len());
    }
}

fn view_all_orders() {
    let orders: Vec<Order> = load_data("orders.json");
    
    println!("\n{}", style("All Orders:").bold().green());
    for order in orders {
        println!("\n{}: {}", style("Order ID").bold().green(), order.id);
        println!("{}: {}", style("Customer").bold().green(), order.username);
        println!("{}: {}", style("Restaurant").bold().green(), order.restaurant);
        println!("{}: {}", style("Date").bold().green(), order.datetime);
        println!("{}: {}", style("Status").bold().green(), order.status);
        println!("{}: {} Toman", style("Total").bold().green(), order.total_price);
        println!("{}", style("----------------").bold().cyan());
    }
}

fn block_user() {
    let mut users: Vec<User> = load_data("users.json");
    view_all_users();
    
    let username = get_input("\nEnter username to block/unblock: ");
    if let Some(user) = users.iter_mut().find(|u| u.username == username) {
        if user.role == "admin" {
            println!("{}", style("Cannot block admin user!").bold().red());
            return;
        }
        
        if user.role.starts_with("blocked_") {
            user.role = user.role.replace("blocked_", "");
            println!("{}", style("User unblocked successfully!").bold().green());
        } else {
            user.role = format!("blocked_{}", user.role);
            println!("{}", style("User blocked successfully!").bold().green());
        }
        
        save_data("users.json", &users);
    } else {
        println!("{}", style("User not found!").bold().red());
    }
}

fn delete_restaurant() {
    let mut restaurants: Vec<Restaurant> = load_data("restaurants.json");
    view_all_restaurants();
    
    let name = get_input("\nEnter restaurant name to delete: ");
    if let Some(index) = restaurants.iter().position(|r| r.name == name) {
        restaurants.remove(index);
        save_data("restaurants.json", &restaurants);
        
        
        let mut users: Vec<User> = load_data("users.json");
        for user in users.iter_mut() {
            if let Some(restaurant) = &user.restaurant {
                if restaurant == &name {
                    user.restaurant = None;
                }
            }
        }
        save_data("users.json", &users);
        
        println!("{}", style("Restaurant deleted successfully!").bold().green());
    } else {
        println!("{}", style("Restaurant not found!").bold().red());
    }
}