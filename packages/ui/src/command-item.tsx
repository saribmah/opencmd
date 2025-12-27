/**
 * Command item component.
 */

import { Command } from "cmdk";
import type { ReactNode } from "react";

export interface CommandItemProps {
  /** Unique identifier */
  id: string;
  /** Display name */
  name: string;
  /** Optional description/subtitle */
  description?: string;
  /** Optional icon (React node) */
  icon?: ReactNode;
  /** Optional right-side accessory (e.g., keyboard shortcut) */
  accessory?: ReactNode;
  /** Callback when selected */
  onSelect?: () => void;
  /** CSS class name */
  className?: string;
  /** Keywords for filtering (handled by cmdk) */
  keywords?: string[];
  /** Whether this item is disabled */
  disabled?: boolean;
}

export function CommandItem({
  id,
  name,
  description,
  icon,
  accessory,
  onSelect,
  className,
  keywords = [],
  disabled = false,
}: CommandItemProps) {
  return (
    <Command.Item
      value={id}
      keywords={keywords}
      onSelect={onSelect}
      className={className}
      disabled={disabled}
    >
      <div className="command-item-left">
        {icon && <span className="command-item-icon">{icon}</span>}
        <div className="command-item-content">
          <span className="command-item-name">{name}</span>
          {description && (
            <span className="command-item-description">{description}</span>
          )}
        </div>
      </div>
      {accessory && (
        <div className="command-item-accessory">{accessory}</div>
      )}
    </Command.Item>
  );
}
