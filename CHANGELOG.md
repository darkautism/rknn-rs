# Changelog

## [v0.2.0]

### Breaking Changes

- **`Rknn` is no longer `Clone`**: The `Clone` implementation has been removed from the `Rknn` struct to prevent double-free errors. The context is now a unique resource. If you need to share the `Rknn` instance, consider wrapping it in `Arc` or `Rc`.
- **`Rknn::destroy` removed**: The `destroy` method has been removed. Resource cleanup is now handled automatically via the `Drop` trait when the `Rknn` instance goes out of scope.
- **`outputs_get` return type changed**: This method now returns a safe `RknnOutput<'a, T>` instead of `ManuallyDrop<RknnOutput<T>>`.
- **`outputs_get_raw` removed**: Because RknnOutput type is safe type and not need copy so remove `outputs_get_raw`.
- The new `RknnOutput` ties its lifetime to the `Rknn` instance to prevent Use-After-Free errors.
- It automatically calls `rknn_outputs_release` on drop, so manual release is no longer required or possible.
- It provides zero-copy access to the data via a slice (`&[T]`) instead of a `Vec`, preventing unsafe memory management.
- **Project Structure**: The project has been split into a Cargo workspace with two crates:
  - `rknn-sys-rs`: Low-level FFI bindings.
  - `rknn-rs`: Safe Rust wrappers (the main entry point).

### Added

- **`rknnmrt` feature**: Added a new feature flag `rknnmrt`. When enabled, the library links against `librknnmrt` instead of `librknnrt`.
- **Safe `rknn_init`**: `rknn_init` now returns a `Result` instead of panicking if the model path contains invalid characters (e.g., null bytes).

### Changed

- **Error Handling**: The `Error` struct and `rkerr!` macro have been moved to a dedicated `rknn::error` module (re-exported in `prelude`).
