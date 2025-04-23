# 👾 Space Invaders Clone (Bevy + Rust)

A modern, Rust-powered reimagining of the classic **Space Invaders** arcade game — built using the [Bevy](https://bevyengine.org/) game engine. This is a 2D, pixel-art style shooter where you control a cannon to blast alien invaders, dodge incoming fire, and aim for a high score.

## 🚀 Introduction

This is a work-in-progress retro-style arcade shooter inspired by the legendary **Space Invaders**. Built from scratch in **Rust** using the **Bevy ECS game engine**, the project focuses on simplicity, code clarity, and expandability.

It's ideal for anyone learning Bevy or building their first custom gameplay loop in Rust. The core systems like movement, collision, HUD, scoring, and event-based architecture are all designed with readability and modularity in mind.

## 🛠️ Technical Details

- **Language**: Rust 🦀
- **Engine**: [Bevy](https://bevyengine.org/) (Entity Component System, data-driven, parallel-friendly)
- **Graphics**: Simple 2D Sprites
- **Architecture**: Modular systems with event-driven HUD and game state handling

## ⚙️ Mechanics

- Cannon movement  
- Cannon ball shooting  
- Static aliens  
- Sprite loading  
- Collision detection  

## 🎯 Core Gameplay Features to Add

### ✅ Implemented

- **Alien Movement Pattern**  
  - Horizontal alien movement with bouncing at screen edge  
- **Alien Firing Mechanism**  
  - Aliens shoot at intervals or randomly  
  - Cannon loses life when hit  
- **Lives and Game Over**  
  - Track cannon's lives  
  - Show game over screen  
- **Win Condition**  
  - Game ends when all aliens are defeated  
  - Show "You Win" message  
- **Score System**  
  - Scoring when hitting aliens  
  - Different score values for different alien types  

### 🚧 In Progress / Planned

- **Vertical alien descent**  
- **Levels or Waves**  
  - More alien waves with unique speed, patterns, or difficulty  



## 🔊 Feedback & Polish Features (Upcoming)

- Sound Effects: Shooting, hits, cannon destruction  
- Animations: Explosions and death effects  
- UI Elements:  
  - Score display  
  - Lives indicator  
  - Start / Pause / Game Over screens  
- Title Screen & Restart Option  



## 🧠 Improvement Ideas / Extra Challenges

- **Power-ups**  
  - Random drops like shields, double-shots  
- **Alien Behavior Variety**  
  - UFOs, dodging aliens, etc.  
- **Difficulty Scaling**  
  - Faster waves, tighter formations  
- **Mobile Support**  
  - Touch-based controls  
- **Pause Functionality**  
- **Save High Score**  



## 🧪 How to Run

Make sure you have the Rust toolchain installed. Then:

```bash
git clone https://github.com/your-username/space-invaders-bevy.git
cd space-invaders-bevy
cargo run
```
## Contributions
Pull requests and feature contributions are always welcome 👽🛸