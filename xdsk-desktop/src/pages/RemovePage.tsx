// MIT License
// Copyright (c) Destroyer 2026.
// @ts-nocheck
import { useState, useEffect, useCallback } from 'react';
import { Trash2, FolderOpen, RefreshCw } from 'lucide-react';
import { useAppStore } from '../store/appStore';
import { useDiscCommand } from '../hooks/useDiscCommand';
import { useDiskFile } from '../hooks/useDiskFile';
import { parseListJson } from '../utils/parsers';
import { SectionHeader } from '../components/SectionHeader';
import { Toolbar, ToolbarSeparator } from '../components/Toolbar';
import { Button } from '../components/Button';
import { IconButton } from '../components/IconButton';
import { DataTable, type Column } from '../components/DataTable';
import { Modal } from '../components/Modal';
import { EmptyState } from '../components/EmptyState';
import { StatusIndicator } from '../components/StatusIndicator';
import { formatBytes, fileTypeLabel } from '../utils/formatters';
import type { DiskFile } from '../types/disc';
import styles from './DiskListPage.module.css';

const COLUMNS: Column<DiskFile & Record<string, unknown>>[] = [
  { key: 'name', header: 'Name', render: (v) => <span style={{ fontFamily: 'var(--font-mono)', fontSize: '12px' }}>{String(v)}</span> },
  { key: 'type', header: 'Type', render: (v) => fileTypeLabel(String(v)) },
  { key: 'size', header: 'Size', align: 'right', render: (v) => formatBytes(Number(v)) },
];

export function RemovePage() {
  const { currentDiskPath } = useAppStore();
  const { openDisk } = useDiskFile();
  const { execute, loading } = useDiscCommand();
  const [files, setFiles] = useState<DiskFile[]>([]);
  const [selected, setSelected] = useState<string[]>([]);
  const [confirmOpen, setConfirmOpen] = useState(false);
  const [status, setStatus] = useState<'idle' | 'success' | 'error'>('idle');

  const refresh = useCallback(async () => {
    if (!currentDiskPath) return;
    const result = await execute(['list', currentDiskPath, '--format', 'json']);
    if (result?.success) {
      setFiles(parseListJson(result.stdout).files);
      setSelected([]);
      setStatus('idle');
    }
  }, [currentDiskPath, execute]);

  useEffect(() => { refresh(); }, [refresh]);

  const handleRemove = async () => {
    if (!currentDiskPath || selected.length === 0) return;
    setConfirmOpen(false);
    const result = await execute(['remove', currentDiskPath, ...selected, '--force']);
    setStatus(result?.success ? 'success' : 'error');
    if (result?.success) refresh();
  };

  if (!currentDiskPath) {
    return (
      <EmptyState
        icon={<Trash2 size={36} />}
        title="No disk image open"
        action={<Button variant="primary" icon={<FolderOpen size={13} />} onClick={openDisk}>Open DSK Image</Button>}
      />
    );
  }

  return (
    <div className={styles.page}>
      <SectionHeader
        title="Remove Files"
        description="Select files to delete from the DSK image."
        actions={<StatusIndicator status={loading ? 'loading' : status === 'idle' ? 'idle' : status} label={loading ? 'Working…' : status === 'success' ? 'Removed' : status === 'error' ? 'Error' : 'Ready'} size="sm" />}
      />

      <Toolbar>
        <Button
          variant="danger"
          icon={<Trash2 size={12} />}
          disabled={selected.length === 0 || loading}
          onClick={() => setConfirmOpen(true)}
        >
          Remove {selected.length > 0 ? `(${selected.length})` : ''}
        </Button>
        <ToolbarSeparator />
        <IconButton icon={<RefreshCw size={13} />} title="Refresh" onClick={refresh} />
      </Toolbar>

      <DataTable
        columns={COLUMNS as Column<Record<string, unknown>>[]}
        data={files as unknown as Record<string, unknown>[]}
        keyField="name"
        loading={loading}
        selectable
        selectedKeys={selected}
        onSelectionChange={setSelected}
        emptyMessage="Disk image is empty"
      />

      <Modal
        open={confirmOpen}
        title="Remove files"
        onClose={() => setConfirmOpen(false)}
        footer={
          <>
            <Button onClick={() => setConfirmOpen(false)}>Cancel</Button>
            <Button variant="danger" onClick={handleRemove}>Remove {selected.length} file{selected.length !== 1 ? 's' : ''}</Button>
          </>
        }
      >
        <p style={{ color: 'var(--text-secondary)', fontSize: 'var(--font-size-sm)' }}>
          This will permanently delete <strong style={{ color: 'var(--text-primary)' }}>{selected.length} file{selected.length !== 1 ? 's' : ''}</strong> from the disk image. This action cannot be undone.
        </p>
      </Modal>
    </div>
  );
}
