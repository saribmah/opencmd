/**
 * Event types for the pub/sub event bus.
 */

import type { ExtensionStatus } from "./extensions";

/** Extension was loaded event. */
export interface ExtensionLoadedEvent {
  type: "extension_loaded";
  extensionId: string;
}

/** Extension was unloaded event. */
export interface ExtensionUnloadedEvent {
  type: "extension_unloaded";
  extensionId: string;
}

/** Extension status changed event. */
export interface ExtensionStatusChangedEvent {
  type: "extension_status_changed";
  extensionId: string;
  status: ExtensionStatus;
}

/** Process output from a runner event. */
export interface ProcessOutputEvent {
  type: "process_output";
  sessionId: string;
  data: string;
}

/** Process exited event. */
export interface ProcessExitedEvent {
  type: "process_exited";
  sessionId: string;
  exitCode?: number;
}

/** Configuration changed event. */
export interface ConfigChangedEvent {
  type: "config_changed";
  key: string;
}

/** Window visibility changed event. */
export interface WindowVisibilityChangedEvent {
  type: "window_visibility_changed";
  visible: boolean;
}

/** Events emitted by the application. */
export type AppEvent =
  | ExtensionLoadedEvent
  | ExtensionUnloadedEvent
  | ExtensionStatusChangedEvent
  | ProcessOutputEvent
  | ProcessExitedEvent
  | ConfigChangedEvent
  | WindowVisibilityChangedEvent;

/** Event subscription request. */
export interface EventSubscription {
  /** Event types to subscribe to (empty = all) */
  eventTypes: string[];
}
