// MIT License
// Copyright (c) Destroyer 2026.
import type { ReactNode } from 'react';
import styles from './SectionHeader.module.css';
import { cx } from '../utils/formatters';

interface SectionHeaderProps {
  title: string;
  description?: string;
  actions?: ReactNode;
  className?: string;
}

export function SectionHeader({ title, description, actions, className }: SectionHeaderProps) {
  return (
    <div className={cx(styles.root, className)}>
      <div className={styles.text}>
        <h2 className={styles.title}>{title}</h2>
        {description && <p className={styles.description}>{description}</p>}
      </div>
      {actions && <div className={styles.actions}>{actions}</div>}
    </div>
  );
}
