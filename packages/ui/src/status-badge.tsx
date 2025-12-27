/**
 * Status badge component.
 */

import clsx from "clsx";
import type { ExtensionStatus } from "@opencmd/protocol";

export interface StatusBadgeProps {
  /** Status to display */
  status: ExtensionStatus;
  /** CSS class name */
  className?: string;
}

const statusLabels: Record<ExtensionStatus, string> = {
  ready: "Ready",
  running: "Running",
  error: "Error",
  disabled: "Disabled",
};

const statusColors: Record<ExtensionStatus, string> = {
  ready: "status-ready",
  running: "status-running",
  error: "status-error",
  disabled: "status-disabled",
};

export function StatusBadge({ status, className }: StatusBadgeProps) {
  return (
    <span className={clsx("status-badge", statusColors[status], className)}>
      {statusLabels[status]}
    </span>
  );
}
