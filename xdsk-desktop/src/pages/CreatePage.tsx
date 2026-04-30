// MIT License
// Copyright (c) Destroyer 2026.
// @ts-nocheck
import { useState } from 'react';
import { FilePlus2, FolderOpen } from 'lucide-react';
import { useDiscCommand } from '../hooks/useDiscCommand';
import { useDiskFile } from '../hooks/useDiskFile';
import { SectionHeader } from '../components/SectionHeader';
import { Card } from '../components/Card';
import { TextInput } from '../components/TextInput';
import { Button } from '../components/Button';
import { Checkbox } from '../components/Checkbox';
import { StatusIndicator } from '../components/StatusIndicator';
import styles from './FormPage.module.css';

export function CreatePage() {
  const [path, setPath] = useState('');
  const [tracks, setTracks] = useState('40');
  const [sectors, setSectors] = useState('9');
  const [force, setForce] = useState(false);
  const [status, setStatus] = useState<'idle' | 'success' | 'error'>('idle');
  const { execute, loading } = useDiscCommand();
  const { pickSaveDsk, setCurrentDiskPath } = useDiskFile();

  const browse = async () => {
    const p = await pickSaveDsk('new-disk.dsk');
    if (p) setPath(p);
  };

  const handleCreate = async () => {
    if (!path) return;
    const args = ['create', path, '--tracks', tracks, '--sectors', sectors];
    if (force) args.push('--force');
    const result = await execute(args);
    if (result?.success) {
      setStatus('success');
      setCurrentDiskPath(path);
    } else {
      setStatus('error');
    }
  };

  return (
    <div className={styles.page}>
      <SectionHeader
        title="Create Disk Image"
        description="Create a new blank Amstrad CPC DSK image."
      />

      <Card className={styles.card}>
        <div className={styles.form}>
          <div className={styles.row}>
            <TextInput
              label="Output path"
              value={path}
              onChange={setPath}
              placeholder="e.g. /home/user/new-disk.dsk"
              suffix={
                <button style={{ padding: '0 6px', cursor: 'pointer', color: 'var(--text-muted)' }} onClick={browse}>
                  <FolderOpen size={13} />
                </button>
              }
            />
          </div>

          <div className={styles.row2}>
            <TextInput
              label="Tracks"
              value={tracks}
              onChange={setTracks}
              type="number"
              hint="Standard: 40"
            />
            <TextInput
              label="Sectors per track"
              value={sectors}
              onChange={setSectors}
              type="number"
              hint="Standard: 9"
            />
          </div>

          <Checkbox
            label="Overwrite if exists"
            description="Use --force to overwrite an existing file."
            checked={force}
            onChange={setForce}
          />

          <div className={styles.footer}>
            <Button
              variant="primary"
              icon={<FilePlus2 size={13} />}
              loading={loading}
              disabled={!path}
              onClick={handleCreate}
            >
              Create Image
            </Button>
            {status !== 'idle' && (
              <StatusIndicator
                status={status}
                label={status === 'success' ? 'Image created successfully' : 'Creation failed'}
              />
            )}
          </div>
        </div>
      </Card>
    </div>
  );
}
