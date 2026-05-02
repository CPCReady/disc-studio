// MIT License
// Copyright (c) Destroyer 2026.
import styles from './StatusIndicator.module.css';
import { cx } from '../utils/formatters';

export type StatusType = 'success' | 'error' | 'warning' | 'idle' | 'loading';

interface StatusIndicatorProps {
  status: StatusType;
  label: string;
  size?: 'sm' | 'md';
  className?: string;
}

export function StatusIndicator({ status, label, size = 'md', className }: StatusIndicatorProps) {
  return (
    <span className={cx(styles.root, styles[status], styles[size], className)}>
      <span className={styles.dot} />
      <span className={styles.label}>{label}</span>
    </span>
  );
}
