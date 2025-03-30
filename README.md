# Command Line Whack-a-Mole

[![CI](https://github.com/sakamotodesu/cmdpanic/actions/workflows/ci.yml/badge.svg)](https://github.com/sakamotodesu/cmdpanic/actions/workflows/ci.yml)

A simple command-line game where you catch moles (🐊) that appear in different holes. Test your reflexes and see how many moles you can catch within the time limit!

## Installation

### Using Homebrew (Recommended)

```bash
# Add the tap
brew tap sakamotodesu/cmdpanic https://github.com/sakamotodesu/homebrew-cmdpanic

# Install the game
brew install sakamotodesu/cmdpanic/cmdpanic
```

### Building from Source

1. Clone this repository:
```bash
git clone https://github.com/sakamotodesu/cmdpanic.git
cd cmdpanic
```

2. Build the project:
```bash
cargo build --release
```

3. Run the game:
```bash
cargo run --release
```

## Development

### Creating a New Release

1. Create and push a new tag:
```bash
# Create a new tag
git tag v0.1.0  # Replace with your version number

# Push the tag to GitHub
git push origin v0.1.0
```

2. Update the Homebrew formula:
```bash
# Download the release tarball
curl -L https://github.com/sakamotodesu/cmdpanic/archive/refs/tags/v0.1.0.tar.gz -o cmdpanic-0.1.0.tar.gz

# Calculate SHA256
shasum -a 256 cmdpanic-0.1.0.tar.gz

# Update the SHA256 in homebrew-cmdpanic repository
# Edit cmdpanic.rb and update the sha256 value
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