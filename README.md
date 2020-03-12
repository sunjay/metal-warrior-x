# metal-warrior-x

This is a concept game that uses [spritec] to generate its assets. The game has
not been created yet.

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
