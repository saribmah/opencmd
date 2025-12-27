/**
 * Extension context providing APIs for extensions to interact with the host.
 */

export interface StorageAPI {
  /** Get a value from extension storage */
  get(key: string): Promise<string | null>;
  /** Set a value in extension storage */
  set(key: string, value: string): Promise<void>;
  /** Delete a value from extension storage */
  delete(key: string): Promise<void>;
}

export interface ResultItem {
  /** Display title */
  title: string;
  /** Optional subtitle */
  subtitle?: string;
  /** Optional icon */
  icon?: string;
  /** Data to pass when selected */
  data?: unknown;
}

export interface ExtensionContext {
  /** Extension storage API */
  storage: StorageAPI;

  /**
   * Fetch a URL (only allowed hosts from permissions).
   * This is a sandboxed fetch that respects network permissions.
   */
  fetch(url: string, init?: RequestInit): Promise<Response>;

  /**
   * Show results in the command palette.
   */
  showResults(items: ResultItem[]): Promise<void>;

  /**
   * Show a toast notification.
   */
  showToast(message: string, type?: "info" | "success" | "error"): Promise<void>;

  /**
   * Copy text to clipboard.
   */
  copyToClipboard(text: string): Promise<void>;

  /**
   * Get the current configuration for this extension.
   */
  getConfig<T = Record<string, unknown>>(): Promise<T>;
}

/**
 * Create a mock context for testing.
 */
export function createMockContext(): ExtensionContext {
  const storage = new Map<string, string>();

  return {
    storage: {
      async get(key: string) {
        return storage.get(key) ?? null;
      },
      async set(key: string, value: string) {
        storage.set(key, value);
      },
      async delete(key: string) {
        storage.delete(key);
      },
    },
    async fetch(url: string, init?: RequestInit) {
      return globalThis.fetch(url, init);
    },
    async showResults(_items: ResultItem[]) {
      // No-op in mock
    },
    async showToast(message: string, _type?: "info" | "success" | "error") {
      console.log(`[Toast] ${message}`);
    },
    async copyToClipboard(text: string) {
      console.log(`[Clipboard] ${text}`);
    },
    async getConfig<T>() {
      return {} as T;
    },
  };
}
