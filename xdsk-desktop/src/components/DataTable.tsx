// MIT License
// Copyright (c) Destroyer 2026.
import type { ReactNode } from 'react';
import styles from './DataTable.module.css';
import { cx } from '../utils/formatters';

export interface Column<T> {
  key: string;
  header: string;
  width?: string;
  align?: 'left' | 'center' | 'right';
  render?: (value: unknown, row: T) => ReactNode;
}

interface DataTableProps<T extends Record<string, unknown>> {
  columns: Column<T>[];
  data: T[];
  keyField: string;
  loading?: boolean;
  emptyMessage?: string;
  selectable?: boolean;
  selectedKeys?: string[];
  onSelectionChange?: (keys: string[]) => void;
  onRowClick?: (row: T) => void;
  onRowContextMenu?: (row: T, e: React.MouseEvent) => void;
  className?: string;
}

export function DataTable<T extends Record<string, unknown>>({
  columns,
  data,
  keyField,
  loading,
  emptyMessage = 'No data',
  selectable,
  selectedKeys = [],
  onSelectionChange,
  onRowClick,
  onRowContextMenu,
  className,
}: DataTableProps<T>) {
  const toggle = (key: string) => {
    if (!onSelectionChange) return;
    const next = selectedKeys.includes(key)
      ? selectedKeys.filter((k) => k !== key)
      : [...selectedKeys, key];
    onSelectionChange(next);
  };

  const toggleAll = () => {
    if (!onSelectionChange) return;
    const allKeys = data.map((r) => String(r[keyField]));
    onSelectionChange(selectedKeys.length === data.length ? [] : allKeys);
  };

  return (
    <div className={cx(styles.wrapper, className)}>
      <table className={styles.table}>
        <thead className={styles.thead}>
          <tr>
            {selectable && (
              <th className={styles.tdCheck}>
                <input
                  type="checkbox"
                  checked={data.length > 0 && selectedKeys.length === data.length}
                  onChange={toggleAll}
                />
              </th>
            )}
            {columns.map((col) => (
              <th
                key={col.key}
                className={cx(styles.th, col.align === 'right' && styles.right, col.align === 'center' && styles.center)}
                style={col.width ? { width: col.width } : undefined}
              >
                {col.header}
              </th>
            ))}
          </tr>
        </thead>
        <tbody>
          {loading ? (
            <tr><td colSpan={columns.length + (selectable ? 1 : 0)} className={styles.loading}>Loading…</td></tr>
          ) : data.length === 0 ? (
            <tr><td colSpan={columns.length + (selectable ? 1 : 0)} className={styles.empty}>{emptyMessage}</td></tr>
          ) : (
            data.map((row) => {
              const key = String(row[keyField]);
              const isSelected = selectedKeys.includes(key);
              return (
                <tr
                  key={key}
                  className={cx(styles.tr, isSelected && styles.selected, onRowClick && styles.clickable)}
                  onClick={() => onRowClick?.(row)}
                  onContextMenu={(e) => { e.preventDefault(); onRowContextMenu?.(row, e); }}
                >
                  {selectable && (
                    <td className={styles.tdCheck} onClick={(e) => { e.stopPropagation(); toggle(key); }}>
                      <input type="checkbox" checked={isSelected} onChange={() => toggle(key)} />
                    </td>
                  )}
                  {columns.map((col) => (
                    <td
                      key={col.key}
                      className={cx(styles.td, col.align === 'right' && styles.right, col.align === 'center' && styles.center)}
                    >
                      {col.render ? col.render(row[col.key], row) : String(row[col.key] ?? '—')}
                    </td>
                  ))}
                </tr>
              );
            })
          )}
        </tbody>
      </table>
    </div>
  );
}
