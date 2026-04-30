// MIT License
// Copyright (c) Destroyer 2026.
import styles from './Select.module.css';
import { cx } from '../utils/formatters';

export interface SelectOption {
  value: string;
  label: string;
}

interface SelectProps {
  label?: string;
  value: string;
  onChange: (v: string) => void;
  options: SelectOption[];
  disabled?: boolean;
  error?: string;
  placeholder?: string;
  className?: string;
}

export function Select({
  label,
  value,
  onChange,
  options,
  disabled,
  error,
  placeholder,
  className,
}: SelectProps) {
  return (
    <div className={cx(styles.root, className)}>
      {label && <label className={styles.label}>{label}</label>}
      <select
        value={value}
        disabled={disabled}
        className={styles.select}
        onChange={(e) => onChange(e.target.value)}
      >
        {placeholder && <option value="" disabled>{placeholder}</option>}
        {options.map((o) => (
          <option key={o.value} value={o.value}>{o.label}</option>
        ))}
      </select>
      {error && <span className={styles.errorMsg}>{error}</span>}
    </div>
  );
}
