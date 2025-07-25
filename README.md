# 🍽️ SnapFood CLI – Rust-Based Food Ordering System

**SnapFood CLI** is a simple command-line food ordering platform written in **Rust**, inspired by real-world systems like **SnappFood**. It simulates a multi-role system where **users**, **restaurant owners**, and **admins** interact via a terminal interface. All data is persisted in structured **JSON** files.

---

## 🎯 Project Goals

- Practice core Rust concepts: `struct`, `enum`, `Vec<T>`, ownership & borrowing, conditionals, loops, and file I/O
- Simulate a real-world, role-based application using CLI
- Learn modular design and JSON-based data handling

---

## 🧰 Technologies & Libraries

| Tool / Library     | Purpose                                 |
|--------------------|------------------------------------------|
| Rust               | Main language                           |
| CLI                | User interaction through terminal        |
| `serde`, `serde_json` | JSON serialization/deserialization |
| `std::fs`, `std::io`   | File operations and I/O             |

---


---

## 👥 User Roles & Features

### 👤 User
- Register and login
- Browse restaurants by category (e.g., Fastfood, Traditional)
- View restaurant menus
- Add items to cart
- Confirm and place orders
- View past order history

### 👨‍🍳 Restaurant Owner
- Login as restaurant owner
- Register a restaurant with a category
- Add/edit/delete menu items
- View received orders
- View sales history

### 🛡️ Admin
- View all registered users
- View all restaurants and categories
- Monitor all system orders
- *(Future)* Ban/remove users or restaurants for violations

---

## 🧪 Sample Data Format

### `users.json`
```json
[
  {
    "username": "ali",
    "password": "1234",
    "role": "user",
    "orders": []
  },
  {
    "username": "reza_owner",
    "password": "pass123",
    "role": "restaurant_owner",
    "restaurant": "Reza Grill"
  },
  {
    "username": "admin",
    "password": "admin123",
    "role": "admin"
  }
]


