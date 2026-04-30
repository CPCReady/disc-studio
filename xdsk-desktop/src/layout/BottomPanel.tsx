// MIT License
// Copyright (c) Destroyer 2026.
import { useRef, useState, useCallback } from 'react';
import { GripHorizontal } from 'lucide-react';
import { useAppStore } from '../store/appStore';
import { useI18n } from '../i18n/useI18n';
import { ViewPanel } from '../panels/ViewPanel';
import type { BottomTabId } from '../types/app';
import styles from './BottomPanel.module.css';

const MIN_HEIGHT = 120;
const DEFAULT_HEIGHT = 200;

export function BottomPanel() {
  const {
    activeBottomTab,
    setActiveBottomTab,
    isBottomPanelOpen,
    toggleBottomPanel,
  } = useAppStore();
  const { t } = useI18n();
  const [panelHeight, setPanelHeight] = useState(DEFAULT_HEIGHT);
  const dragStart = useRef<{ y: number; h: number } | null>(null);

  const handleResizeMouseDown = useCallback((e: React.MouseEvent) => {
    e.preventDefault();
    dragStart.current = { y: e.clientY, h: panelHeight };
    document.body.style.cursor = 'ns-resize';

    const onMove = (ev: MouseEvent) => {
      if (!dragStart.current) return;
      const delta = dragStart.current.y - ev.clientY;
      setPanelHeight(Math.max(MIN_HEIGHT, dragStart.current.h + delta));
    };
    const onUp = () => {
      dragStart.current = null;
      document.body.style.cursor = '';
      window.removeEventListener('mousemove', onMove);
      window.removeEventListener('mouseup', onUp);
    };
    window.addEventListener('mousemove', onMove);
    window.addEventListener('mouseup', onUp);
  }, [panelHeight]);

  const TABS: { id: BottomTabId; label: string }[] = [
    { id: 'view',    label: t('panel_view') },
  ];

  const handleTabClick = (id: BottomTabId) => {
    if (id === activeBottomTab && isBottomPanelOpen) {
      toggleBottomPanel();
    } else {
      setActiveBottomTab(id);
    }
  };

  return (
    <div
      className={`${styles.panel}${isBottomPanelOpen ? ' ' + styles.open : ''}`}
      style={isBottomPanelOpen ? { height: panelHeight } : undefined}
    >
      {/* Resize handle — only visible when open */}
      {isBottomPanelOpen && (
        <div className={styles.resizeHandle} onMouseDown={handleResizeMouseDown}>
          <GripHorizontal size={14} className={styles.resizeIcon} />
        </div>
      )}

      {/* Tab bar */}
      <div className={styles.tabBar}>
        <div className={styles.tabs}>
          {TABS.map((tab) => (
            <button
              key={tab.id}
              className={`${styles.tab}${activeBottomTab === tab.id ? ' ' + styles.activeTab : ''}`}
              onClick={() => handleTabClick(tab.id)}
            >
              {tab.label}
            </button>
          ))}
        </div>

        <div className={styles.tabBarRight}>
          <button className={styles.toggleBtn} onClick={toggleBottomPanel} title={isBottomPanelOpen ? 'Collapse' : 'Expand'}>
            <GripHorizontal size={13} />
          </button>
        </div>
      </div>

      {isBottomPanelOpen && (
        <div className={styles.content}>
          {activeBottomTab === 'view' && <ViewPanel />}
        </div>
      )}
    </div>
  );
}
