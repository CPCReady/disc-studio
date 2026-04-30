// MIT License
// Copyright (c) Destroyer 2026.
import type { ReactNode } from 'react';
import styles from './Card.module.css';
import { cx } from '../utils/formatters';

interface CardProps {
  children: ReactNode;
  title?: string;
  actions?: ReactNode;
  variant?: 'default' | 'compact';
  className?: string;
}

export function Card({ children, title, actions, variant = 'default', className }: CardProps) {
  return (
    <div className={cx(styles.card, variant === 'compact' && styles.compact, className)}>
      {(title || actions) && (
        <div className={styles.header}>
          {title && <span className={styles.title}>{title}</span>}
          {actions}
        </div>
      )}
      <div className={styles.body}>{children}</div>
    </div>
  );
}
