/**
 * Command list component.
 */

import { Command } from "cmdk";
import type { ReactNode } from "react";

export interface CommandListProps {
  /** Children (CommandItem components) */
  children?: ReactNode;
  /** CSS class name */
  className?: string;
}

export function CommandList({ children, className }: CommandListProps) {
  return (
    <Command.List className={className}>
      <Command.Empty>No results found.</Command.Empty>
      {children}
    </Command.List>
  );
}
