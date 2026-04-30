// MIT License
// Copyright (c) Destroyer 2026.
// @ts-nocheck
import { useState } from 'react';
import { Download, FolderOpen } from 'lucide-react';
import { useAppStore } from '../store/appStore';
import { useDiscCommand } from '../hooks/useDiscCommand';
import { useDiskFile } from '../hooks/useDiskFile';
import { SectionHeader } from '../components/SectionHeader';
import { Card } from '../components/Card';
import { TextInput } from '../components/TextInput';
import { Button } from '../components/Button';
import { Checkbox } from '../components/Checkbox';
import { StatusIndicator } from '../components/StatusIndicator';
import { EmptyState } from '../components/EmptyState';
import styles from './FormPage.module.css';

export function ExportPage() {
  const { currentDiskPath } = useAppStore();
  const { openDisk, pickDirectory } = useDiskFile();
  const { execute, loading } = useDiscCommand();

  const [pattern, setPattern] = useState('*');
  const [outputDir, setOutputDir] = useState('');
  const [stripHeader, setStripHeader] = useState(false);
  const [status, setStatus] = useState<'idle' | 'success' | 'error'>('idle');

  const browseDir = async () => {
    const d = await pickDirectory();
    if (d) setOutputDir(d);
  };

  const handleExport = async () => {
    if (!currentDiskPath) return;
    const patterns = pattern.split(/\s+/).filter(Boolean);
    const args: string[] = ['export', currentDiskPath, ...patterns];
    if (outputDir)   args.push('--output', outputDir);
    if (stripHeader) args.push('--strip-header');
    const result = await execute(args);
    setStatus(result?.success ? 'success' : 'error');
  };

  if (!currentDiskPath) {
    return (
      <EmptyState
        icon={<Download size={36} />}
        title="No disk image open"
        description="Open a DSK image first."
        action={<Button variant="primary" icon={<FolderOpen size={13} />} onClick={openDisk}>Open DSK Image</Button>}
      />
    );
  }

  return (
    <div className={styles.page}>
      <SectionHeader title="Export Files" description="Extract files from the current DSK image." />

      <Card className={styles.card}>
        <div className={styles.form}>
          <TextInput
            label="File pattern"
            value={pattern}
            onChange={setPattern}
            placeholder="* or *.BAS or LEVEL* or EXACT"
            hint="Use * for all files. Separate multiple patterns with spaces."
          />

          <div>
            <TextInput
              label="Output directory"
              value={outputDir}
              onChange={setOutputDir}
              placeholder="Leave empty to use current directory"
              suffix={
                <button style={{ padding: '0 6px', cursor: 'pointer', color: 'var(--text-muted)' }} onClick={browseDir}>
                  <FolderOpen size={13} />
                </button>
              }
            />
          </div>

          <Checkbox
            label="Strip AMSDOS header"
            description="Export raw file data without the 128-byte AMSDOS header."
            checked={stripHeader}
            onChange={setStripHeader}
          />

          <div className={styles.footer}>
            <Button
              variant="primary"
              icon={<Download size={13} />}
              loading={loading}
              disabled={!pattern}
              onClick={handleExport}
            >
              Export
            </Button>
            {status !== 'idle' && (
              <StatusIndicator
                status={status}
                label={status === 'success' ? 'Export successful' : 'Export failed'}
              />
            )}
          </div>
        </div>
      </Card>
    </div>
  );
}
