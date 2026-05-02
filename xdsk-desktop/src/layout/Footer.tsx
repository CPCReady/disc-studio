// MIT License
// Copyright (c) Destroyer 2026.
import { useEffect, useState } from 'react';
import { getVersion } from '@tauri-apps/api/app';
import { useI18n } from '../i18n/useI18n';
import styles from './Footer.module.css';

export function Footer() {
  const { t } = useI18n();
  const [version, setVersion] = useState<string>('');

  useEffect(() => {
    getVersion().then(setVersion).catch(() => {});
  }, []);

  return (
    <footer className={styles.footer}>
      <span className={styles.copy}>{t('footer_copyright')}</span>
      {version && <span className={styles.version}>v{version}</span>}
    </footer>
  );
}
