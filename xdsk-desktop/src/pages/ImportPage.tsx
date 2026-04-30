// MIT License
// Copyright (c) Destroyer 2026.
// @ts-nocheck
import { useState } from 'react';
import { Upload, FolderOpen, X } from 'lucide-react';
import { useAppStore } from '../store/appStore';
import { useDiscCommand } from '../hooks/useDiscCommand';
import { useDiskFile } from '../hooks/useDiskFile';
import { SectionHeader } from '../components/SectionHeader';
import { Card } from '../components/Card';
import { TextInput } from '../components/TextInput';
import { Select } from '../components/Select';
import { Button } from '../components/Button';
import { Checkbox } from '../components/Checkbox';
import { StatusIndicator } from '../components/StatusIndicator';
import { EmptyState } from '../components/EmptyState';
import { basename } from '../utils/formatters';
import styles from './FormPage.module.css';

const FILE_TYPE_OPTIONS = [
  { value: '',       label: 'Auto-detect' },
  { value: 'binary', label: 'Binary' },
  { value: 'ascii',  label: 'ASCII' },
  { value: 'raw',    label: 'Raw' },
];

export function ImportPage() {
  const { currentDiskPath } = useAppStore();
  const { openDisk, pickFiles } = useDiskFile();
  const { execute, loading } = useDiscCommand();

  const [files, setFiles] = useState<string[]>([]);
  const [fileType, setFileType] = useState('');
  const [loadAddr, setLoadAddr] = useState('');
  const [execAddr, setExecAddr] = useState('');
  const [user, setUser] = useState('0');
  const [readOnly, setReadOnly] = useState(false);
  const [system, setSystem] = useState(false);
  const [force, setForce] = useState(false);
  const [status, setStatus] = useState<'idle' | 'success' | 'error'>('idle');

  const addFiles = async () => {
    const selected = await pickFiles();
    setFiles((prev) => [...new Set([...prev, ...selected])]);
  };

  const removeFile = (path: string) => setFiles((prev) => prev.filter((f) => f !== path));

  const handleImport = async () => {
    if (!currentDiskPath || files.length === 0) return;
    const args: string[] = ['import', currentDiskPath, ...files];
    if (fileType)   args.push('--file-type', fileType);
    if (loadAddr)   args.push('--load', loadAddr);
    if (execAddr)   args.push('--exec', execAddr);
    if (user !== '0') args.push('--user', user);
    if (readOnly)   args.push('--read-only');
    if (system)     args.push('--system');
    if (force)      args.push('--force');
    const result = await execute(args);
    setStatus(result?.success ? 'success' : 'error');
  };

  if (!currentDiskPath) {
    return (
      <EmptyState
        icon={<Upload size={36} />}
        title="No disk image open"
        description="Open a DSK image first."
        action={<Button variant="primary" icon={<FolderOpen size={13} />} onClick={openDisk}>Open DSK Image</Button>}
      />
    );
  }

  return (
    <div className={styles.page}>
      <SectionHeader title="Import Files" description="Add files to the current DSK image." />

      <Card className={styles.card}>
        <div className={styles.form}>
          {/* File list */}
          <div>
            <div style={{ display: 'flex', alignItems: 'center', justifyContent: 'space-between', marginBottom: 'var(--space-3)' }}>
              <span style={{ fontSize: 'var(--font-size-xs)', fontWeight: 500, color: 'var(--text-secondary)' }}>
                Files to import ({files.length})
              </span>
              <Button icon={<FolderOpen size={12} />} onClick={addFiles}>Browse files</Button>
            </div>
            {files.length > 0 && (
              <div className={styles.tags}>
                {files.map((f) => (
                  <span key={f} className={styles.tag}>
                    {basename(f)}
                    <span className={styles.tagRemove} onClick={() => removeFile(f)}><X size={10} /></span>
                  </span>
                ))}
              </div>
            )}
          </div>

          <div className={styles.divider} />

          {/* Options */}
          <div className={styles.row2}>
            <Select label="File type" value={fileType} onChange={setFileType} options={FILE_TYPE_OPTIONS} />
            <TextInput label="User number (0–15)" value={user} onChange={setUser} type="number" />
          </div>

          <div className={styles.row2}>
            <TextInput label="Load address (hex)" value={loadAddr} onChange={setLoadAddr} placeholder="e.g. 0x4000" hint="Binary files only" />
            <TextInput label="Exec address (hex)" value={execAddr} onChange={setExecAddr} placeholder="e.g. 0xC000" hint="Binary files only" />
          </div>

          <div style={{ display: 'flex', gap: 'var(--space-6)' }}>
            <Checkbox label="Read-only" checked={readOnly} onChange={setReadOnly} />
            <Checkbox label="System" checked={system} onChange={setSystem} />
            <Checkbox label="Force overwrite" checked={force} onChange={setForce} />
          </div>

          <div className={styles.footer}>
            <Button
              variant="primary"
              icon={<Upload size={13} />}
              loading={loading}
              disabled={files.length === 0}
              onClick={handleImport}
            >
              Import {files.length > 0 ? `${files.length} file${files.length !== 1 ? 's' : ''}` : ''}
            </Button>
            {status !== 'idle' && (
              <StatusIndicator
                status={status}
                label={status === 'success' ? 'Import successful' : 'Import failed'}
              />
            )}
          </div>
        </div>
      </Card>
    </div>
  );
}
