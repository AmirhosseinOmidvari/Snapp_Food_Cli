use std::io::{self, Write};
use console::style;
use chrono::Utc;
use rand::Rng;
use crate::models::{User, Restaurant, Order, OrderItem, MenuItem};
use crate::{get_input, load_data, save_data};

pub fn user_handler(username: &str) {
    println!("\n{} {}", style("User Panel:").bold().green(), style(username).bold().blue());
    
    loop {
        println!("\n{}", style("Options:").bold().green());
        println!("1. View Restaurants");
        println!("2. View Menu");
        println!("3. Add to Cart");
        println!("4. View Cart");
        println!("5. Place Order");
        println!("6. View Order History");
        println!("7. Logout");

        let choice = get_input("Enter your choice: ");

        match choice.as_str() {
            "1" => view_restaurants(),
            "2" => view_menu(),
            "3" => add_to_cart(username),
            "4" => view_cart(username),
            "5" => place_order(username),
            "6" => view_order_history(username),
            "7" => break,
            _ => println!("{}", style("Invalid choice!").bold().red()),
        }
    }
}

pub fn restaurant_handler(username: &str) {
    let users: Vec<User> = load_data("users.json");
    let user = users.iter().find(|u| u.username == username).unwrap();
    let restaurant_name = user.restaurant.as_ref().unwrap();
    
    println!("\n{} {} {}", style("Restaurant Owner Panel:").bold().green(), 
             style(username).bold().blue(), 
             style(format!("({})", restaurant_name)).bold().cyan());
    
    loop {
        println!("\n{}", style("Options:").bold().green());
        println!("1. View Restaurant Info");
        println!("2. Add Menu Item");
        println!("3. Edit Menu Item");
        println!("4. Remove Menu Item");
        println!("5. View Current Orders");
        println!("6. View Order History");
        println!("7. Logout");

        let choice = get_input("Enter your choice: ");

        match choice.as_str() {
            "1" => view_restaurant_info(restaurant_name),
            "2" => add_menu_item(restaurant_name),
            "3" => edit_menu_item(restaurant_name),
            "4" => remove_menu_item(restaurant_name),
            "5" => view_current_orders(restaurant_name),
            "6" => view_restaurant_order_history(restaurant_name),
            "7" => break,
            _ => println!("{}", style("Invalid choice!").bold().red()),
        }
    }
}

fn view_restaurants() {
    let restaurants: Vec<Restaurant> = load_data("restaurants.json");
    
    println!("\n{}", style("Available Restaurants:").bold().green());
    for (i, restaurant) in restaurants.iter().enumerate() {
        println!("{}. {} ({})", i + 1, style(&restaurant.name).bold().blue(), &restaurant.category);
    }
}

fn view_menu() {
    let restaurants: Vec<Restaurant> = load_data("restaurants.json");
    view_restaurants();
    
    let choice = get_input("\nSelect a restaurant to view menu: ");
    if let Ok(index) = choice.parse::<usize>() {
        if index > 0 && index <= restaurants.len() {
            let restaurant = &restaurants[index - 1];
            println!("\n{} {}", style("Menu for").bold().green(), style(&restaurant.name).bold().blue());
            
            for (i, item) in restaurant.menu.iter().enumerate() {
                println!("{}. {} - {} Toman", i + 1, style(&item.name).bold().cyan(), item.price);
            }
        } else {
            println!("{}", style("Invalid selection!").bold().red());
        }
    } else {
        println!("{}", style("Please enter a valid number!").bold().red());
    }
}

