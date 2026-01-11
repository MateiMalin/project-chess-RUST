This README is designed to be professional, technical, and clear. It highlights the architecture of your project and the technologies used, which will look great to anyone reviewing your code.

---

# ♟️ Rust Chess: GUI & Engine

A fully functional, real-time Chess application built with **Rust** using the **Iced** GUI framework and the **Rodio** audio library. This project demonstrates a clean implementation of the Model-View-Update (MVU) architecture, asynchronous sound handling, and precise game state management.

## 🚀 Features

* **Real-time Interaction:** Smooth piece selection and movement with legal move highlighting.
* **Game Engine Logic:** Full support for chess rules, including castling, captures, and checkmate detection.
* **Timed Gameplay:** Dual clocks for White and Black that update every 100ms.
* **Audio Feedback:** Asynchronous sound system for moves, captures, checks, and illegal actions.
* **Move History:** A scrollable log of all moves made during the session in algebraic notation.
* **Captured Pieces:** Visual tracking of captured pieces for both players.
* **Dynamic UI:** Responsive layout that adapts to window resizing.

---

## 🏗️ Architecture

The project follows the **Elm Architecture (Model-View-Update)**, ensuring a strict one-way data flow:

1. **Model (`ChessState`):** Holds the "Source of Truth," including the board state, timers, and move history.
2. **View (`view` function):** A pure function that transforms the current state into a visual interface using Iced widgets.
3. **Update (`update` function):** Processes `Messages` (like clicks or timer ticks) to produce a new state.

---

## 🛠️ Tech Stack

* **Language:** [Rust](https://www.rust-lang.org/) (Memory-safe, high-performance)
* **GUI Framework:** [Iced](https://iced.rs/) (Type-safe, cross-platform UI)
* **Audio:** [Rodio](https://github.com/RustAudio/rodio) (Low-level audio playback)
* **Concurrency:** Standard library `std::thread` for non-blocking audio processing.

---

## 📂 Project Structure

```text
├── src/
│   ├── main.rs          # Entry point
│   ├── engine/          # Chess logic (Board, Pieces, Rules)
│   └── gui.rs           # UI logic (View, Update, Subscriptions)
├── assets/              # Piece sprites (images)
├── sound_assets/        # Audio files (mp3)
└── Cargo.toml           # Dependencies and metadata

```

---

## 🎮 How to Play

### 1. Installation

Ensure you have the Rust toolchain installed. Clone the repository and run:

```bash
cargo run --release

```

### 2. Controls

* **Select a Piece:** Click on any of your pieces to see valid moves (highlighted in green).
* **Make a Move:** Click on a highlighted square to move the selected piece.
* **Switch Selection:** Click on another of your pieces to change your selection.
* **Restart:** Use the "Restart Game" button to clear the board and reset timers.

---

## 🧠 Key Technical Challenges Solved

### Non-Blocking Audio

To prevent the UI from stuttering when a sound is played, the audio engine runs on a dedicated background thread. This ensures that disk I/O and audio decoding never interrupt the main rendering loop.

### Coordinate Mapping

The board is represented internally as a 1D array of 64 squares. The GUI maps these to an 8x8 grid, handling the coordinate inversion (Chess ranks start from the bottom, while UI coordinates start from the top) using functional Rust iterators.

### Precise Timing

The game uses Iced `Subscriptions` to listen to a 100ms "heartbeat." This allows for high-precision timers without the overhead of a continuous `while` loop, keeping CPU usage low.

---
