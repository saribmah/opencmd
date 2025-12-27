import { useEffect, useState, useCallback } from "react";
import { Command } from "cmdk";
import { invoke } from "@tauri-apps/api/core";
import type { SearchResult } from "@opencmd/protocol";
import "./App.css";

function App() {
  const [open, setOpen] = useState(true);
  const [query, setQuery] = useState("");
  const [results, setResults] = useState<SearchResult[]>([]);
  const [initialized, setInitialized] = useState(false);

  // Initialize the app on mount
  useEffect(() => {
    async function init() {
      try {
        await invoke("initialize");
        setInitialized(true);
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
      try {
        const searchResults = await invoke<SearchResult[]>("search", {
          query,
          limit: 20,
        });
        setResults(searchResults);
      } catch (error) {
        console.error("Search failed:", error);
      }
    }

    const debounce = setTimeout(search, 100);
    return () => clearTimeout(debounce);
  }, [query, initialized]);

  // Handle command selection
  const handleSelect = useCallback(async (result: SearchResult) => {
    try {
      const response = await invoke("execute", {
        extensionId: result.extensionId,
        commandId: result.commandId,
        input: query,
      });
      console.log("Execute result:", response);
      // Handle the result based on type
    } catch (error) {
      console.error("Execute failed:", error);
    }
  }, [query]);

  // Toggle with keyboard shortcut
  useEffect(() => {
    const handleKeyDown = (e: KeyboardEvent) => {
      if (e.key === "k" && (e.metaKey || e.ctrlKey)) {
        e.preventDefault();
        setOpen((prev) => !prev);
      }
      if (e.key === "Escape") {
        setOpen(false);
      }
    };

    document.addEventListener("keydown", handleKeyDown);
    return () => document.removeEventListener("keydown", handleKeyDown);
  }, []);

  return (
    <div className="app">
      <Command.Dialog
        open={open}
        onOpenChange={setOpen}
        className="command-dialog"
        label="Command Palette"
      >
        <Command.Input
          placeholder="Type a command or search..."
          value={query}
          onValueChange={setQuery}
          className="command-input"
        />

        <Command.List className="command-list">
          <Command.Empty className="command-empty">
            {initialized ? "No results found." : "Initializing..."}
          </Command.Empty>

          {results.map((result) => (
            <Command.Item
              key={result.id}
              value={result.id}
              onSelect={() => handleSelect(result)}
              className="command-item"
            >
              <div className="command-item-content">
                <span className="command-item-name">{result.name}</span>
                {result.description && (
                  <span className="command-item-description">
                    {result.description}
                  </span>
                )}
              </div>
              <span className="command-item-extension">
                {result.extensionId}
              </span>
            </Command.Item>
          ))}
        </Command.List>
      </Command.Dialog>

      {!open && (
        <div className="hint">
          Press <kbd>Cmd</kbd> + <kbd>K</kbd> to open command palette
        </div>
      )}
    </div>
  );
}

export default App;
