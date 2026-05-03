// MIT License
// Copyright (c) Destroyer 2026.
import { useState, useCallback, useRef } from 'react';
import { Plus, X, Save, GitCompare, ScrollText, Settings, Sun, Moon } from 'lucide-react';
import { useAppStore } from '../store/appStore';
import { useThemeStore } from '../store/themeStore';
import { useDiskFile } from '../hooks/useDiskFile';
import { useI18n } from '../i18n/useI18n';
import { useSettingsStore } from '../store/settingsStore';
import { CreateModal } from '../components/CreateModal';
import { DiffModal } from '../components/DiffModal';
import { ConsoleModal } from '../components/ConsoleModal';
import { SettingsModal } from '../components/SettingsModal';
import { cx } from '../utils/formatters';
import styles from './Sidebar.module.css';

const MIN_WIDTH = 240;
const MAX_WIDTH = 460;

export function Sidebar() {
  const { openDisks, activeDiskId, setActiveDisk, removeOpenDisk, diskHealth } = useAppStore();
  const { theme, toggleTheme } = useThemeStore();
  const { openDisk } = useDiskFile();
  const { t } = useI18n();
  const { leftSidebarWidth, setLeftSidebarWidth } = useSettingsStore();
  const [createOpen, setCreateOpen] = useState(false);
  const [compareOpen, setCompareOpen] = useState(false);
  const [consoleOpen, setConsoleOpen] = useState(false);
  const [settingsOpen, setSettingsOpen] = useState(false);
  const [width, setWidth] = useState(leftSidebarWidth);
  const currentWidth = useRef(leftSidebarWidth);
  const dragStart = useRef<{ x: number; w: number } | null>(null);

  const handleAdd = async () => {
    await openDisk();
  };

  const handleResizeMouseDown = useCallback((e: React.MouseEvent) => {
    e.preventDefault();
    dragStart.current = { x: e.clientX, w: currentWidth.current };
    document.body.style.cursor = 'ew-resize';
    const onMove = (ev: MouseEvent) => {
      if (!dragStart.current) return;
      const delta = ev.clientX - dragStart.current.x;
      const next = Math.min(MAX_WIDTH, Math.max(MIN_WIDTH, dragStart.current.w + delta));
      currentWidth.current = next;
      setWidth(next);
    };
    const onUp = () => {
      setLeftSidebarWidth(currentWidth.current);
      dragStart.current = null;
      document.body.style.cursor = '';
      window.removeEventListener('mousemove', onMove);
      window.removeEventListener('mouseup', onUp);
    };
    window.addEventListener('mousemove', onMove);
    window.addEventListener('mouseup', onUp);
  }, [setLeftSidebarWidth]);

  return (
    <aside className={styles.sidebar} style={{ width }}>
      <div className={styles.resizeHandle} onMouseDown={handleResizeMouseDown} />
      {/* Header */}
      <div className={styles.header}>
        <span className={styles.appBrand}>
          <span className={styles.brandAccent}>xDSK</span>
          <span className={styles.brandName}>Explorer</span>
        </span>
        <button className={styles.addBtn} onClick={handleAdd} title={t('sidebar_add_disk')}>
          <Plus size={14} />
        </button>
      </div>

      {/* Disk list */}
      <div className={styles.diskList}>
        {openDisks.length === 0 ? (
          <div className={styles.empty}>
              <Save size={24} className={styles.emptyIcon} />
            <p className={styles.emptyText}>{t('sidebar_no_disks')}</p>
            <p className={styles.emptyHint}>{t('sidebar_no_disks_hint')}</p>
          </div>
        ) : (
          openDisks.map((disk) => (
            <div
              key={disk.id}
              className={cx(styles.diskItem, activeDiskId === disk.id && styles.active)}
              onClick={() => setActiveDisk(disk.id)}
              title={disk.path}
            >
              <div className={styles.diskCardContent}>
                <Save size={12} className={styles.diskIcon} />
                <span className={styles.diskLabel}>{disk.label}</span>
                {diskHealth[disk.id] && (
                  <span
                    className={cx(
                      styles.healthTag,
                      diskHealth[disk.id].level === 'ok' && styles.healthTagOk,
                      diskHealth[disk.id].level === 'warning' && styles.healthTagWarning,
                      diskHealth[disk.id].level === 'error' && styles.healthTagError,
                    )}
                  >
                    {diskHealth[disk.id].level === 'ok' && 'CHK'}
                    {diskHealth[disk.id].level === 'warning' && 'WARN'}
                    {diskHealth[disk.id].level === 'error' && 'ERR'}
                    {diskHealth[disk.id].level === 'unknown' && '—'}
                  </span>
                )}
              </div>
              <button
                className={styles.closeBtn}
                onClick={(e) => { e.stopPropagation(); removeOpenDisk(disk.id); }}
                title={t('sidebar_close_disk')}
              >
                <X size={11} />
              </button>
            </div>
          ))
        )}
      </div>

      {/* Footer con botones */}
      <div className={styles.footer}>
        <button className={styles.footerBtn} onClick={() => setCreateOpen(true)}>
          <Save size={13} />
          <span>{t('topbar_create_dsk')}</span>
        </button>
        <button className={styles.footerBtn} onClick={() => setCompareOpen(true)}>
          <GitCompare size={13} />
          <span>{t('topbar_compare')}</span>
        </button>
        <button className={styles.footerBtn} onClick={() => setConsoleOpen(true)}>
          <ScrollText size={13} />
          <span>{t('topbar_console')}</span>
        </button>
        <div className={styles.footerDivider} />
        <button className={styles.footerBtn} onClick={() => setSettingsOpen(true)}>
          <Settings size={13} />
          <span>{t('topbar_settings')}</span>
        </button>
        <button className={styles.footerBtn} onClick={toggleTheme}>
          {theme === 'dark' ? <Sun size={13} /> : <Moon size={13} />}
          <span>{t('topbar_theme')}</span>
        </button>
      </div>

      <CreateModal open={createOpen} onClose={() => setCreateOpen(false)} />
      <DiffModal open={compareOpen} onClose={() => setCompareOpen(false)} />
      <ConsoleModal open={consoleOpen} onClose={() => setConsoleOpen(false)} />
      <SettingsModal isOpen={settingsOpen} onClose={() => setSettingsOpen(false)} />
    </aside>
  );
}
