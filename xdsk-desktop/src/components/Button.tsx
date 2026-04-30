// MIT License
// Copyright (c) Destroyer 2026.
import type { ButtonHTMLAttributes, ReactNode } from 'react';
import { Loader2 } from 'lucide-react';
import styles from './Button.module.css';
import { cx } from '../utils/formatters';

export type ButtonVariant = 'primary' | 'secondary' | 'ghost' | 'danger';
export type ButtonSize = 'sm' | 'md';

interface ButtonProps extends ButtonHTMLAttributes<HTMLButtonElement> {
  variant?: ButtonVariant;
  size?: ButtonSize;
  loading?: boolean;
  icon?: ReactNode;
}

export function Button({
  children,
  variant = 'secondary',
  size = 'sm',
  loading = false,
  icon,
  disabled,
  className,
  ...rest
}: ButtonProps) {
  return (
    <button
      {...rest}
      disabled={disabled || loading}
      className={cx(
        styles.btn,
        styles[variant],
        styles[size],
        loading && styles.loading,
        className
      )}
    >
      {loading ? (
        <Loader2 size={12} className={styles.icon} style={{ animation: 'spin 1s linear infinite' }} />
      ) : icon ? (
        <span className={styles.icon}>{icon}</span>
      ) : null}
      {children}
    </button>
  );
}
