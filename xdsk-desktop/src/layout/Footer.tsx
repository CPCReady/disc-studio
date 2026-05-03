// MIT License
// Copyright (c) Destroyer 2026.
import { useEffect, useState } from 'react';
import { getVersion } from '@tauri-apps/api/app';
import { useAppStore } from '../store/appStore';
import { useI18n } from '../i18n/useI18n';
import styles from './Footer.module.css';

export function Footer() {
  const { t } = useI18n();
  const { openDisks, activeDiskId } = useAppStore();
  const [version, setVersion] = useState<string>('');
  const activeDisk = openDisks.find((d) => d.id === activeDiskId);

  useEffect(() => {
    getVersion().then(setVersion).catch(() => {});
  }, []);

  return (
    <footer className={styles.footer}>
      <div className={styles.pathBar}>
        <span className={styles.pathLabel}>{t('explorer_dsk_path')}:</span>
        <span className={styles.pathValue}>{activeDisk?.path ?? '—'}</span>
      </div>
      <div className={styles.meta}>
        <span className={styles.copy}>{t('footer_copyright')}</span>
        {version && <span className={styles.version}>v{version}</span>}
      </div>
    </footer>
  );
}
