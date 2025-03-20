# 🦀 rust-todo  
*A simple and powerful CLI To-Do List App built with Rust!*  

🌟 **Manage your tasks efficiently with rust-todo!**  
Stay organized with task **prioritization, due dates, status filtering, and persistent storage** — all in a sleek command-line interface.  

---

## 📌 Features  
✔️ **Add tasks** with priority & due dates  
✔️ **List tasks** (with filtering for done/pending)  
✔️ **Mark tasks as done**  
✔️ **Remove tasks**  
✔️ **Persistent storage** (saves tasks in JSON)  
✔️ **Color-coded UI** for better readability  

---

## 🎯 What I Learned  
Building `rust-todo` taught me:  
✅ How to create a **command-line application** in Rust  
✅ How to use **Structs & Enums** effectively  
✅ How to handle **file I/O** and store data persistently  
✅ How to work with **external crates** like `serde`, `clap`, `chrono`, and `colored`  
✅ How to format and **display terminal output beautifully**  

---

## 🛠 Installation  

### 1️⃣ Clone the repository  
```sh
git clone https://github.com/yourusername/rust-todo.git
cd rust-todo
```

### 2️⃣ Install Rust (if not already installed)  
```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### 3️⃣ Build the project  
```sh
cargo build
```

---

## 🚀 Usage  

### ➕ **Add a task**  
```sh
cargo run -- add "Complete Rust project" high "2025-03-25"
```
📌 *If no due date is provided, today's date is used.*  
```sh
cargo run -- add "Read a book" medium
```

### 📋 **List all tasks**  
```sh
cargo run -- list
```
*Filter by status:*  
```sh
cargo run -- list --status done
cargo run -- list --status pending
```

### ✅ **Mark a task as done**  
```sh
cargo run -- done 1
```

### ❌ **Remove a task**  
```sh
cargo run -- remove 1
```

---

## ⚙️ Technologies Used  
🦀 **Rust** – Core programming language  
📦 `serde` – JSON Serialization  
📝 `clap` – Command-line Argument Parser  
📅 `chrono` – Date & Time Handling  
🎨 `colored` – Beautiful Terminal Output  

---

## 🚀 Future Enhancements  
🔹 Add **edit task** functionality  
🔹 Implement **recurring tasks**  
🔹 Store tasks in **SQLite/PostgreSQL** for database integration  

---

✨ **Built with ❤️ and Rust!** 🦀  
