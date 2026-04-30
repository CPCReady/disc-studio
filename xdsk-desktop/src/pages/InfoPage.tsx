// MIT License
// Copyright (c) Destroyer 2026.
// @ts-nocheck
import { useState, useEffect, useCallback } from 'react';
import { Info, FolderOpen } from 'lucide-react';
import { useAppStore } from '../store/appStore';
import { useDiscCommand } from '../hooks/useDiscCommand';
import { useDiskFile } from '../hooks/useDiskFile';
import { parseInfo } from '../utils/parsers';
import type { DiskInfo } from '../types/disc';

import { SectionHeader } from '../components/SectionHeader';
import { Card } from '../components/Card';
import { Button } from '../components/Button';
import { EmptyState } from '../components/EmptyState';
import styles from './InfoPage.module.css';

function infoCards(info: Partial<DiskInfo>): Array<{ label: string; value: string }> {
  const items: Array<{ label: string; value: string }> = [];
  if (info.format)        items.push({ label: 'Format',          value: info.format });
  if (info.tracks)        items.push({ label: 'Tracks',          value: String(info.tracks) });
  if (info.sectors)       items.push({ label: 'Sectors / Track', value: String(info.sectors) });
  if (info.sectorSize)    items.push({ label: 'Sector Size',     value: `${info.sectorSize} B` });
  if (info.capacity)      items.push({ label: 'Capacity',        value: info.capacity });
  if (info.entriesUsed !== undefined && info.entriesTotal !== undefined)
    items.push({ label: 'Directory', value: `${info.entriesUsed} / ${info.entriesTotal}` });
  if (info.usedBlocks !== undefined) items.push({ label: 'Used Blocks', value: String(info.usedBlocks) });
  if (info.freeBlocks !== undefined) items.push({ label: 'Free Blocks', value: String(info.freeBlocks) });
  return items;
}

export function InfoPage() {
  const { currentDiskPath } = useAppStore();
  const { openDisk } = useDiskFile();
  const { execute } = useDiscCommand();
  const [info, setInfo] = useState<Partial<DiskInfo> | null>(null);
  const [rawText, setRawText] = useState('');

  const refresh = useCallback(async () => {
    if (!currentDiskPath) return;
    const r = await execute(['info', currentDiskPath]);
    if (r?.success) { setInfo(parseInfo(r.stdout)); setRawText(r.stdout); }
  }, [currentDiskPath, execute]);

  useEffect(() => { refresh(); }, [refresh]);

  if (!currentDiskPath) {
    return (
      <EmptyState
        icon={<Info size={36} />}
        title="No disk image open"
        action={<Button variant="primary" icon={<FolderOpen size={13} />} onClick={openDisk}>Open DSK Image</Button>}
      />
    );
  }

  if (!info) return null;

  const items = infoCards(info);

  return (
    <div className={styles.page}>
      <SectionHeader title="Disk Info" description="Geometry and usage information for the current DSK image." />

      <div className={styles.grid}>
        {items.map(({ label, value }) => (
          <Card key={label} className={styles.infoCard}>
            <p className={styles.label}>{label}</p>
            <p className={styles.value}>{value}</p>
          </Card>
        ))}
      </div>

      {rawText && (
        <Card title="Raw output" className={styles.rawCard}>
          <pre className={styles.rawBox}>{rawText}</pre>
        </Card>
      )}
    </div>
  );
}
