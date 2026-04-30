// MIT License
// Copyright (c) Destroyer 2026.
import { useState, useRef, useEffect, type ReactNode } from 'react';
import styles from './ContextMenu.module.css';
import { cx } from '../utils/formatters';

export interface ContextMenuItem {
  id: string;
  label: string;
  icon?: ReactNode;
  onClick: () => void;
  disabled?: boolean;
  danger?: boolean;
}

export interface ContextMenuSeparator {
  id: string;
  separator: true;
}

interface ContextMenuProps {
  items: (ContextMenuItem | ContextMenuSeparator)[];
  trigger: ReactNode;
  className?: string;
}

function isSeparator(item: ContextMenuItem | ContextMenuSeparator): item is ContextMenuSeparator {
  return 'separator' in item;
}

export function ContextMenu({ items, trigger, className }: ContextMenuProps) {
  const [open, setOpen] = useState(false);
  const ref = useRef<HTMLDivElement>(null);

  useEffect(() => {
    if (!open) return;
    const handler = (e: MouseEvent) => {
      if (!ref.current?.contains(e.target as Node)) setOpen(false);
    };
    document.addEventListener('mousedown', handler);
    return () => document.removeEventListener('mousedown', handler);
  }, [open]);

  return (
    <div ref={ref} className={cx(styles.anchor, className)}>
      <div onClick={() => setOpen((o) => !o)}>{trigger}</div>
      {open && (
        <div className={styles.menu}>
          {items.map((item) =>
            isSeparator(item) ? (
              <div key={item.id} className={styles.separator} />
            ) : (
              <button
                key={item.id}
                disabled={item.disabled}
                className={cx(styles.item, item.danger && styles.danger)}
                onClick={() => { item.onClick(); setOpen(false); }}
              >
                {item.icon && <span className={styles.itemIcon}>{item.icon}</span>}
                {item.label}
              </button>
            )
          )}
        </div>
      )}
    </div>
  );
}
