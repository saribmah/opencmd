/**
 * Terminal view component for displaying runner output.
 */

import { useEffect, useRef } from "react";
import clsx from "clsx";

export interface TerminalViewProps {
  /** Terminal output content */
  content: string;
  /** Whether the terminal is active/running */
  active?: boolean;
  /** Callback for sending input */
  onInput?: (data: string) => void;
  /** CSS class name */
  className?: string;
}

export function TerminalView({
  content,
  active = false,
  onInput,
  className,
}: TerminalViewProps) {
  const containerRef = useRef<HTMLDivElement>(null);
  const inputRef = useRef<HTMLInputElement>(null);

  // Auto-scroll to bottom when content changes
  useEffect(() => {
    if (containerRef.current) {
      containerRef.current.scrollTop = containerRef.current.scrollHeight;
    }
  }, [content]);

  // Focus input when active
  useEffect(() => {
    if (active && inputRef.current) {
      inputRef.current.focus();
    }
  }, [active]);

  const handleKeyDown = (e: React.KeyboardEvent<HTMLInputElement>) => {
    if (e.key === "Enter" && onInput) {
      const input = inputRef.current?.value;
      if (input !== undefined) {
        onInput(input + "\n");
        if (inputRef.current) {
          inputRef.current.value = "";
        }
      }
    }
  };

  return (
    <div className={clsx("terminal-view", className)}>
      <div ref={containerRef} className="terminal-output">
        <pre>{content}</pre>
      </div>
      {active && onInput && (
        <input
          ref={inputRef}
          type="text"
          className="terminal-input"
          onKeyDown={handleKeyDown}
          placeholder="Type here..."
        />
      )}
    </div>
  );
}
