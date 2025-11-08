# Procedural Forest Explorer

A 3D procedural forest exploration game built with Bevy (Rust game engine). Features dynamically generated terrain, trees, day/night cycles, and interactive elements.

## Features

### ✅ Implemented Features

1. **Procedural Terrain Generation**
   - Chunk-based terrain system using Perlin noise
   - Dynamic loading/unloading of terrain chunks based on player position
   - Height-based material variation (grass, dark grass, rock)

2. **Procedural Tree Generation**
   - Trees procedurally placed using noise functions
   - Dynamic spawning around player
   - Wind animation effects on tree canopies

3. **Player Navigation**
   - First-person camera controls
   - WASD movement
   - Mouse look (locked cursor)
   - ESC to unlock cursor

4. **Day/Night Cycle**
   - Dynamic time progression
   - Sun position changes throughout the day
   - Lighting and sky color transitions (dawn, day, dusk, night)
   - Smooth color interpolation

5. **Interactive Elements**
   - Glowing mushrooms scattered throughout the forest
   - Collect mushrooms with E key
   - Particle effects on collection

6. **Atmospheric Effects**
   - Dynamic fog that changes with time of day
   - Fog density varies (thicker at night, lighter during day)

7. **UI System**
   - On-screen controls display
   - Time of day display
   - Mushroom collection counter

## Controls

- **W/A/S/D** - Move forward/left/backward/right
- **Mouse** - Look around (camera rotation)
- **E** - Collect nearby mushrooms
- **ESC** - Toggle cursor lock (unlock to interact with window)

## Project Structure

The project is modularly organized:

```
src/
├── main.rs          # Main entry point, plugin registration
├── terrain.rs       # Procedural terrain generation with chunks
├── trees.rs         # Tree and foliage generation
├── player.rs        # Player camera and movement controls
├── day_night.rs     # Day/night cycle and lighting
├── interactivity.rs # Mushrooms and collection system
├── fog.rs           # Fog and atmospheric effects
└── ui.rs            # User interface elements
```

## Building and Running

### Prerequisites
- Rust (latest stable version)
- Cargo

### Build
```bash
cargo build --release
```

### Run
```bash
cargo run --release
```

Or run the release binary directly:
```bash
./target/release/virtual_env
```

## Technical Details

### Dependencies
- **bevy** (0.14) - Game engine
- **noise** (0.9) - Perlin noise for procedural generation
- **rand** (0.8) - Random number generation

### Performance Considerations
- Chunk-based terrain loading for efficient memory usage
- Dynamic entity spawning around player
- Optimized mesh generation
- LOD considerations can be added for further optimization

### Architecture
- **ECS (Entity-Component-System)** - Bevy's architecture
- **Plugin-based** - Each feature is a separate plugin
- **Resource-based** - Shared state via Bevy resources
- **Event-driven** - Mushroom collection uses events

## Future Enhancements

Potential additions:
- LOD (Level of Detail) system for trees
- More complex terrain features (rivers, caves)
- Wildlife entities
- Sound effects
- More interactive elements
- Custom shaders for advanced visual effects
- Save/load system
- More detailed particle systems

## Notes

- The terrain uses simplified height sampling (in production, you'd query actual terrain height)
- Tree placement uses noise-based density distribution
- Mushroom spawning uses rarity-based placement
- All procedural generation uses fixed seeds for reproducibility

## License

This project is for educational/demonstration purposes.

