// MIT License
// Copyright (c) Destroyer 2026.
import { useState, useEffect, useCallback, useRef } from 'react';
import { Save } from 'lucide-react';
import { useAppStore } from '../store/appStore';
import { useDiscCommand } from '../hooks/useDiscCommand';
import { useI18n } from '../i18n/useI18n';
import { useSettingsStore } from '../store/settingsStore';
import { CheckPanel } from '../panels/CheckPanel';
import styles from './RightSidebar.module.css';

const MIN_WIDTH = 160;
const MAX_WIDTH = 480;

interface InfoRow { label: string; value: string }
interface InfoData {
  disk: InfoRow[];
  directory: InfoRow[];
  blockMap: InfoRow[];
}

function parseInfo(raw: string): InfoData {
  const data: InfoData = { disk: [], directory: [], blockMap: [] };
  let section: keyof InfoData | null = null;

  for (const line of raw.split('\n')) {
    const t = line.trim();
    if (!t) continue;
    if (/\.dsk/i.test(t) || /^[\u2501\u2500=\-]{5,}/.test(t)) continue;
    if (/^files/i.test(t)) break;

    if (/^directory/i.test(t))  { section = 'directory'; continue; }
    if (/^block.?map/i.test(t)) { section = 'blockMap';  continue; }
    if (/^format|^tracks|^sectors|^sector.?size|^capacity/i.test(t)) section = 'disk';

    if (!section) continue;
    const m = t.match(/^([A-Za-z][A-Za-z0-9 /()]+?)\s{2,}(.+)$/) ||
               t.match(/^([A-Za-z][A-Za-z0-9 /()]+?)\s+(.+)$/);
    if (m) data[section].push({ label: m[1].trim(), value: m[2].trim() });
  }
  return data;
}

export function RightSidebar() {
  const { activeDiskId, openDisks, diskHealth, triggerCheck } = useAppStore();
  const { execute } = useDiscCommand();
  const { t } = useI18n();
  const { rightSidebarWidth, setRightSidebarWidth } = useSettingsStore();
  const [info, setInfo] = useState<InfoData | null>(null);
  const [width, setWidth] = useState(rightSidebarWidth);
  const currentWidth = useRef(rightSidebarWidth);
  const dragStart = useRef<{ x: number; w: number } | null>(null);

  const handleResizeMouseDown = useCallback((e: React.MouseEvent) => {
    e.preventDefault();
    dragStart.current = { x: e.clientX, w: currentWidth.current };
    document.body.style.cursor = 'ew-resize';
    const onMove = (ev: MouseEvent) => {
      if (!dragStart.current) return;
      const delta = dragStart.current.x - ev.clientX;
      const next = Math.min(MAX_WIDTH, Math.max(MIN_WIDTH, dragStart.current.w + delta));
      currentWidth.current = next;
      setWidth(next);
    };
    const onUp = () => {
      setRightSidebarWidth(currentWidth.current);
      dragStart.current = null;
      document.body.style.cursor = '';
      window.removeEventListener('mousemove', onMove);
      window.removeEventListener('mouseup', onUp);
    };
    window.addEventListener('mousemove', onMove);
    window.addEventListener('mouseup', onUp);
  }, [setRightSidebarWidth]);

  const activeDisk = openDisks.find((d) => d.id === activeDiskId);
  const health = activeDisk ? diskHealth[activeDisk.id] : null;

  const formatCheckedAt = (iso: string | null) => {
    if (!iso) return '—';
    const d = new Date(iso);
    if (Number.isNaN(d.getTime())) return '—';
    return d.toLocaleTimeString();
  };

  const refresh = useCallback(async () => {
    if (!activeDisk) { setInfo(null); return; }
    const r = await execute(['info', activeDisk.path]);
    if (r?.success) setInfo(parseInfo(r.stdout));
  }, [activeDisk, execute]);

  useEffect(() => { refresh(); }, [refresh]);

  const sections: Array<{ title: string; key: keyof InfoData }> = [
    { title: t('right_section_disk'),      key: 'disk' },
    { title: t('right_section_directory'), key: 'directory' },
    { title: t('right_section_blockmap'),  key: 'blockMap' },
  ];

  return (
    <aside className={styles.sidebar} style={{ width }}>
      {/* Drag handle en el borde izquierdo */}
      <div className={styles.resizeHandle} onMouseDown={handleResizeMouseDown} />

      <div className={styles.header}>
        <span className={styles.title}>{t('right_info')}</span>
      </div>

      <div className={styles.body}>
        {!activeDisk ? (
          <div className={styles.empty}>
            <Save size={24} className={styles.emptyIcon} />
            <p>{t('right_no_disk')}</p>
          </div>
        ) : !info ? (
          <div className={styles.empty}><p>{t('loading')}</p></div>
        ) : (
          <>
            <section className={styles.section}>
              <span className={styles.sectionTitle}>{t('right_health')}</span>
              <div className={styles.healthCard}>
                <div className={styles.healthTop}>
                  <span className={styles.key}>{t('right_health_state')}</span>
                  <span className={`${styles.healthBadge} ${styles[`health_${health?.level ?? 'unknown'}`]}`}>
                    {health?.level === 'ok'
                      ? t('right_health_ok')
                      : health?.level === 'warning'
                        ? t('right_health_warning')
                        : health?.level === 'error'
                          ? t('right_health_error')
                          : t('right_health_unknown')}
                  </span>
                </div>
                <div className={styles.healthMeta}>
                  <span>{t('right_health_errors')}: {health?.errors ?? 0}</span>
                  <span>{t('right_health_warnings')}: {health?.warnings ?? 0}</span>
                  <span>{t('right_health_checked')}: {formatCheckedAt(health?.checkedAt ?? null)}</span>
                </div>
                <button className={styles.healthActionBtn} onClick={triggerCheck}>
                  {t('right_health_run_check')}
                </button>
              </div>
            </section>

            {sections.map(({ title, key }) => {
              const rows = info[key];
              if (rows.length === 0) return null;
              return (
                <section key={key} className={styles.section}>
                  <span className={styles.sectionTitle}>{title}</span>
                  <table className={styles.table}>
                    <tbody>
                      {rows.map(({ label, value }) => (
                        <tr key={label}>
                          <td className={styles.key}>{label}</td>
                          <td className={styles.val}>{value}</td>
                        </tr>
                      ))}
                    </tbody>
                  </table>
                </section>
              );
            })}
          </>
        )}
      </div>

      {/* Check panel debajo */}
      {activeDisk && (
        <div className={styles.checkWrapper}>
          <div className={styles.checkDivider}>
            <span className={styles.sectionTitle}>{t('right_verification')}</span>
          </div>
          <CheckPanel />
        </div>
      )}
    </aside>
  );
}
