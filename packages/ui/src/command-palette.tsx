/**
 * Command palette component - the main launcher interface.
 */

import { Command } from "cmdk";
import type { ReactNode } from "react";

export interface CommandPaletteProps {
  /** Whether the palette is open */
  open?: boolean;
  /** Callback when open state changes */
  onOpenChange?: (open: boolean) => void;
  /** Children to render inside the palette */
  children?: ReactNode;
  /** Placeholder text for the input */
  placeholder?: string;
  /** Current search value */
  value?: string;
  /** Callback when search value changes */
  onValueChange?: (value: string) => void;
  /** CSS class name */
  className?: string;
}

export function CommandPalette({
  open,
  onOpenChange,
  children,
  placeholder = "Type a command or search...",
  value,
  onValueChange,
  className,
}: CommandPaletteProps) {
  return (
    <Command.Dialog
      open={open}
      onOpenChange={onOpenChange}
      className={className}
    >
      <Command.Input
        placeholder={placeholder}
        value={value}
        onValueChange={onValueChange}
      />
      <Command.List>{children}</Command.List>
    </Command.Dialog>
  );
}
