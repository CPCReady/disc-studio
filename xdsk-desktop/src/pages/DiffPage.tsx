// MIT License
// Copyright (c) Destroyer 2026.
// @ts-nocheck
import { useState } from 'react';
import { FolderOpen, Play } from 'lucide-react';
import { useDiscCommand } from '../hooks/useDiscCommand';
import { useDiskFile } from '../hooks/useDiskFile';
import { parseDiff } from '../utils/parsers';
import { SectionHeader } from '../components/SectionHeader';
import { Card } from '../components/Card';
import { TextInput } from '../components/TextInput';
import { Button } from '../components/Button';
import { DataTable, type Column } from '../components/DataTable';
import type { DiffEntry } from '../types/disc';
import styles from './FormPage.module.css';
import diffStyles from './DiffPage.module.css';

const STATUS_COLORS: Record<string, string> = {
  added:    'var(--accent-green)',
  removed:  'var(--accent-red)',
  changed:  'var(--accent-amber)',
  identical:'var(--text-muted)',
};

const COLUMNS: Column<DiffEntry & Record<string, unknown>>[] = [
  {
    key: 'name',
    header: 'File',
    render: (v) => <span style={{ fontFamily: 'var(--font-mono)', fontSize: '12px' }}>{String(v)}</span>,
  },
  {
    key: 'status',
    header: 'Status',
    width: '120px',
    render: (v) => (
      <span style={{ color: STATUS_COLORS[String(v)] || 'var(--text-secondary)', fontSize: 'var(--font-size-xs)', fontWeight: 500, textTransform: 'uppercase', letterSpacing: '0.04em' }}>
        {String(v)}
      </span>
    ),
  },
  {
    key: 'detail',
    header: 'Details',
    render: (v) => v ? <span style={{ color: 'var(--text-muted)', fontSize: 'var(--font-size-xs)' }}>{String(v)}</span> : null,
  },
];

export function DiffPage() {
  const { execute, loading } = useDiscCommand();
  const { pickDisk } = useDiskFile();

  const [disk1, setDisk1] = useState('');
  const [disk2, setDisk2] = useState('');
  const [entries, setEntries] = useState<DiffEntry[]>([]);
  const [ran, setRan] = useState(false);

  const browseDisk1 = async () => { const p = await pickDisk(); if (p) setDisk1(p); };
  const browseDisk2 = async () => { const p = await pickDisk(); if (p) setDisk2(p); };

  const handleDiff = async () => {
    if (!disk1 || !disk2) return;
    const r = await execute(['diff', disk1, disk2]);
    if (r) { setEntries(parseDiff(r.stdout + r.stderr)); setRan(true); }
  };

  return (
    <div className={styles.page}>
      <SectionHeader title="Diff Disks" description="Compare two DSK images and show file differences." />

      <Card className={styles.card}>
        <div className={styles.form}>
          <div className={styles.row2}>
            <TextInput
              label="Disk A"
              value={disk1}
              onChange={setDisk1}
              placeholder="Path to first DSK"
              suffix={<button style={{ padding: '0 6px', cursor: 'pointer', color: 'var(--text-muted)' }} onClick={browseDisk1}><FolderOpen size={13} /></button>}
            />
            <TextInput
              label="Disk B"
              value={disk2}
              onChange={setDisk2}
              placeholder="Path to second DSK"
              suffix={<button style={{ padding: '0 6px', cursor: 'pointer', color: 'var(--text-muted)' }} onClick={browseDisk2}><FolderOpen size={13} /></button>}
            />
          </div>
          <div className={styles.footer}>
            <Button
              variant="primary"
              icon={<Play size={13} />}
              loading={loading}
              disabled={!disk1 || !disk2}
              onClick={handleDiff}
            >
              Compare
            </Button>
          </div>
        </div>
      </Card>

      {ran && (
        <div className={diffStyles.results}>
          <DataTable
            columns={COLUMNS as Column<Record<string, unknown>>[]}
            data={entries as unknown as Record<string, unknown>[]}
            keyField="name"
            loading={loading}
            emptyMessage="No differences found — disks are identical"
          />
        </div>
      )}
    </div>
  );
}
