/**
 * Command palette related types.
 * 
 * Note: These match Rust structs exactly (snake_case) since we don't
 * configure serde to rename to camelCase.
 */

/** A searchable command shown in the command palette. */
export interface SearchResult {
  /** Unique identifier (extension_id:command_id) */
  id: string;
  /** Extension that provides this command */
  extension_id: string;
  /** Command ID within the extension */
  command_id: string;
  /** Display name */
  name: string;
  /** Optional description */
  description?: string;
  /** Optional icon identifier */
  icon?: string;
  /** Search relevance score (higher = more relevant) */
  score?: number;
}

/** Request to search for commands. */
export interface SearchRequest {
  /** The search query */
  query: string;
  /** Maximum number of results to return */
  limit?: number;
}

/** Request to execute a command. */
export interface ExecuteRequest {
  /** Extension ID */
  extension_id: string;
  /** Command ID within the extension */
  command_id: string;
  /** Optional input/prompt */
  input?: string;
}

/** Result of command execution - success variant. */
export interface ExecuteResultSuccess {
  type: "success";
  /** Optional output data */
  output?: unknown;
}

/** Result of command execution - process variant. */
export interface ExecuteResultProcess {
  type: "process";
  /** Process/session ID for tracking */
  session_id: string;
}

/** Result of command execution - error variant. */
export interface ExecuteResultError {
  type: "error";
  /** Error message */
  message: string;
}

/** Result of command execution. */
export type ExecuteResult =
  | ExecuteResultSuccess
  | ExecuteResultProcess
  | ExecuteResultError;
