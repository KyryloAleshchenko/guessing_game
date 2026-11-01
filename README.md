# Guessing Game

German version below ↓

## EN

My first Rust project: a console number guessing game. Built to get acquainted with the language, its core features and the Cargo package manager.

### How it works

The program picks a random number from 1 to 100. You type guesses and get a hint after each one ("Too small!" / "Too big!"). When you guess correctly, the game shows how many attempts it took.

### Requirements

- Rust and Cargo 1.85 or newer (the project uses edition 2024; tested on 1.99.1)

### Running

1. Install Rust and Cargo: https://www.rust-lang.org/tools/install
2. Clone the repository:
   ```sh
   git clone https://github.com/KyryloAleshchenko/guessing_game.git
   cd guessing_game
   ```
3. Start the game:
   ```sh
   cargo run
   ```

Note: typing anything other than a positive whole number currently ends the program with an error.

## DE

Mein erstes Rust-Projekt: ein Zahlenratespiel für die Konsole. Entstanden, um die Sprache, ihre Grundfunktionen und den Paketmanager Cargo kennenzulernen.

### Funktionsweise

Das Programm wählt eine Zufallszahl zwischen 1 und 100. Du gibst Tipps ein und bekommst nach jedem einen Hinweis („Too small!“ / „Too big!“). Wenn du richtig rätst, zeigt das Spiel an, wie viele Versuche du gebraucht hast.

### Voraussetzungen

- Rust und Cargo 1.85 oder neuer (das Projekt nutzt Edition 2024; getestet mit 1.99.1)

### Starten

1. Rust und Cargo installieren: https://www.rust-lang.org/tools/install
2. Repository klonen:
   ```sh
   git clone https://github.com/KyryloAleshchenko/guessing_game.git
   cd guessing_game
   ```
3. Spiel starten:
   ```sh
   cargo run
   ```

Hinweis: Wenn du etwas anderes als eine positive ganze Zahl eingibst, beendet sich das Programm derzeit mit einem Fehler.
