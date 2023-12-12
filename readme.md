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

   The game should launch in a new window. Have fun!

<p align="center">
  <img src="resources/showcase-commit14.gif" />
</p>

## Controls

|   Key   |         Description         |
| ------- | --------------------------- |
|  [UP]   | Move/attack north direction |
| [DOWN]  | Move/attack south direction |
| [LEFT]  | Move/attack west direction  |
| [RIGHT] | Move/attack east direction  |
|   [G]   | Pickup item                 |
| [1 - 9] | Use item                    | 
| [SPACE] | Skip turn                   |

## To Do
- [X] Create a basic dungeon map
- [X] Place the player and let them walk around
- [X] Spawn monsters, draw them, and let the player kill them by walking into them
- [X] Add turn-based system 
- [X] Add health and a combat system that uses it
- [X] Display a "game over" screen when the player dies
- [X] Add the Amulet of Yala to the level and let the player win by reaching it
- [X] Add Fields-of-View
- [X] Add more interesting dungeon designs
- [X] Add some dungeon themes
- [X] Add healing potions
- [X] Add multiple layers to the dungeon, with the Amulet on the last one
- [X] Add varied weapons to the game
- [X] Move to a data-driven design for spawning enemies
---
- [ ] add window and door tiles
- [ ] refine FOV to more accurately reflect LOS
- [ ] restrict map theme usage to specific map builder algorithms
- [ ] Consider some visual effects to make combat more visceral
- [ ] Consider keeping score

## Bugs
- Monsters can enter the same tile if they move into it at the same time e.g. tile is empty so considered a valid destination, so both monsters move into it simultaneously

### Credits
The dungeon floor, wall, and adventurer graphics were kindly provided by Buch for free, [here](https://opengameart.org/content/unfinished-dungeon-tileset). The potion and scroll graphics are from Melissa Krautheim’s [Fantasy Magic Set](https://opengameart.org/content/fantasy-magic-set). Weaponry is from Melle’s [Fantasy Sword Set](https://opengameart.org/content/fantasy-sword-set). Monster graphics are from the game Dungeon Crawl Stone Soup (CC0 license), packaged by Chris Hamons, [here](https://github.com/crawl/tiles).