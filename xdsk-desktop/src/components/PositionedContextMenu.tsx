// MIT License
// Copyright (c) Destroyer 2026.
import { useState, useRef } from 'react';
import { createPortal } from 'react-dom';
import { ChevronRight } from 'lucide-react';
import styles from './ContextMenu.module.css';

export interface PositionedMenuItem {
  label?: string;
  onClick?: () => void;
  danger?: boolean;
  disabled?: boolean;
  type?: 'separator';
  submenu?: PositionedMenuItem[];
}

interface Props {
  x: number;
  y: number;
  items: PositionedMenuItem[];
  onClose: () => void;
}

export function PositionedContextMenu({ x, y, items, onClose }: Props) {
  const [activeSubmenu, setActiveSubmenu] = useState<number | null>(null);
  const [submenuPos, setSubmenuPos] = useState({ x: 0, y: 0 });
  const closeTimer = useRef<ReturnType<typeof setTimeout> | null>(null);

  const clearCloseTimer = () => {
    if (closeTimer.current) {
      clearTimeout(closeTimer.current);
      closeTimer.current = null;
    }
  };

  const scheduleClose = () => {
    clearCloseTimer();
    closeTimer.current = setTimeout(() => setActiveSubmenu(null), 150);
  };

  const handleParentEnter = (i: number, e: React.MouseEvent<HTMLDivElement>) => {
    clearCloseTimer();
    const rect = e.currentTarget.getBoundingClientRect();
    setSubmenuPos({ x: rect.right, y: rect.top });
    setActiveSubmenu(i);
  };

  const activeItems = activeSubmenu !== null ? items[activeSubmenu]?.submenu : undefined;

  return createPortal(
    <div
      className={styles.backdrop}
      onMouseDown={onClose}
      onContextMenu={(e) => { e.preventDefault(); onClose(); }}
    >
      {/* Menú principal */}
      <div
        className={styles.posMenu}
        style={{ left: x, top: y }}
        onMouseDown={(e) => e.stopPropagation()}
      >
        {items.map((item, i) =>
          item.type === 'separator' ? (
            <div key={i} className={styles.separator} />
          ) : item.submenu ? (
            // div en vez de button para evitar botones anidados
            <div
              key={i}
              role="menuitem"
              tabIndex={0}
              className={`${styles.item} ${styles.hasSubmenu}`}
              onMouseEnter={(e) => handleParentEnter(i, e)}
              onMouseLeave={scheduleClose}
            >
              <span className={styles.itemLabel}>{item.label}</span>
              <ChevronRight size={12} className={styles.chevron} />
            </div>
          ) : (
            <button
              key={i}
              className={`${styles.item}${item.danger ? ' ' + styles.danger : ''}`}
              disabled={item.disabled}
              onMouseEnter={() => { clearCloseTimer(); setActiveSubmenu(null); }}
              onClick={() => { item.onClick?.(); onClose(); }}
            >
              {item.label}
            </button>
          )
        )}
      </div>

      {/* Submenú renderizado como hermano, no dentro del item padre */}
      {activeSubmenu !== null && activeItems && (
        <div
          className={styles.posMenu}
          style={{ position: 'fixed', left: submenuPos.x, top: submenuPos.y }}
          onMouseEnter={clearCloseTimer}
          onMouseLeave={scheduleClose}
          onMouseDown={(e) => e.stopPropagation()}
        >
          {activeItems.map((sub, j) =>
            sub.type === 'separator' ? (
              <div key={j} className={styles.separator} />
            ) : (
              <button
                key={j}
                className={`${styles.item}${sub.danger ? ' ' + styles.danger : ''}`}
                disabled={sub.disabled}
                onClick={() => { sub.onClick?.(); onClose(); }}
              >
                {sub.label}
              </button>
            )
          )}
        </div>
      )}
    </div>,
    document.body
  );
}
