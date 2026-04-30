// MIT License
// Copyright (c) Destroyer 2026.
import type { ReactNode } from 'react';
import { Sidebar } from './Sidebar';
import { Topbar } from './Topbar';
import { DiskTabBar } from './DiskTabBar';
import { BottomPanel } from './BottomPanel';
import { RightSidebar } from './RightSidebar';
import { useSettingsStore } from '../store/settingsStore';
import styles from './AppShell.module.css';

interface Props {
  children: ReactNode;
}

export function AppShell({ children }: Props) {
  const { fontSize } = useSettingsStore();
  return (
    <div className={styles.shell} data-font-size={fontSize}>
      <Sidebar />
      <div className={styles.main}>
        <Topbar />
        <DiskTabBar />
        <div className={styles.content}>{children}</div>
        <BottomPanel />
      </div>
      <RightSidebar />
    </div>
  );
}
