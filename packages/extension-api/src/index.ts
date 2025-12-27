/**
 * OpenCMD Extension API
 *
 * SDK for building OpenCMD extensions. Integration extensions import this
 * package to define their commands and interact with the host.
 */

export { defineExtension } from "./extension";
export { ExtensionContext } from "./context";

// Re-export protocol types
export type {
  ExtensionManifest,
  ExtensionCommand,
  ExtensionType,
  Permission,
  SearchResult,
  ExecuteResult,
} from "@opencmd/protocol";
