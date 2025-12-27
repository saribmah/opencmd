/**
 * Command list component - container for command items.
 */

import type { ReactNode } from "react";

export interface CommandListProps {
  /** Children (CommandItem components) */
  children?: ReactNode;
  /** CSS class name */
  className?: string;
}

export function CommandList({ children, className }: CommandListProps) {
  // Note: The actual Command.List is rendered by CommandPalette
  // This is a semantic wrapper for organizing items
  return <div className={className}>{children}</div>;
}
