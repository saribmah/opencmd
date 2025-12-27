/**
 * Command input component.
 */

import { Command } from "cmdk";

export interface CommandInputProps {
  /** Placeholder text */
  placeholder?: string;
  /** Current value */
  value?: string;
  /** Callback when value changes */
  onValueChange?: (value: string) => void;
  /** CSS class name */
  className?: string;
}

export function CommandInput({
  placeholder = "Type a command...",
  value,
  onValueChange,
  className,
}: CommandInputProps) {
  return (
    <Command.Input
      placeholder={placeholder}
      value={value}
      onValueChange={onValueChange}
      className={className}
    />
  );
}
