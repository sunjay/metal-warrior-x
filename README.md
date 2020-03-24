# metal-warrior-x

This is a concept game that uses [spritec] to generate its assets. The game has
not been created yet.

## Notes

Ideas:

* full screen game
* enemy swarms
* Kenney 2D/3D assets
* 3D game with 8 directional movement (like [Zealot](https://primecut.games/zealot))
   * Orthographic camera?
* 32x32 character
* Much larger sprite for boss (in the same spritesheet)
* Integrate bigboi, alien_ghost, etc. if possible (not required)
   * Maybe as special items or NPCs

Game concept:

* robots have taken over
* goal: take back the city
    * robots are planning to poison the water supply, need to get to the final boss before everything is totally poisoned (or before they manage to get all the poison?)
* player is a human who uses robot parts (e.g. robot arms, projectiles, etc.) against the robots in order to defeat them
* need to destroy mobs of robots, earn points, and collect parts
* boss level only unlocked once you have a certain number of points
* fight through waves of enemies
* tanks, bigger robots, turrets
* boss robots
* different weapons (short-range, long-range, timed/bombs, etc.)
    * limited weapon slots
    * limited ammo (can be picked up during the level or bought in the shop)
    * weapons break after a certain amount of use (main fist weapon is always available)
* walls that you can shoot and destroy
* health bar
* pickup robot parts
    * robots drop random parts when defeated
    * sell parts for money or use them in upgrades
* defeating robots gives you a fixed number of points
* buy upgrades and health - from where?
    * map of levels/worlds like in Celeste or Super Mario
    * shop is accessed by going back and forth through the level menu (like Super Mario)

## Generating assets

To generate the assets, run the following command from the root directory of
this project:

```bash
cargo run --manifest-path assets/spritesheets/Cargo.toml
```

Currently, you need to have [spritec] in the same directory as the directory
where you have this project. This is because the asset generation uses spritec
as a path dependency.

[spritec]: https://github.com/ProtoArt/spritec
