// MIT License
// Copyright (c) Destroyer 2026.
import type { InputHTMLAttributes, ReactNode } from 'react';
import styles from './TextInput.module.css';
import { cx } from '../utils/formatters';

interface TextInputProps extends Omit<InputHTMLAttributes<HTMLInputElement>, 'onChange' | 'prefix'> {
  label?: string;
  value: string;
  onChange: (v: string) => void;
  error?: string;
  hint?: string;
  prefix?: ReactNode;
  suffix?: ReactNode;
}

export function TextInput({
  label,
  value,
  onChange,
  error,
  hint,
  prefix,
  suffix,
  disabled,
  className,
  ...rest
}: TextInputProps) {
  return (
    <div className={cx(styles.root, className)}>
      {label && <label className={styles.label}>{label}</label>}
      <div className={cx(styles.wrapper, error && styles.error, disabled && styles.disabled)}>
        {prefix && <span className={styles.prefix}>{prefix}</span>}
        <input
          {...rest}
          value={value}
          disabled={disabled}
          className={styles.input}
          onChange={(e) => onChange(e.target.value)}
        />
        {suffix && <span className={styles.suffix}>{suffix}</span>}
      </div>
      {error && <span className={styles.errorMsg}>{error}</span>}
      {!error && hint && <span className={styles.hint}>{hint}</span>}
    </div>
  );
}
