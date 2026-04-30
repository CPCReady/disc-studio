// MIT License
// Copyright (c) Destroyer 2026.
import { useAppStore } from '../store/appStore';
import { useI18n } from '../i18n/useI18n';
import styles from './Topbar.module.css';

export function Topbar() {
  const { openDisks, activeDiskId } = useAppStore();
  const { t } = useI18n();

  const activeDisk = openDisks.find((d) => d.id === activeDiskId);

  return (
    <header className={styles.topbar}>
      <div className={styles.pathBar}>
        {activeDisk ? (
          <>
            <span className={styles.pathLabel}>{t('explorer_dsk_path')}:</span>
            <span className={styles.pathValue}>{activeDisk.path}</span>
          </>
        ) : (
          <span className={styles.pathPlaceholder}>—</span>
        )}
      </div>
    </header>
  );
}

