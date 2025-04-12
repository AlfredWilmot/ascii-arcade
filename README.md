# AsciiArcade

This is a simple 2D physics engine. \
A TUI (_Terminal User Interface_) frontend application is used to showcase
various arcade games based that are built using this physics engine.

> [!WARNING]
> 🚧 **WORK IN PROGRESS** 🚧
> The following dependencies are used:
> ```text
> rustup 1.28.1 (2025-03-05)
> cargo 1.85.1 (d73d2caf9 2024-12-31)
> rustc 1.85.1 (4eb161250 2025-03-15)
> ```

To run the application, execute the following command in your favorite terminal:

```bash
cargo run
```

![main-menu](./examples/pics/menu.jpeg)

### Games

This section overviews all games that are currently implemented. \
These commands can be run at any time:

```
return to menu (ESC)
exit (q)
```

#### Sandbox

<img src="./examples/gifs/sandbox_game.gif" alt="linear collision" style="width:400px;height:250px;">

```text
move player (wasd)
spawn collision entity (LMB)
spawn static entity (RMB)
