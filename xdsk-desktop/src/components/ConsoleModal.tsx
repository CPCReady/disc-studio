// MIT License
// Copyright (c) Destroyer 2026.
import { useRef, useEffect } from 'react';
import { X, Trash2 } from 'lucide-react';
import { useAppStore } from '../store/appStore';
import { createPortal } from 'react-dom';
import styles from './ConsoleModal.module.css';

interface Props {
  open: boolean;
  onClose: () => void;
}

export function ConsoleModal({ open, onClose }: Props) {
  const { consoleEntries, clearConsole } = useAppStore();
  const bottomRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    if (open) bottomRef.current?.scrollIntoView({ behavior: 'smooth' });
  }, [open, consoleEntries.length]);

  if (!open) return null;

  return createPortal(
    <div className={styles.backdrop} onMouseDown={onClose}>
      <div className={styles.modal} onMouseDown={(e) => e.stopPropagation()}>
        {/* Header */}
        <div className={styles.header}>
          <span className={styles.title}>Registro de comandos</span>
          <div className={styles.headerActions}>
            <button className={styles.iconBtn} title="Limpiar" onClick={clearConsole}>
              <Trash2 size={13} />
            </button>
            <button className={styles.iconBtn} title="Cerrar" onClick={onClose}>
              <X size={13} />
            </button>
          </div>
        </div>

        {/* Log entries */}
        <div className={styles.body}>
          {consoleEntries.length === 0 ? (
            <span className={styles.empty}>Sin entradas.</span>
          ) : (
            consoleEntries.map((entry) => (
              <div key={entry.id} className={`${styles.entry} ${styles[entry.type]}`}>
                <span className={styles.ts}>
                  {new Date(entry.timestamp).toLocaleTimeString()}
                </span>
                <span className={styles.content}>{entry.content}</span>
              </div>
            ))
          )}
          <div ref={bottomRef} />
        </div>
      </div>
    </div>,
    document.body
  );
}
