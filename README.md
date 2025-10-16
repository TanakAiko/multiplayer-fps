# Multiplayer FPS - Maze Game

<div align="center">

![Rust](https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white)
![Bevy](https://img.shields.io/badge/Bevy-232326?style=for-the-badge&logo=bevy&logoColor=white)
![Tokio](https://img.shields.io/badge/Tokio-00ADD8?style=for-the-badge&logo=rust&logoColor=white)

</div>

A multiplayer first-person maze game built with Rust, featuring procedurally generated mazes, real-time networking, and support for 10+ concurrent players.

</div>

## 📸 Screenshots

<div align="center">

![Player View Inside Map](imgs/Screenshot%20from%202025-10-16%2015-28-23.png)
*First-person view of a player navigating through the procedurally generated maze*

</div>

## 🎮 Features

<div align="left">

- **Multiplayer Support**: Real-time UDP-based networking for 10+ concurrent players
- **Procedural Generation**: Dynamically generated mazes with increasing difficulty
- **3D Graphics**: Built with Bevy ECS engine for smooth rendering and game logic
- **HUD System**: Real-time display of FPS, player names, mini-map, and level information
- **Client Prediction**: Latency compensation for smooth gameplay
- **Collision Detection**: Physics-based movement and obstacle interaction

## 🚀 Getting Started

### Prerequisites

- Rust (latest stable version)
- Cargo

### Installation

1. Clone the repository:
```bash
git clone https://github.com/yourusername/multiplayer-fps.git
cd multiplayer-fps
```

2. Build the project:
```bash
cargo build --release
```

### Running the Game

**Start the server:**
```bash
cargo run --bin server
```

**Start a client:**
```bash
cargo run --bin client
```

You'll be prompted to enter:
- Server IP address
- Your player username

## 📦 Dependencies

### Core Libraries

| Library | Purpose |
|---------|---------|
| **Bevy** | ECS game engine for entity management, 3D/2D rendering, and game logic |
| **Tokio** | Async runtime for high-performance UDP network communication |
| **Serde** | Data serialization/deserialization for network messages |
| **Rand** | Procedural maze generation using algorithms like Prim or Recursive Backtracking |
| **Crossbeam** | Thread-safe communication between networking and rendering subsystems |

### Optional Libraries

- **Quinn**: QUIC protocol support for enhanced UDP with reliability guarantees
- **Petgraph**: Graph analysis for complex maze generation and pathfinding

## 🏗️ Architecture

### Project Structure

```
src/
├── lib.rs                          # Library root
├── bin/
│   ├── client.rs                   # Client entry point
│   └── server.rs                   # Server entry point
├── client/
│   ├── mod.rs
│   ├── udp.rs                      # Client UDP networking
│   ├── components/                 # ECS components
│   │   ├── animation_component.rs
│   │   ├── bullet.rs
│   │   ├── camera_component.rs
│   │   ├── enemy_component.rs
│   │   ├── flag_component.rs
│   │   ├── player_component.rs
│   │   ├── rendering_component.rs
│   │   └── world_component.rs
│   ├── plugins/                    # Bevy plugins
│   │   ├── enemy_plugin.rs
│   │   ├── player_plugin.rs
│   │   └── world_plugin.rs
│   ├── resources/                  # Game resources
│   │   ├── animation_resource.rs
│   │   ├── enemy_resource.rs
│   │   ├── network_resource.rs
│   │   ├── player_resource.rs
│   │   └── world_resource.rs
│   └── systems/                    # Game systems
│       ├── camera/
│       ├── common/
│       ├── enemy/
│       ├── page/
│       ├── player/
│       └── world/
├── server/
│   ├── mod.rs
│   ├── udp.rs                      # Server UDP networking
│   └── utils/
│       └── exeption.rs
└── common/
    ├── types/
    │   ├── game_state.rs           # Shared game state
    │   └── protocol.rs             # Network protocol definitions
    └── utils/
        └── socket_utils.rs         # Socket utilities
```

### Module Responsibilities

#### 🌐 Network Module
- **Purpose**: Client-server communication via UDP
- **Responsibilities**:
  - UDP server implementation using Tokio
  - Message serialization/deserialization with Serde
  - Handling 10+ simultaneous connections
  - Packet loss management and latency optimization

#### 🎯 Game Logic Module
- **Purpose**: Core game mechanics and rules
- **Responsibilities**:
  - Entity management (players, mazes, obstacles, levels)
  - Procedural maze generation with increasing difficulty
  - Collision detection and movement mechanics
  - Win condition implementation

#### 🖼️ Graphics Module
- **Purpose**: Rendering and visual presentation
- **Responsibilities**:
  - Maze and world rendering
  - Camera synchronization with player movement
  - HUD display (FPS counter, mini-map, player name, level)
  - Real-time position updates from server state

#### 💻 Client Module
- **Purpose**: User interaction and input handling
- **Responsibilities**:
  - Command-line interface for connection setup
  - Keyboard and mouse input processing
  - Server state synchronization
  - Client-side prediction for latency compensation

#### 🖥️ Server Module
- **Purpose**: Centralized game state management
- **Responsibilities**:
  - Global game state maintenance (positions, scores, maze state)
  - Network connection management
  - Periodic state broadcasting to connected clients
  - Player session management

## 🔧 Development Guidelines

### Integration Best Practices

**Network-Graphics Consistency**
- Network module sends only essential data (player positions, maze state)
- Graphics module updates display based on received data
- Use efficient data structures to minimize bandwidth

**Client-Server Synchronization**
- Unique player IDs for state association
- Client-side prediction to mask latency
- Server authoritative for game state
- Periodic reconciliation of client predictions

**Performance & Modularity**
- Clear separation of concerns between modules
- Unit tests for each module
- Optimized maze algorithms
- Minimal computations in main game loop

### Testing Strategy

Create automated tests for:
- Network connectivity and message passing
- Maze generation algorithms
- Collision detection
- Player movement
- State synchronization

## 🤝 Contributing

Contributions are welcome! Here are some ways you can contribute:

- **Add New Features**: Implement power-ups, weapons, or game modes
- **Improve Maze Generation**: Enhance algorithms for better maze layouts and difficulty scaling
- **Optimize Networking**: Reduce latency, improve packet handling, or add TCP fallback
- **Enhance Graphics**: Add better textures, lighting effects, or visual feedback
- **Improve Collision Detection**: Fine-tune physics and movement mechanics
- **Add Tests**: Write unit and integration tests for critical systems
- **Fix Bugs**: Report and fix issues in gameplay, networking, or rendering
- **Improve Documentation**: Enhance code comments, add tutorials, or create guides

---

<div align="center">

**⭐ Star this repository if you found it helpful! ⭐**

Made with ❤️ from 🇸🇳

</div>