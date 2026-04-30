// MIT License
// Copyright (c) Destroyer 2026.
import { useEffect, useRef } from 'react';
import { Trash2, ChevronDown, ChevronUp } from 'lucide-react';
import type { ConsoleEntry } from '../types/app';
import { formatTimestamp } from '../utils/formatters';
import { IconButton } from './IconButton';
import styles from './ConsolePanel.module.css';
import { cx } from '../utils/formatters';

interface ConsolePanelProps {
  entries: ConsoleEntry[];
  isOpen: boolean;
  onToggle: () => void;
  onClear: () => void;
  className?: string;
}

export function ConsolePanel({ entries, isOpen, onToggle, onClear, className }: ConsolePanelProps) {
  const bottomRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    if (isOpen) bottomRef.current?.scrollIntoView({ behavior: 'smooth' });
  }, [entries, isOpen]);

  return (
    <div className={cx(styles.panel, !isOpen && styles.closed, className)}>
      <div className={styles.header} onClick={onToggle} style={{ cursor: 'pointer' }}>
        <span className={styles.title}>CONSOLE</span>
        <div className={styles.actions} onClick={(e) => e.stopPropagation()}>
          {isOpen && (
            <IconButton
              icon={<Trash2 size={11} />}
              title="Clear console"
              size="sm"
              onClick={onClear}
            />
          )}
          <IconButton
            icon={isOpen ? <ChevronDown size={12} /> : <ChevronUp size={12} />}
            title={isOpen ? 'Collapse' : 'Expand'}
            size="sm"
            onClick={onToggle}
          />
        </div>
      </div>

      {isOpen && (
        <div className={styles.body}>
          {entries.length === 0 && (
            <span className={cx(styles.entry, styles.info)}>
              <span className={styles.content}>No output yet.</span>
            </span>
          )}
          {entries.map((e) => (
            <div key={e.id} className={cx(styles.entry, styles[e.type])}>
              <span className={styles.ts}>{formatTimestamp(e.timestamp)}</span>
              <span className={styles.content}>{e.content}</span>
            </div>
          ))}
          <div ref={bottomRef} />
        </div>
      )}
    </div>
  );
}
