RustGameEngine
RustGameEngine is a work-in-progress game engine written in Rust. Right now, it’s mostly focused on setting up the UI framework and basic architecture. It’s not really ready for full game development yet, but we’re building the foundation for it.

The engine is inspired by popular engines like Unity and Godot, but we’re using Rust for its speed and safety. There’s a long way to go, but the goal is to eventually support 2D and 3D game development.

What’s Done So Far
UI System: The main thing so far is a basic UI with panels, scene hierarchy, and more.
Modular Design: The engine is being built to be modular, so adding new features should be easy.
Cross-Platform: It works on both Windows and Linux for now.
What’s Coming




Disclaimer
This game engine is still in development. It is not yet stable, and things may change frequently. The API is subject to change as we improve the engine.

Features (Planned)
2D/3D Rendering: Eventually, we’ll add rendering with wgpu (cross-platform support).
Modular Architecture: Each component (graphics, audio, input, etc.) is separate for ease of extensibility.
Entity Component System (ECS): A flexible architecture for game object management.
Audio & Physics: These are on the roadmap for future updates.
Scene Graph: A hierarchical structure for game objects.
Dockable Panels: With a scene hierarchy, viewport, and context menu.
Cross-Platform Support: Targeting both Windows and Linux.

Note: The feature set is subject to change as development progresses and priorities evolve.

Contributing
Contributions are welcome! If you want to contribute to RustGameEngine, feel free to fork the repository, make changes, and submit a pull request. Please follow these guidelines:

Ensure your code passes the tests (cargo test).
Follow Rust's coding standards and best practices.
Provide clear documentation for any new features or changes.

License
* MIT License ([LICENSE-MIT](LICENSE-MIT) or [http://opensource.org/licenses/MIT](http://opensource.org/licenses/MIT))
* Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or [http://www.apache.org/licenses/LICENSE-2.0](http://www.apache.org/licenses/LICENSE-2.0))