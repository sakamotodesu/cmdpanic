# Command Line Whack-a-Mole

A simple command-line game where you catch moles (🐊) that appear in different holes. Test your reflexes and see how many moles you can catch within the time limit!

## Prerequisites

- Rust (latest stable version)
- Cargo (comes with Rust)

## Building

1. Clone this repository:
```bash
git clone <repository-url>
cd <repository-name>
```

2. Build the project:
```bash
cargo build --release
```

3. Run the game:
```bash
cargo run --release
```

## How to Play

1. When you start the game, you'll see a welcome screen with instructions
2. Press Enter to begin
3. You'll see 5 holes displayed as `[ ]` with one containing a mole `[🐊]`
4. Enter the number (1-5) corresponding to the hole where the mole appears
5. Each round lasts 2 seconds
6. The total game time is 10 seconds
7. Press ESC to exit the game

### Scoring
- +1 point for each correct catch
- No penalty for wrong guesses
- Try to get the highest score possible within the time limit!

## Controls
- Numbers 1-5: Select a hole
- ESC: Exit the game
- Enter: Start the game

## Game Rules
- Each round is 2 seconds long
- The total game time is 10 seconds
- The mole appears in a random hole each round
- The game ends when:
  - You press ESC
  - The total time limit is reached
  - The game crashes (hopefully not!)