// MIT License
// Copyright (c) Destroyer 2026.
import { useState } from 'react';
import { Plus, X, Save, GitCompare, ScrollText, Settings, Sun, Moon } from 'lucide-react';
import { useAppStore } from '../store/appStore';
import { useThemeStore } from '../store/themeStore';
import { useDiskFile } from '../hooks/useDiskFile';
import { useI18n } from '../i18n/useI18n';
import { CreateModal } from '../components/CreateModal';
import { DiffModal } from '../components/DiffModal';
import { ConsoleModal } from '../components/ConsoleModal';
import { SettingsModal } from '../components/SettingsModal';
import styles from './Sidebar.module.css';

export function Sidebar() {
  const { openDisks, activeDiskId, setActiveDisk, removeOpenDisk } = useAppStore();
  const { theme, toggleTheme } = useThemeStore();
  const { openDisk } = useDiskFile();
  const { t } = useI18n();
  const [createOpen, setCreateOpen] = useState(false);
  const [compareOpen, setCompareOpen] = useState(false);
  const [consoleOpen, setConsoleOpen] = useState(false);
  const [settingsOpen, setSettingsOpen] = useState(false);

  const handleAdd = async () => {
    await openDisk();
  };

  return (
    <aside className={styles.sidebar}>
      {/* Header */}
      <div className={styles.header}>
        <span className={styles.appTitle}>xDSK Desktop</span>
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
              className={`${styles.diskItem}${activeDiskId === disk.id ? ' ' + styles.active : ''}`}
              onClick={() => setActiveDisk(disk.id)}
              title={disk.path}
            >
              <Save size={12} className={styles.diskIcon} />
              <span className={styles.diskLabel}>{disk.label}</span>
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
