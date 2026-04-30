// MIT License
// Copyright (c) Destroyer 2026.
import { useAppStore } from '../store/appStore';
import { DataTable, type Column } from './DataTable';
import { Card } from './Card';
import { useI18n } from '../i18n/useI18n';
import styles from './DiffResultView.module.css';

const STATUS_COLORS: Record<string, string> = {
  'only-a':  'var(--accent-red)',
  'only-b':  'var(--accent-green)',
  modified:  'var(--accent-amber)',
  identical: 'var(--text-muted)',
};

const STATUS_LABELS: Record<string, string> = {
  'only-a':  'Only A',
  'only-b':  'Only B',
  modified:  'Modified',
  identical: 'Identical',
};

const COLUMNS: Column<Record<string, unknown>>[] = [
  {
    key: 'name',
    header: 'File',
    render: (v) => (
      <span style={{ fontFamily: 'var(--font-mono)', fontSize: '12px' }}>{String(v)}</span>
    ),
  },
  {
    key: 'status',
    header: 'Status',
    width: '100px',
    render: (v) => (
      <span style={{
        color: STATUS_COLORS[String(v)] ?? 'var(--text-secondary)',
        fontSize: 'var(--font-size-xs)',
        fontWeight: 500,
        textTransform: 'uppercase',
      }}>
        {STATUS_LABELS[String(v)] ?? String(v)}
      </span>
    ),
  },
  {
    key: 'sizeA',
    header: 'Size A',
    width: '80px',
    render: (v) => v != null
      ? <span style={{ fontFamily: 'var(--font-mono)', fontSize: '12px', color: 'var(--text-secondary)' }}>{String(v)}</span>
      : <span style={{ color: 'var(--text-muted)' }}>—</span>,
  },
  {
    key: 'sizeB',
    header: 'Size B',
    width: '80px',
    render: (v) => v != null
      ? <span style={{ fontFamily: 'var(--font-mono)', fontSize: '12px', color: 'var(--text-secondary)' }}>{String(v)}</span>
      : <span style={{ color: 'var(--text-muted)' }}>—</span>,
  },
];

interface Props {
  compareId: string;
}

export function DiffResultView({ compareId }: Props) {
  const { t } = useI18n();
  const result = useAppStore((s) => s.compareResults.find((r) => r.id === compareId));

  if (!result) return null;

  const counts = {
    onlyA: result.entries.filter((e) => e.status === 'only-a').length,
    onlyB: result.entries.filter((e) => e.status === 'only-b').length,
    modified: result.entries.filter((e) => e.status === 'modified').length,
    identical: result.entries.filter((e) => e.status === 'identical').length,
  };

  return (
    <div className={styles.container}>
      {/* Header */}
      <div className={styles.header}>
        <div className={styles.disks}>
          <div className={styles.diskBadge}>
            <span className={styles.diskLabel}>A</span>
            <span className={styles.diskPath}>{result.disk1}</span>
          </div>
          <span className={styles.vs}>{t('diff_vs')}</span>
          <div className={styles.diskBadge}>
            <span className={styles.diskLabel}>B</span>
            <span className={styles.diskPath}>{result.disk2}</span>
          </div>
        </div>

        {/* Summary chips */}
        <div className={styles.summary}>
          {counts.onlyA > 0 && (
            <span className={styles.chip} style={{ color: STATUS_COLORS['only-a'] }}>
              {counts.onlyA} {t('diff_status_only_a')}
            </span>
          )}
          {counts.onlyB > 0 && (
            <span className={styles.chip} style={{ color: STATUS_COLORS['only-b'] }}>
              {counts.onlyB} {t('diff_status_only_b')}
            </span>
          )}
          {counts.modified > 0 && (
            <span className={styles.chip} style={{ color: STATUS_COLORS['modified'] }}>
              {counts.modified} {t('diff_status_modified')}
            </span>
          )}
          {counts.identical > 0 && (
            <span className={styles.chip} style={{ color: STATUS_COLORS['identical'] }}>
              {counts.identical} {t('diff_status_identical')}
            </span>
          )}
        </div>
      </div>

      {/* Table */}
      <Card className={styles.tableCard}>
        {result.entries.length === 0 ? (
          <p className={styles.empty}>{t('diff_no_results')}</p>
        ) : (
          <DataTable
            keyField="name"
            columns={COLUMNS}
            data={result.entries as unknown as Record<string, unknown>[]}
          />
        )}
      </Card>
    </div>
  );
}
