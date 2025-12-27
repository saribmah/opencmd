import { useEffect, useState, useCallback, useRef } from "react";
import { invoke } from "@tauri-apps/api/core";
import { getCurrentWebviewWindow } from "@tauri-apps/api/webviewWindow";
import {
  CommandPalette,
  CommandItem,
  CommandEmpty,
  CommandGroup,
} from "@opencmd/ui";
import type { SearchResult } from "@opencmd/protocol";
import "./App.css";

function App() {
  const [query, setQuery] = useState("");
  const [results, setResults] = useState<SearchResult[]>([]);
  const [initialized, setInitialized] = useState(false);
  const [loading, setLoading] = useState(false);
  const containerRef = useRef<HTMLDivElement>(null);

  // Initialize the app on mount
  useEffect(() => {
    async function init() {
      try {
        await invoke("initialize");
        setInitialized(true);
        // Show window after initialization
        await invoke("show_window");
      } catch (error) {
        console.error("Failed to initialize:", error);
      }
    }
    init();
  }, []);

  // Search when query changes
  useEffect(() => {
    if (!initialized) return;

    async function search() {
      setLoading(true);
      try {
        const searchResults = await invoke<SearchResult[]>("search", {
          query,
          limit: 10,
        });
        setResults(searchResults);
      } catch (error) {
        console.error("Search failed:", error);
      } finally {
        setLoading(false);
      }
    }

    const debounce = setTimeout(search, 50);
    return () => clearTimeout(debounce);
  }, [query, initialized]);

  // Handle command selection
  const handleSelect = useCallback(
    async (result: SearchResult) => {
      try {
        const response = await invoke("execute", {
          extensionId: result.extension_id,
          commandId: result.command_id,
          input: query,
        });
        console.log("Execute result:", response);
        // Hide window after execution
        await invoke("hide_window");
        // Reset state for next invocation
        setQuery("");
      } catch (error) {
        console.error("Execute failed:", error);
      }
    },
    [query]
  );

  // Hide window on escape
  const handleEscape = useCallback(async () => {
    await invoke("hide_window");
    setQuery("");
  }, []);

  // Hide window on blur (click outside)
  useEffect(() => {
    const appWindow = getCurrentWebviewWindow();
    
    const unlisten = appWindow.onFocusChanged(({ payload: focused }) => {
      if (!focused) {
        invoke("hide_window");
        setQuery("");
      }
    });

    return () => {
      unlisten.then((fn) => fn());
    };
  }, []);

  // Group results by extension
  const groupedResults = results.reduce(
    (acc, result) => {
      const key = result.extension_id;
      if (!acc[key]) {
        acc[key] = [];
      }
      acc[key].push(result);
      return acc;
    },
    {} as Record<string, SearchResult[]>
  );

  return (
    <div className="app" ref={containerRef}>
      <CommandPalette
        value={query}
        onValueChange={setQuery}
        onEscape={handleEscape}
        placeholder={initialized ? "Type a command or search..." : "Initializing..."}
        loading={loading}
        className="command-root"
      >
        {!initialized ? (
          <CommandEmpty>Initializing...</CommandEmpty>
        ) : results.length === 0 && query ? (
          <CommandEmpty>No results found for "{query}"</CommandEmpty>
        ) : results.length === 0 ? (
          <CommandEmpty className="command-hint">
            Start typing to search commands...
          </CommandEmpty>
        ) : (
          Object.entries(groupedResults).map(([extensionId, items]) => (
            <CommandGroup key={extensionId} heading={formatExtensionName(extensionId)}>
              {items.map((result) => (
                <CommandItem
                  key={result.id}
                  id={result.id}
                  name={result.name}
                  description={result.description}
                  onSelect={() => handleSelect(result)}
                  className="command-item"
                  icon={<ExtensionIcon extensionId={extensionId} />}
                />
              ))}
            </CommandGroup>
          ))
        )}
      </CommandPalette>
    </div>
  );
}

// Format extension ID to display name
function formatExtensionName(extensionId: string): string {
  return extensionId
    .split("-")
    .map((word) => word.charAt(0).toUpperCase() + word.slice(1))
    .join(" ");
}

// Simple icon component based on extension type
function ExtensionIcon({ extensionId }: { extensionId: string }) {
  const icons: Record<string, string> = {
    "claude-code": "🤖",
    "opencode": "⚡",
    "shell": "💻",
    "github-pr": "🐙",
  };
  return <span>{icons[extensionId] || "📦"}</span>;
}

export default App;
