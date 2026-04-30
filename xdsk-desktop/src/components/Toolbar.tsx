// MIT License
// Copyright (c) Destroyer 2026.
import type { ReactNode } from 'react';
import styles from './Toolbar.module.css';
import { cx } from '../utils/formatters';

interface ToolbarProps {
  children: ReactNode;
  className?: string;
}

export function Toolbar({ children, className }: ToolbarProps) {
  return <div className={cx(styles.toolbar, className)}>{children}</div>;
}

export function ToolbarSeparator() {
  return <div className={styles.separator} />;
}

export function ToolbarSpacer() {
  return <div className={styles.spacer} />;
}
