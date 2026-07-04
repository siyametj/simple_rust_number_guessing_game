# 🎯 Ultimate Rust Number Guessing Game 🦀

Welcome to the **Ultimate Number Guessing Game** built from scratch using Rust! This project is designed with a clean, modular architecture and features robust input validation, dynamic round tracking, and a smooth terminal interface experience.

---

## 🚀 Features

- **Multi-Module Architecture:** Clean code separation using Rust's module system (`main`, `input_handler`, `number_gen`, and `helper`).
- **Defensive Input Handling:** Built-in validation that rejects empty spaces, non-integer gibberish, and prevents infinite loops on EOF/interruption.
- **Smart Round Tracker:** Keeps track of your current round and consecutive wins.
- **Terminal Auto-Cleaner:** Automatically flushes and clears the screen between rounds for a native game feel.
- **Play Again Loop:** Seamless replay prompt that validates `y/yes` or `n/no` inputs like a pro.

---

## 📂 Project Structure

```text
random_number_guessing/
├── Cargo.toml
├── README.md
└── src/
    ├── main.rs          # Main game loop & orchestration
    ├── input_handler.rs # Safe integer & string input takers
    ├── number_gen.rs    # Random number generator logic
    └── helper.rs        # Terminal screen utility functions
