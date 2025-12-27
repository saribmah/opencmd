/**
 * Command palette component - the main launcher interface.
 * 
 * This component is designed to be the root of a Raycast-like window.
 * It's not a dialog - it's a standalone command interface.
 */

import { Command } from "cmdk";
import { forwardRef, type ReactNode } from "react";

export interface CommandPaletteProps {
  /** Children to render inside the palette (CommandList, groups, etc.) */
  children?: ReactNode;
  /** Placeholder text for the input */
  placeholder?: string;
  /** Current search value */
  value?: string;
  /** Callback when search value changes */
  onValueChange?: (value: string) => void;
  /** Callback when escape is pressed */
  onEscape?: () => void;
  /** CSS class name for the root container */
  className?: string;
  /** Whether to show the loading state */
  loading?: boolean;
}

export const CommandPalette = forwardRef<HTMLDivElement, CommandPaletteProps>(
  function CommandPalette(
    {
      children,
      placeholder = "Type a command or search...",
      value,
      onValueChange,
      onEscape,
      className,
      loading = false,
    },
    ref
  ) {
    return (
      <Command
        ref={ref}
        className={className}
        onKeyDown={(e) => {
          if (e.key === "Escape") {
            onEscape?.();
          }
        }}
        shouldFilter={false} // We handle filtering on the backend
      >
        <div className="command-header">
          <Command.Input
            placeholder={placeholder}
            value={value}
            onValueChange={onValueChange}
            className="command-input"
            autoFocus
          />
          {loading && <div className="command-loading" />}
        </div>
        <Command.List className="command-list">
          {children}
        </Command.List>
      </Command>
    );
  }
);

export interface CommandEmptyProps {
  children?: ReactNode;
  className?: string;
}

export function CommandEmpty({ children, className }: CommandEmptyProps) {
  return (
    <Command.Empty className={className}>
      {children || "No results found."}
    </Command.Empty>
  );
}

export interface CommandGroupProps {
  heading?: string;
  children?: ReactNode;
  className?: string;
}

export function CommandGroup({ heading, children, className }: CommandGroupProps) {
  return (
    <Command.Group heading={heading} className={className}>
      {children}
    </Command.Group>
  );
}
