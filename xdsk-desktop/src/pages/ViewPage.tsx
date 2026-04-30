// MIT License
// Copyright (c) Destroyer 2026.
// @ts-nocheck
import { useState } from 'react';
import { Eye, FolderOpen } from 'lucide-react';
import { useAppStore } from '../store/appStore';
import { useDiscCommand } from '../hooks/useDiscCommand';
import { useDiskFile } from '../hooks/useDiskFile';
import { SectionHeader } from '../components/SectionHeader';
import { Card } from '../components/Card';
import { TextInput } from '../components/TextInput';
import { Select } from '../components/Select';
import { Button } from '../components/Button';
import { EmptyState } from '../components/EmptyState';
import styles from './FormPage.module.css';
import viewStyles from './ViewPage.module.css';

const FORMAT_OPTIONS = [
  { value: 'auto',   label: 'Auto-detect' },
  { value: 'basic',  label: 'BASIC listing' },
  { value: 'hex',    label: 'Hex dump' },
  { value: 'ascii',  label: 'ASCII text' },
  { value: 'disasm', label: 'Disassembly' },
];

export function ViewPage() {
  const { currentDiskPath } = useAppStore();
  const { openDisk } = useDiskFile();
  const { execute, loading } = useDiscCommand();
  const [fileName, setFileName] = useState('');
  const [format, setFormat] = useState('auto');
  const [output, setOutput] = useState('');

  const handleView = async () => {
    if (!currentDiskPath || !fileName) return;
    const args = ['view', currentDiskPath, fileName, '--format', format];
    const result = await execute(args);
    setOutput(result ? result.stdout || result.stderr : '');
  };

  if (!currentDiskPath) {
    return (
      <EmptyState
        icon={<Eye size={36} />}
        title="No disk image open"
        action={<Button variant="primary" icon={<FolderOpen size={13} />} onClick={openDisk}>Open DSK Image</Button>}
      />
    );
  }

  return (
    <div className={styles.page}>
      <SectionHeader title="View File" description="Preview the content of a file inside the DSK image." />

      <Card className={styles.card}>
        <div className={styles.form}>
          <div className={styles.row2}>
            <TextInput
              label="File name"
              value={fileName}
              onChange={setFileName}
              placeholder="e.g. GAME.BAS"
            />
            <Select label="Format" value={format} onChange={setFormat} options={FORMAT_OPTIONS} />
          </div>

          <div className={styles.footer}>
            <Button
              variant="primary"
              icon={<Eye size={13} />}
              loading={loading}
              disabled={!fileName}
              onClick={handleView}
            >
              View
            </Button>
          </div>
        </div>
      </Card>

      {output && (
        <Card title="Output" className={viewStyles.resultCard}>
          <pre className={styles.resultBox}>{output}</pre>
        </Card>
      )}
    </div>
  );
}
