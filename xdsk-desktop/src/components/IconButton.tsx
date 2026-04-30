// MIT License
// Copyright (c) Destroyer 2026.
import type { ButtonHTMLAttributes, ReactNode } from 'react';
import styles from './IconButton.module.css';
import { cx } from '../utils/formatters';

interface IconButtonProps extends ButtonHTMLAttributes<HTMLButtonElement> {
  icon: ReactNode;
  size?: 'sm' | 'md';
  variant?: 'ghost' | 'default' | 'danger';
  active?: boolean;
}

export function IconButton({
  icon,
  size = 'sm',
  variant = 'ghost',
  active = false,
  className,
  title,
  ...rest
}: IconButtonProps) {
  return (
    <button
      {...rest}
      title={title}
      aria-label={title}
      className={cx(
        styles.btn,
        styles[size],
        styles[variant],
        active && styles.active,
        className
      )}
    >
      {icon}
    </button>
  );
}
