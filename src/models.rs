use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    pub username: String,
    pub password: String,
    pub role: String,
    pub restaurant: Option<String>,
    #[serde(default)]
    pub orders: Vec<String>, // Order IDs
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Restaurant {
    pub name: String,
    pub owner: String,
    pub category: String,
    pub menu: Vec<MenuItem>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MenuItem {
    pub name: String,
    pub price: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Order {
    pub id: String,
    pub username: String,
    pub restaurant: String,
    pub items: Vec<OrderItem>,
    pub total_price: u32,
    pub datetime: DateTime<Utc>,
    pub status: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OrderItem {
    pub name: String,
    pub quantity: u32,
    pub price: u32,
}