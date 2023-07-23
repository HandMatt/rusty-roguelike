# Rusty Roguelike

Welcome to the "Rusty Roguelike" Rust project! This project is a roguelike dungeon crawler game built using the Rust game programming library *bracket-lib*.

## Prerequisites

Before you proceed, make sure you have the following installed on your system:

- Rust programming language: You can download and install Rust from the official website at [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install).

## Getting Started

To get started with this project, follow the steps below:

1. Clone the repository or download the project files to your local machine.

2. Open a terminal or command prompt and navigate to the project's directory.

3. Build the project by executing the following command:

   ```shell
   cargo build
   ```

   This command will compile the Rust code and create an executable binary.

4. Run the program using the following command:

   ```shell
   cargo run
   ```

   The game should launch in a new window, have fun!

<p align="center">
  <img src="" />
</p>

## Controls

|   Key   |    Description    |
| ------- | ----------------- |
|  [UP]   | Move player north |
| [DOWN]  | Move player south |
| [LEFT]  | Move player west  |
| [RIGHT] | Move player east  |

## To Do
- [X] Create a basic dungeon map
- [X] Place the player and let them walk around
- [ ] Spawn monsters, draw them, and let the player kill them by walking into them
- [ ] Add health and a combat system that uses it
- [ ] Add healing potions
- [ ] Display a "game over" screen when the player dies
- [ ] Add the Amulet of Yala to the level and let the player win by reaching it
---
- [ ] Add Fields-of-View
- [ ] Add more interesting dungeon designs
- [ ] Add some dungeon themes
- [ ] Add multiple layers to the dungeon, with the Amulet on the last one
- [ ] Add varied weapons to the game
- [ ] Move to a data-driven design for spawning enemies
- [ ] Consider some visual effects to make combat more visceral
- [ ] Consider keeping score


### Credits
The dungeon floor, wall, and adventurer graphics were kindly provided by Buch for free, [here](https://opengameart.org/content/unfinished-dungeon-tileset). The potion and scroll graphics are from Melissa Krautheim’s [Fantasy Magic Set](https://opengameart.org/content/fantasy-magic-set). Weaponry is from Melle’s [Fantasy Sword Set](https://opengameart.org/content/fantasy-sword-set). Monster graphics are from the game Dungeon Crawl Stone Soup (CC0 license), packaged by Chris Hamons, [here](https://github.com/crawl/tiles).