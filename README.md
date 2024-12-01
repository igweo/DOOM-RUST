## DOOM.rs: A Rust Implementation of DOOM

---

### Features

This implementation replicates key features of the original DOOM engine:

- Rendering of 2.5D environments using a BSP-based level structure.
- Full support for WAD files to load original DOOM assets and levels.
- Sprite-based rendering for enemies, items, and weapons.
- Physics simulation for movement, collision detection, and projectiles.
- Audio playback for both music and sound effects.
- AI for enemy behaviors, pathfinding, and attack patterns.
- Experimental networked multiplayer support (work-in-progress).
- Modular architecture for customization and modern enhancements.

---

### Requirements

To build and run DOOM.rs, the following are required:

- **Rust**: A stable version of the Rust compiler and Cargo.
- **SDL2**: For windowing, input handling, and audio playback.
- **WAD Files**: The original DOOM or DOOM II WAD files are necessary to load assets and levels.

You can install SDL2 via your system's package manager or from [SDL2's official website](https://www.libsdl.org/).

---

### Installation

1. Install dependencies:

   Ensure Rust and Cargo are installed. Use your system’s package manager or manual installation to set up SDL2.

2. Build the project:

   ```
   cargo build --release
   ```

3. Run the game with a WAD file:

   ```
   cargo run --release -- --wad path/to/doom.wad
   ```

Replace `path/to/doom.wad` with the path to a valid WAD file.

---

### Usage

You can pass the following runtime options to the executable:

- `--wad <path>`: Specifies the WAD file to load (required).
- `--fullscreen`: Runs the game in fullscreen mode.
- `--resolution <width>x<height>`: Sets a custom resolution.
- `--debug`: Enables debug rendering and logging.

Example command:

```
cargo run --release -- --wad ./DOOM.WAD --fullscreen --resolution 1920x1080
```

---

### Controls

- **WASD**: Move forward/backward and strafe.
- **Arrow Keys**: Turn or strafe (alternative).
- **Space**: Interact with doors, switches, and other objects.
- **Ctrl**: Shoot weapon.
- **Shift**: Sprint.
- **ESC**: Pause the game or quit.

---

### Project Structure

The project is organized into modular subsystems for clarity and scalability:

- **`engine/`**: Core game engine logic, including physics, input handling, and event systems.
- **`renderer/`**: Implements BSP traversal, raycasting, and sprite-based rendering.
- **`wad/`**: Responsible for parsing WAD files and extracting level, texture, and sprite data.
- **`audio/`**: Handles sound effects and music playback using a Rust audio backend.
- **`game/`**: Manages player mechanics, enemy AI, and game state transitions.
- **`network/`**: (Optional) Implements experimental networked multiplayer logic.
- **`util/`**: Shared utilities such as random number generators and lookup tables.

---

### License

DOOM.rs is licensed under the GNU General Public License (GPL). Refer to the LICENSE file for details. Note that using original WAD files requires adherence to their respective copyright and licensing terms.
