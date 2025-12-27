/**
 * Protocol types matching the Rust crates/protocol/ types.
 * These are serialized over Tauri's IPC bridge.
 */

// Re-export all types
export * from "./extensions";
export * from "./commands";
export * from "./events";
