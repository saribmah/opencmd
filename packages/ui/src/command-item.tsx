/**
 * Command item component.
 */

import { Command } from "cmdk";
import type { ReactNode } from "react";
import clsx from "clsx";

export interface CommandItemProps {
  /** Unique identifier */
  id: string;
  /** Display name */
  name: string;
  /** Optional description */
  description?: string;
  /** Optional icon */
  icon?: ReactNode;
  /** Callback when selected */
  onSelect?: () => void;
  /** CSS class name */
  className?: string;
  /** Keywords for filtering */
  keywords?: string[];
}

export function CommandItem({
  id,
  name,
  description,
  icon,
  onSelect,
  className,
  keywords = [],
}: CommandItemProps) {
  return (
    <Command.Item
      value={id}
      keywords={keywords}
      onSelect={onSelect}
      className={clsx("command-item", className)}
    >
      {icon && <span className="command-item-icon">{icon}</span>}
      <div className="command-item-content">
        <span className="command-item-name">{name}</span>
        {description && (
          <span className="command-item-description">{description}</span>
        )}
      </div>
    </Command.Item>
  );
}
