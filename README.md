# skyward

Skyward is a 3rd person 3D survival builder game written in Rust and using Bevy Engine.

The goal of the game it to beat the end boss through constant evolution of the players equipment which is upgraded via building various types of building.

The initial types of materials that a player can gather are:
- Wood
- Stone
- Metal
- Flux

Materials are spawn randomly over the map as nodes which can be harvested by the player using the appropriate tool.
The tools are:
- Axe for Wood
- Pick for Stone and Metal
- Scoop for Flux

The initial buildings that a player can build are:
- Apex - Level 1 (cost: 1000 wood, 1000 stone)
- Work Bench (cost: 250 wood)
- Armoury - Level 1 (cost: 250 wood, 250 stone, 500 metal)
- Temple - Level 1 (cost: 250 wood, 250 stone, 250 metal, 1000 Flux)

The map should be procedurally generated using Perlin Noise and should be 1000x1000 tiles

