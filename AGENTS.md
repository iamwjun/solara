# Repository Guidelines

## Project Structure & Module Organization

Solara is a Rust 2024 experimental browser and GPU-rendering project. `src/main.rs` is the binary entry point and dispatches the `gpu-ui` demo. Most implementation lives under `src/gpu_ui/`: `app.rs` manages the window and event loop, `renderer.rs` owns WGPU rendering, and `geometry.rs`, `shapes.rs`, and `text.rs` provide drawing primitives. HTML parsing, layout, and painting are grouped in `src/gpu_ui/html/`; CSS resolution is in `src/gpu_ui/css/`. Keep WGSL shaders in `src/gpu_ui/shaders/` and bundled fonts in `src/gpu_ui/fonts/`.

Reference material and demo inputs live in `docs/`, notably `demoui.html`, `demoui.css`, and `elements.md`. Cargo build output belongs in `target/` and must not be committed.

## Build, Test, and Development Commands

- `cargo build --locked`: compile using the committed dependency lockfile.
- `cargo run -- gpu-ui`: launch the current WGPU UI demo for visual testing.
- `cargo check --locked`: run a fast type and borrow check without producing a binary.
- `cargo test --locked`: run all unit and integration tests; this is also the publish workflow's test command.
- `cargo fmt --all -- --check`: verify standard Rust formatting.
- `cargo clippy --all-targets --all-features -- -D warnings`: catch common Rust mistakes and reject warnings.

Rust 1.85 or newer is required.

## Coding Style & Naming Conventions

Use `rustfmt` defaults (four-space indentation) and keep modules focused on one rendering responsibility. Follow Rust naming conventions: `snake_case` for modules, functions, and variables; `CamelCase` for structs, enums, and traits; `SCREAMING_SNAKE_CASE` for constants. Prefer explicit error handling over `unwrap()` in runtime paths. Add comments only where rendering, layout, or GPU lifetime constraints are not evident from the code.

## Testing Guidelines

The repository currently has no committed tests, so new behavior should include focused unit tests in a colocated `#[cfg(test)] mod tests`. Use descriptive names such as `resolves_inherited_text_color`. Add integration tests under `tests/` when behavior crosses module boundaries. For renderer or layout changes, run `cargo run -- gpu-ui` and inspect the demo; document the manual scenario in the pull request.

## Commit & Pull Request Guidelines

Recent history follows Conventional Commit-style prefixes such as `feat:`, `fix:`, `chore:`, and `docs:`. Keep the subject imperative and scoped to one logical change, for example `fix: prevent text overlap`.

Pull requests should explain the behavior change, list verification commands, and link relevant issues. Include before/after screenshots for visual changes. Keep `Cargo.lock` synchronized with dependency changes, and call out any `Cargo.toml` version update because changes on `main` trigger the crates.io publish workflow.
