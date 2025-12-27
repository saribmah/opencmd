/**
 * Extension definition and registration.
 */

import type { ExtensionContext } from "./context";

export interface CommandHandler {
  /** Unique command ID (must match manifest) */
  id: string;
  /** Handler function */
  handler: (ctx: ExtensionContext, input?: string) => Promise<void>;
}

export interface ExtensionDefinition {
  /** Commands provided by this extension */
  commands: CommandHandler[];
  /** Called when extension is loaded */
  onLoad?: (ctx: ExtensionContext) => Promise<void>;
  /** Called when extension is unloaded */
  onUnload?: (ctx: ExtensionContext) => Promise<void>;
}

// Global registry for the extension (used by the host to call handlers)
let registeredExtension: ExtensionDefinition | null = null;

/**
 * Define an extension.
 *
 * @example
 * ```ts
 * import { defineExtension } from '@opencmd/extension-api';
 *
 * defineExtension({
 *   commands: [
 *     {
 *       id: 'list-issues',
 *       async handler(ctx) {
 *         const apiKey = await ctx.storage.get('api_key');
 *         // ... fetch and display issues
 *       }
 *     }
 *   ]
 * });
 * ```
 */
export function defineExtension(definition: ExtensionDefinition): void {
  registeredExtension = definition;
}

/**
 * Get the registered extension (used by the host).
 */
export function getRegisteredExtension(): ExtensionDefinition | null {
  return registeredExtension;
}
