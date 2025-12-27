//! Protocol types shared between Rust backend and TypeScript frontend.
//!
//! These types are serialized to JSON and sent over Tauri's IPC bridge.
//! The TypeScript equivalents live in `packages/protocol/`.

mod commands;
mod events;
mod extensions;

pub use commands::*;
pub use events::*;
pub use extensions::*;
