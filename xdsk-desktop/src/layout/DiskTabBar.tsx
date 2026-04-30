// MIT License
// Copyright (c) Destroyer 2026.
import { X, Save, GitCompare } from 'lucide-react';
import { useAppStore } from '../store/appStore';
import styles from './DiskTabBar.module.css';

export function DiskTabBar() {
  const {
    openDisks, activeDiskId, setActiveDisk, closeTab,
    compareResults, activeCompareId, setActiveCompareId, removeCompareResult,
  } = useAppStore();

  const tabDisks = openDisks.filter((d) => d.tabOpen);

  if (tabDisks.length === 0 && compareResults.length === 0) return null;

  return (
    <div className={styles.tabBar}>
      {/* Disk tabs */}
      {tabDisks.map((disk) => (
        <button
          key={disk.id}
          className={`${styles.tab}${activeDiskId === disk.id && !activeCompareId ? ' ' + styles.active : ''}`}
          onClick={() => setActiveDisk(disk.id)}
          title={disk.path}
        >
          <Save size={11} className={styles.tabIcon} />
          <span className={styles.tabLabel}>{disk.label}</span>
          <span
            className={styles.closeBtn}
            role="button"
            onClick={(e) => { e.stopPropagation(); closeTab(disk.id); }}
          >
            <X size={10} />
          </span>
        </button>
      ))}

      {/* Compare result tabs */}
      {compareResults.map((cmp) => (
        <button
          key={cmp.id}
          className={`${styles.tab} ${styles.compareTab}${activeCompareId === cmp.id ? ' ' + styles.active : ''}`}
          onClick={() => setActiveCompareId(cmp.id)}
          title={`${cmp.disk1} vs ${cmp.disk2}`}
        >
          <GitCompare size={11} className={styles.tabIcon} />
          <span className={styles.tabLabel}>{cmp.label}</span>
          <span
            className={styles.closeBtn}
            role="button"
            onClick={(e) => { e.stopPropagation(); removeCompareResult(cmp.id); }}
          >
            <X size={10} />
          </span>
        </button>
      ))}
    </div>
  );
}
