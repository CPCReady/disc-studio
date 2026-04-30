// MIT License
// Copyright (c) Destroyer 2026.
import { useState } from 'react';
import { FolderOpen, Play } from 'lucide-react';
import { useDiscCommand } from '../hooks/useDiscCommand';
import { useDiskFile } from '../hooks/useDiskFile';
import { parseDiff } from '../utils/parsers';
import { Card } from '../components/Card';
import { TextInput } from '../components/TextInput';
import { Button } from '../components/Button';
import { DataTable, type Column } from '../components/DataTable';
import { useI18n } from '../i18n/useI18n';
import type { DiffEntry } from '../types/xdsk';
import styles from './Panel.module.css';
import diffStyles from './DiffPanel.module.css';

const STATUS_COLORS: Record<string, string> = {
  added:     'var(--accent-green)',
  removed:   'var(--accent-red)',
  changed:   'var(--accent-amber)',
  identical: 'var(--text-muted)',
};

const COLUMNS: Column<Record<string, unknown>>[] = [
  {
    key: 'name',
    header: 'File',
    render: (v) => <span style={{ fontFamily: 'var(--font-mono)', fontSize: '12px' }}>{String(v)}</span>,
  },
  {
    key: 'status',
    header: 'Status',
    width: '100px',
    render: (v) => (
      <span style={{ color: STATUS_COLORS[String(v)] || 'var(--text-secondary)', fontSize: 'var(--font-size-xs)', fontWeight: 500, textTransform: 'uppercase' }}>
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

export function DiffPanel() {
  const { t } = useI18n();
  const { execute, loading } = useDiscCommand();
  const { pickDisk } = useDiskFile();
  const [disk1, setDisk1] = useState('');
  const [disk2, setDisk2] = useState('');
  const [entries, setEntries] = useState<DiffEntry[]>([]);
  const [ran, setRan] = useState(false);

  const browse1 = async () => { const p = await pickDisk(); if (p) setDisk1(p); };
  const browse2 = async () => { const p = await pickDisk(); if (p) setDisk2(p); };

  const handleDiff = async () => {
    if (!disk1 || !disk2) return;
    const r = await execute(['diff', disk1, disk2]);
    if (r) { setEntries(parseDiff(r.stdout + r.stderr)); setRan(true); }
  };

  return (
    <div className={styles.panel}>
      <Card>
        <div className={diffStyles.form}>
          <div className={diffStyles.row2}>
            <TextInput
              label={t('diff_disk_a')}
              value={disk1}
              onChange={setDisk1}
              placeholder={t('diff_disk_placeholder')}
              suffix={<button className={diffStyles.browseInline} onClick={browse1}><FolderOpen size={13} /></button>}
            />
            <TextInput
              label={t('diff_disk_b')}
              value={disk2}
              onChange={setDisk2}
              placeholder={t('diff_disk_placeholder')}
              suffix={<button className={diffStyles.browseInline} onClick={browse2}><FolderOpen size={13} /></button>}
            />
          </div>
          <Button
            variant="primary"
            icon={<Play size={12} />}
            loading={loading}
            disabled={!disk1 || !disk2}
            onClick={handleDiff}
          >
            {t('diff_run')}
          </Button>
        </div>
      </Card>

      {ran && (
        <Card title={t('diff_results')}>
          <DataTable
            data={entries as unknown as Record<string, unknown>[]}
            columns={COLUMNS}
            keyField="name"
          />
        </Card>
      )}
    </div>
  );
}
