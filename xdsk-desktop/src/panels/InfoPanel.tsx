// MIT License
// Copyright (c) Destroyer 2026.
import { useState, useEffect, useCallback } from 'react';
import { Info } from 'lucide-react';
import { useAppStore } from '../store/appStore';
import { useDiscCommand } from '../hooks/useDiscCommand';
import { EmptyState } from '../components/EmptyState';
import { useI18n } from '../i18n/useI18n';
import styles from './Panel.module.css';
import s from './InfoPanel.module.css';

interface InfoData {
  disk: Array<{ label: string; value: string }>;
  directory: Array<{ label: string; value: string }>;
  blockMap: Array<{ label: string; value: string }>;
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

function InfoTable({ rows }: { rows: Array<{ label: string; value: string }> }) {
  if (rows.length === 0) return null;
  return (
    <table className={s.table}>
      <tbody>
        {rows.map(({ label, value }) => (
          <tr key={label} className={s.row}>
            <td className={s.key}>{label}</td>
            <td className={s.val}>{value}</td>
          </tr>
        ))}
      </tbody>
    </table>
  );
}

export function InfoPanel() {
  const { activeDiskId, openDisks } = useAppStore();
  const { t } = useI18n();
  const { execute } = useDiscCommand();
  const [info, setInfo] = useState<InfoData | null>(null);

  const activeDisk = openDisks.find((d) => d.id === activeDiskId);

  const refresh = useCallback(async () => {
    if (!activeDisk) return;
    const r = await execute(['info', activeDisk.path]);
    if (r?.success) setInfo(parseInfo(r.stdout));
  }, [activeDisk, execute]);

  useEffect(() => { refresh(); }, [refresh]);

  if (!activeDisk) {
    return <EmptyState icon={<Info size={28} />} title={t('info_no_disk')} />;
  }

  if (!info) return null;

  const sections: Array<{ title: string; rows: InfoData[keyof InfoData] }> = [
    { title: 'Disk',      rows: info.disk },
    { title: 'Directory', rows: info.directory },
    { title: 'Block Map', rows: info.blockMap },
  ];

  return (
    <div className={styles.panel}>
      <div className={s.grid}>
        {sections.filter(sec => sec.rows.length > 0).map(sec => (
          <div key={sec.title} className={s.section}>
            <div className={s.sectionTitle}>{sec.title}</div>
            <InfoTable rows={sec.rows} />
          </div>
        ))}
      </div>
    </div>
  );
}