fn add_to_cart(username: &str) {
    let mut restaurants: Vec<Restaurant> = load_data("restaurants.json");
    view_restaurants();
    
    let rest_choice = get_input("\nSelect a restaurant: ");
    if let Ok(rest_index) = rest_choice.parse::<usize>() {
        if rest_index > 0 && rest_index <= restaurants.len() {
            let restaurant = &restaurants[rest_index - 1];
            println!("\n{}", style("Menu Items:").bold().green());
            
            for (i, item) in restaurant.menu.iter().enumerate() {
                println!("{}. {} - {} Toman", i + 1, style(&item.name).bold().cyan(), item.price);
            }
            
            let item_choice = get_input("\nSelect an item: ");
            if let Ok(item_index) = item_choice.parse::<usize>() {
                if item_index > 0 && item_index <= restaurant.menu.len() {
                    let quantity = get_input("Enter quantity: ");
                    if let Ok(qty) = quantity.parse::<u32>() {
                        let item = &restaurant.menu[item_index - 1];
                        
                        
                        let mut orders: Vec<Order> = load_data("orders.json");
                        let cart = orders.iter_mut().find(|o| o.username == username && o.status == "cart");
                        
                        if let Some(cart) = cart {
                            
                            if let Some(cart_item) = cart.items.iter_mut().find(|i| i.name == item.name) {
                                cart_item.quantity += qty;
                            } else {
                                cart.items.push(OrderItem {
                                    name: item.name.clone(),
                                    quantity: qty,
                                    price: item.price,
                                });
                            }
                            cart.total_price = cart.items.iter().map(|i| i.price * i.quantity).sum();
                        } else {
                            
                            let new_cart = Order {
                                id: generate_id(),
                                username: username.to_string(),
                                restaurant: restaurant.name.clone(),
                                items: vec![OrderItem {
                                    name: item.name.clone(),
                                    quantity: qty,
                                    price: item.price,
                                }],
                                total_price: item.price * qty,
                                datetime: Utc::now(),
                                status: "cart".to_string(),
                            };
                            orders.push(new_cart);
                        }
                        
                        save_data("orders.json", &orders);
                        println!("{}", style("Item added to cart!").bold().green());
                    } else {
                        println!("{}", style("Invalid quantity!").bold().red());
                    }
                } else {
                    println!("{}", style("Invalid item selection!").bold().red());
                }
            } else {
                println!("{}", style("Please enter a valid number!").bold().red());
            }
        } else {
            println!("{}", style("Invalid restaurant selection!").bold().red());
        }
    } else {
        println!("{}", style("Please enter a valid number!").bold().red());
    }
}

fn view_cart(username: &str) {
    let orders: Vec<Order> = load_data("orders.json");
    if let Some(cart) = orders.iter().find(|o| o.username == username && o.status == "cart") {
        println!("\n{} {}", style("Your Cart:").bold().green(), style(&cart.restaurant).bold().blue());
        
        for (i, item) in cart.items.iter().enumerate() {
            println!("{}. {} x {} - {} Toman each ({} Toman total)", 
                     i + 1, 
                     style(&item.name).bold().cyan(), 
                     item.quantity, 
                     item.price, 
                     item.price * item.quantity);
        }
        
        println!("\n{}: {} Toman", style("Total Price").bold().green(), style(cart.total_price).bold().yellow());
    } else {
        println!("{}", style("Your cart is empty!").bold().yellow());
    }
}

fn place_order(username: &str) {
    let mut orders: Vec<Order> = load_data("orders.json");
    if let Some(cart_index) = orders.iter().position(|o| o.username == username && o.status == "cart") {
        let mut cart = orders.remove(cart_index);
        
        if cart.items.is_empty() {
            println!("{}", style("Your cart is empty!").bold().yellow());
            orders.push(cart);
            save_data("orders.json", &orders);
            return;
        }
        
        println!("\n{}", style("Order Summary:").bold().green());
        view_cart(username);
        
        let confirm = get_input("\nConfirm order? (y/n): ");
        if confirm.to_lowercase() == "y" {
            
            let order_id = cart.id.clone();
            
            cart.status = "pending".to_string();
            cart.datetime = Utc::now();
            orders.push(cart);
            
            
            let mut users: Vec<User> = load_data("users.json");
            if let Some(user) = users.iter_mut().find(|u| u.username == username) {
                user.orders.push(order_id);
            }
            
            save_data("orders.json", &orders);
            save_data("users.json", &users);
            
            println!("{}", style("Order placed successfully!").bold().green());
        } else {
            orders.push(cart);
            save_data("orders.json", &orders);
            println!("{}", style("Order canceled.").bold().yellow());
        }
    } else {
        println!("{}", style("Your cart is empty!").bold().yellow());
    }
}

