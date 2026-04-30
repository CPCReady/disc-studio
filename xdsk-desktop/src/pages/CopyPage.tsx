// MIT License
// Copyright (c) Destroyer 2026.
// @ts-nocheck
import { useState } from 'react';
import { Copy, FolderOpen } from 'lucide-react';
import { useDiscCommand } from '../hooks/useDiscCommand';
import { useDiskFile } from '../hooks/useDiskFile';
import { SectionHeader } from '../components/SectionHeader';
import { Card } from '../components/Card';
import { TextInput } from '../components/TextInput';
import { Button } from '../components/Button';
import { Checkbox } from '../components/Checkbox';
import { StatusIndicator } from '../components/StatusIndicator';
import styles from './FormPage.module.css';

export function CopyPage() {
  const { execute, loading } = useDiscCommand();
  const { pickDisk, pickSaveDsk } = useDiskFile();

  const [src, setSrc] = useState('');
  const [dst, setDst] = useState('');
  const [pattern, setPattern] = useState('*');
  const [force, setForce] = useState(false);
  const [status, setStatus] = useState<'idle' | 'success' | 'error'>('idle');

  const browseSrc = async () => { const p = await pickDisk(); if (p) setSrc(p); };
  const browseDst = async () => { const p = await pickSaveDsk('destination.dsk'); if (p) setDst(p); };

  const handleCopy = async () => {
    if (!src || !dst) return;
    const patterns = pattern.split(/\s+/).filter(Boolean);
    const args = ['copy', src, dst, ...patterns];
    if (force) args.push('--force');
    const r = await execute(args);
    setStatus(r?.success ? 'success' : 'error');
  };

  return (
    <div className={styles.page}>
      <SectionHeader title="Copy Files" description="Copy files between two DSK images." />

      <Card className={styles.card}>
        <div className={styles.form}>
          <div className={styles.row2}>
            <TextInput
              label="Source disk"
              value={src}
              onChange={setSrc}
              placeholder="Source DSK path"
              suffix={<button style={{ padding: '0 6px', cursor: 'pointer', color: 'var(--text-muted)' }} onClick={browseSrc}><FolderOpen size={13} /></button>}
            />
            <TextInput
              label="Destination disk"
              value={dst}
              onChange={setDst}
              placeholder="Destination DSK path"
              suffix={<button style={{ padding: '0 6px', cursor: 'pointer', color: 'var(--text-muted)' }} onClick={browseDst}><FolderOpen size={13} /></button>}
            />
          </div>

          <TextInput
            label="File pattern"
            value={pattern}
            onChange={setPattern}
            placeholder="* or *.BAS — use * for all files"
            hint="Separate multiple patterns with spaces."
          />

          <Checkbox
            label="Force overwrite"
            description="Overwrite existing files in the destination."
            checked={force}
            onChange={setForce}
          />

          <div className={styles.footer}>
            <Button
              variant="primary"
              icon={<Copy size={13} />}
              loading={loading}
              disabled={!src || !dst}
              onClick={handleCopy}
            >
              Copy
            </Button>
            {status !== 'idle' && (
              <StatusIndicator
                status={status}
                label={status === 'success' ? 'Copy complete' : 'Copy failed'}
              />
            )}
          </div>
        </div>
      </Card>
    </div>
  );
}
