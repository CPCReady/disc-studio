// MIT License
// Copyright (c) Destroyer 2026.
// @ts-nocheck
import { useState, useEffect, useCallback } from 'react';
import { RefreshCw, FolderOpen, Download, Trash2, Lock, Monitor } from 'lucide-react';
import { useAppStore } from '../store/appStore';
import { useDiskFile } from '../hooks/useDiskFile';
import { useDiscCommand } from '../hooks/useDiscCommand';
import { parseListJson } from '../utils/parsers';
import { formatBytes, fileTypeLabel } from '../utils/formatters';
import { SectionHeader } from '../components/SectionHeader';
import { Toolbar, ToolbarSeparator, ToolbarSpacer } from '../components/Toolbar';
import { Button } from '../components/Button';
import { IconButton } from '../components/IconButton';
import { DataTable, type Column } from '../components/DataTable';
import { EmptyState } from '../components/EmptyState';
import { StatusIndicator } from '../components/StatusIndicator';
import type { DiskFile } from '../types/xdsk';
import styles from './DiskListPage.module.css';

const COLUMNS: Column<DiskFile & Record<string, unknown>>[] = [
  {
    key: 'name',
    header: 'Name',
    width: '40%',
    render: (v) => (
      <span style={{ fontFamily: 'var(--font-mono)', fontSize: '12px' }}>{String(v)}</span>
    ),
  },
  {
    key: 'type',
    header: 'Type',
    width: '15%',
    render: (v) => fileTypeLabel(String(v)),
  },
  {
    key: 'size',
    header: 'Size',
    width: '15%',
    align: 'right',
    render: (v) => formatBytes(Number(v)),
  },
  {
    key: 'read_only',
    header: 'R/O',
    width: '10%',
    align: 'center',
    render: (v) => v ? <Lock size={11} style={{ color: 'var(--accent-amber)' }} /> : <span style={{ color: 'var(--text-muted)' }}>—</span>,
  },
  {
    key: 'system',
    header: 'SYS',
    width: '10%',
    align: 'center',
    render: (v) => v ? <Monitor size={11} style={{ color: 'var(--accent-blue)' }} /> : <span style={{ color: 'var(--text-muted)' }}>—</span>,
  },
];

export function DiskListPage() {
  const { currentDiskPath, setCurrentPage } = useAppStore();
  const { openDisk } = useDiskFile();
  const [files, setFiles] = useState<DiskFile[]>([]);
  const [totalSize, setTotalSize] = useState(0);
  const [freeSpace, setFreeSpace] = useState(0);
  const [selectedKeys, setSelectedKeys] = useState<string[]>([]);
  const { execute, loading } = useDiscCommand();

  const refresh = useCallback(async () => {
    if (!currentDiskPath) return;
    const result = await execute(['list', currentDiskPath, '--format', 'json']);
    if (result?.success) {
      const parsed = parseListJson(result.stdout);
      setFiles(parsed.files);
      setTotalSize(parsed.total_size);
      setFreeSpace(parsed.free_space);
      setSelectedKeys([]);
    }
  }, [currentDiskPath, execute]);

  useEffect(() => { refresh(); }, [refresh]);

  if (!currentDiskPath) {
    return (
      <div className={styles.page}>
        <EmptyState
          icon={<FolderOpen size={40} />}
          title="No disk image open"
          description="Open a DSK image to browse its contents."
          action={<Button variant="primary" icon={<FolderOpen size={13} />} onClick={openDisk}>Open DSK Image</Button>}
        />
      </div>
    );
  }

  const usedSpace = totalSize - freeSpace;

  return (
    <div className={styles.page}>
      <SectionHeader
        title="Explore"
        description={`${files.length} file${files.length !== 1 ? 's' : ''} · ${formatBytes(usedSpace)} used · ${formatBytes(freeSpace)} free`}
        actions={
          <StatusIndicator
            status={loading ? 'loading' : 'idle'}
            label={loading ? 'Loading…' : 'Ready'}
            size="sm"
          />
        }
      />

      <Toolbar>
        <Button icon={<FolderOpen size={12} />} onClick={openDisk}>Open</Button>
        <ToolbarSeparator />
        <IconButton icon={<RefreshCw size={13} />} title="Refresh" onClick={refresh} />
        <ToolbarSeparator />
        <Button
          icon={<Download size={12} />}
          disabled={selectedKeys.length === 0}
          onClick={() => setCurrentPage('export')}
        >
          Export selected
        </Button>
        <Button
          variant="danger"
          icon={<Trash2 size={12} />}
          disabled={selectedKeys.length === 0}
          onClick={() => setCurrentPage('remove')}
        >
          Remove
        </Button>
        <ToolbarSpacer />
        <span style={{ fontSize: 'var(--font-size-xs)', color: 'var(--text-muted)' }}>
          {selectedKeys.length > 0 ? `${selectedKeys.length} selected` : ''}
        </span>
      </Toolbar>

      <DataTable
        columns={COLUMNS as Column<Record<string, unknown>>[]}
        data={files as unknown as Record<string, unknown>[]}
        keyField="name"
        loading={loading}
        selectable
        selectedKeys={selectedKeys}
        onSelectionChange={setSelectedKeys}
        emptyMessage="Disk image is empty"
      />
    </div>
  );
}