fn view_order_history(username: &str) {
    let users: Vec<User> = load_data("users.json");
    let orders: Vec<Order> = load_data("orders.json");
    
    if let Some(user) = users.iter().find(|u| u.username == username) {
        println!("\n{}", style("Your Order History:").bold().green());
        
        let user_orders: Vec<&Order> = orders.iter()
            .filter(|o| user.orders.contains(&o.id))
            .collect();
            
        if user_orders.is_empty() {
            println!("{}", style("No orders found!").bold().yellow());
            return;
        }
        
        for order in user_orders {
            println!("\n{}: {}", style("Order ID").bold().green(), order.id);
            println!("{}: {}", style("Restaurant").bold().green(), order.restaurant);
            println!("{}: {}", style("Date").bold().green(), order.datetime);
            println!("{}: {}", style("Status").bold().green(), order.status);
            
            println!("\n{}", style("Items:").bold().green());
            for item in &order.items {
                println!("- {} x {} ({} Toman each)", item.name, item.quantity, item.price);
            }
            
            println!("\n{}: {} Toman", style("Total").bold().green(), order.total_price);
            println!("{}", style("----------------").bold().cyan());
        }
    }
}

fn view_restaurant_info(restaurant_name: &str) {
    let restaurants: Vec<Restaurant> = load_data("restaurants.json");
    
    if let Some(restaurant) = restaurants.iter().find(|r| r.name == restaurant_name) {
        println!("\n{} {}", style("Restaurant Info:").bold().green(), style(restaurant_name).bold().blue());
        println!("{}: {}", style("Owner").bold().green(), restaurant.owner);
        println!("{}: {}", style("Category").bold().green(), restaurant.category);
        
        println!("\n{}", style("Menu:").bold().green());
        for item in &restaurant.menu {
            println!("- {}: {} Toman", item.name, item.price);
        }
    } else {
        println!("{}", style("Restaurant not found!").bold().red());
    }
}

fn add_menu_item(restaurant_name: &str) {
    let mut restaurants: Vec<Restaurant> = load_data("restaurants.json");
    
    if let Some(restaurant) = restaurants.iter_mut().find(|r| r.name == restaurant_name) {
        let name = get_input("Item name: ");
        let price = get_input("Item price (Toman): ");
        
        if let Ok(price) = price.parse::<u32>() {
            restaurant.menu.push(MenuItem { name, price });
            save_data("restaurants.json", &restaurants);
            println!("{}", style("Menu item added successfully!").bold().green());
        } else {
            println!("{}", style("Invalid price!").bold().red());
        }
    } else {
        println!("{}", style("Restaurant not found!").bold().red());
    }
}

fn edit_menu_item(restaurant_name: &str) {
    let mut restaurants: Vec<Restaurant> = load_data("restaurants.json");
    
    if let Some(restaurant) = restaurants.iter_mut().find(|r| r.name == restaurant_name) {
        println!("\n{}", style("Current Menu:").bold().green());
        for (i, item) in restaurant.menu.iter().enumerate() {
            println!("{}. {}: {} Toman", i + 1, item.name, item.price);
        }
        
        let choice = get_input("\nSelect item to edit: ");
        if let Ok(index) = choice.parse::<usize>() {
            if index > 0 && index <= restaurant.menu.len() {
                let new_name = get_input(&format!("New name [{}]: ", restaurant.menu[index - 1].name));
                let new_price = get_input(&format!("New price [{}]: ", restaurant.menu[index - 1].price));
                
                if !new_name.is_empty() {
                    restaurant.menu[index - 1].name = new_name;
                }
                
                if !new_price.is_empty() {
                    if let Ok(price) = new_price.parse::<u32>() {
                        restaurant.menu[index - 1].price = price;
                    } else {
                        println!("{}", style("Invalid price!").bold().red());
                        return;
                    }
                }
                
                save_data("restaurants.json", &restaurants);
                println!("{}", style("Menu item updated successfully!").bold().green());
            } else {
                println!("{}", style("Invalid selection!").bold().red());
            }
        } else {
            println!("{}", style("Please enter a valid number!").bold().red());
        }
    } else {
        println!("{}", style("Restaurant not found!").bold().red());
    }
}

