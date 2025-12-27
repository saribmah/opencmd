/**
 * Extension-related types.
 */

/** Type of extension determining its runtime behavior. */
export type ExtensionType = "runner" | "integration" | "command";

/** Runtime environment for integration extensions. */
export type ExtensionRuntime = "wasm" | "deno";

/** Configuration for runner-type extensions. */
export interface RunnerConfig {
  /** Command to execute */
  command: string;
  /** Arguments to pass (supports {{prompt}} template) */
  args: string[];
  /** Whether the runner is interactive */
  interactive?: boolean;
  /** Whether to use PTY */
  pty?: boolean;
}

/** A command exposed by an extension. */
export interface ExtensionCommand {
  /** Unique identifier for this command within the extension */
  id: string;
  /** Human-readable name */
  name: string;
  /** Optional description */
  description?: string;
  /** Keywords for search/filtering */
  keywords?: string[];
}

/** Network permission for integration extensions. */
export interface NetworkPermission {
  /** Allowed hosts */
  network: string[];
}

/** Permission types for extensions. */
export type Permission = string | NetworkPermission;

/** Extension manifest loaded from manifest.json. */
export interface ExtensionManifest {
  /** Unique identifier for the extension */
  id: string;
  /** Human-readable name */
  name: string;
  /** Version string (semver) */
  version: string;
  /** Type of extension */
  type: ExtensionType;
  /** Optional description */
  description?: string;
  /** Runner configuration (for runner type) */
  runner?: RunnerConfig;
  /** Runtime environment (for integration type) */
  runtime?: ExtensionRuntime;
  /** Main entry point (for integration type) */
  main?: string;
  /** Script to run (for command type) */
  script?: string;
  /** Commands exposed by this extension */
  commands?: ExtensionCommand[];
  /** Required permissions */
  permissions?: Permission[];
}

/** Represents a loaded extension at runtime. */
export interface Extension {
  /** Unique runtime ID */
  id: string;
  /** The extension manifest */
  manifest: ExtensionManifest;
  /** Path to the extension directory */
  path: string;
  /** Whether the extension is currently enabled */
  enabled: boolean;
}

/** Status of an extension. */
export type ExtensionStatus = "ready" | "running" | "error" | "disabled";
