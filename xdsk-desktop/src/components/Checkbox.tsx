// MIT License
// Copyright (c) Destroyer 2026.
import { Check } from 'lucide-react';
import styles from './Checkbox.module.css';
import { cx } from '../utils/formatters';

interface CheckboxProps {
  label: string;
  checked: boolean;
  onChange: (v: boolean) => void;
  disabled?: boolean;
  description?: string;
  className?: string;
}

export function Checkbox({ label, checked, onChange, disabled, description, className }: CheckboxProps) {
  return (
    <label className={cx(styles.root, disabled && styles.disabled, className)}>
      <input
        type="checkbox"
        checked={checked}
        disabled={disabled}
        className={styles.hidden}
        onChange={(e) => onChange(e.target.checked)}
      />
      <span className={cx(styles.box, checked && styles.checked)}>
        {checked && <Check size={10} className={styles.check} strokeWidth={3} />}
      </span>
      <span className={styles.content}>
        <span className={styles.label}>{label}</span>
        {description && <span className={styles.description}>{description}</span>}
      </span>
    </label>
  );
}