fn remove_menu_item(restaurant_name: &str) {
    let mut restaurants: Vec<Restaurant> = load_data("restaurants.json");
    
    if let Some(restaurant) = restaurants.iter_mut().find(|r| r.name == restaurant_name) {
        println!("\n{}", style("Current Menu:").bold().green());
        for (i, item) in restaurant.menu.iter().enumerate() {
            println!("{}. {}: {} Toman", i + 1, item.name, item.price);
        }
        
        let choice = get_input("\nSelect item to remove: ");
        if let Ok(index) = choice.parse::<usize>() {
            if index > 0 && index <= restaurant.menu.len() {
                restaurant.menu.remove(index - 1);
                save_data("restaurants.json", &restaurants);
                println!("{}", style("Menu item removed successfully!").bold().green());
            } else {
                println!("{}", style("Invalid selection!").bold().red());
            }
        } else {
            println!("{}", style("Please enter a valid number!").bold().red());
        }
    } else {
        println!("{}", style("Restaurant not found!").bold().red());
    }
}

fn view_current_orders(restaurant_name: &str) {
    let orders: Vec<Order> = load_data("orders.json");
    let current_orders: Vec<&Order> = orders.iter()
        .filter(|o| o.restaurant == restaurant_name && o.status == "pending")
        .collect();
    
    println!("\n{} {}", style("Current Orders for").bold().green(), style(restaurant_name).bold().blue());
    
    if current_orders.is_empty() {
        println!("{}", style("No current orders!").bold().yellow());
        return;
    }
    
    for order in current_orders {
        println!("\n{}: {}", style("Order ID").bold().green(), order.id);
        println!("{}: {}", style("Customer").bold().green(), order.username);
        println!("{}: {}", style("Date").bold().green(), order.datetime);
        
        println!("\n{}", style("Items:").bold().green());
        for item in &order.items {
            println!("- {} x {} ({} Toman each)", item.name, item.quantity, item.price);
        }
        
        println!("\n{}: {} Toman", style("Total").bold().green(), order.total_price);
        println!("{}", style("----------------").bold().cyan());
    }
}

fn view_restaurant_order_history(restaurant_name: &str) {
    let orders: Vec<Order> = load_data("orders.json");
    let restaurant_orders: Vec<&Order> = orders.iter()
        .filter(|o| o.restaurant == restaurant_name && o.status != "pending" && o.status != "cart")
        .collect();
    
    println!("\n{} {}", style("Order History for").bold().green(), style(restaurant_name).bold().blue());
    
    if restaurant_orders.is_empty() {
        println!("{}", style("No order history!").bold().yellow());
        return;
    }
    
    for order in restaurant_orders {
        println!("\n{}: {}", style("Order ID").bold().green(), order.id);
        println!("{}: {}", style("Customer").bold().green(), order.username);
        println!("{}: {}", style("Date").bold().green(), order.datetime);
        println!("{}: {}", style("Status").bold().green(), order.status);
        
        println!("\n{}", style("Items:").bold().green());
        for item in &order.items {
            println!("- {} x {} ({} Toman each)", item.name, item.quantity, item.price);
        }
        
        println!("\n{}: {} Toman", style("Total").bold().green(), order.total_price);
        println!("{}", style("----------------").bold().cyan());
    }
}

fn generate_id() -> String {
    rand::thread_rng()
        .sample_iter(&rand::distributions::Alphanumeric)
        .take(10)
        .map(char::from)
        .collect()
}