// MIT License
// Copyright (c) Destroyer 2026.
import { useState } from 'react';
import { FilePlus2, FolderOpen } from 'lucide-react';
import { useDiscCommand } from '../hooks/useDiscCommand';
import { useDiskFile } from '../hooks/useDiskFile';
import { useAppStore } from '../store/appStore';
import { Card } from '../components/Card';
import { TextInput } from '../components/TextInput';
import { Button } from '../components/Button';
import { Checkbox } from '../components/Checkbox';
import { StatusIndicator } from '../components/StatusIndicator';
import { useI18n } from '../i18n/useI18n';
import styles from './Panel.module.css';
import createStyles from './CreatePanel.module.css';

export function CreatePanel() {
  const { t } = useI18n();
  const { addOpenDisk } = useAppStore();
  const [path, setPath] = useState('');
  const [tracks, setTracks] = useState('40');
  const [sectors, setSectors] = useState('9');
  const [force, setForce] = useState(false);
  const [status, setStatus] = useState<'idle' | 'success' | 'error'>('idle');
  const { execute, loading } = useDiscCommand();
  const { pickSaveDsk } = useDiskFile();

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
      addOpenDisk(path);
    } else {
      setStatus('error');
    }
  };

  return (
    <div className={styles.panel}>
      <Card>
        <div className={createStyles.form}>
          <div className={createStyles.row}>
            <TextInput
              label={t('create_path')}
              value={path}
              onChange={setPath}
              placeholder="/home/user/new-disk.dsk"
              suffix={
                <button className={createStyles.browseInline} onClick={browse}>
                  <FolderOpen size={13} />
                </button>
              }
            />
          </div>

          <div className={createStyles.row2}>
            <TextInput
              label={t('create_tracks')}
              value={tracks}
              onChange={setTracks}
              type="number"
              hint={t('create_tracks_hint')}
            />
            <TextInput
              label={t('create_sectors')}
              value={sectors}
              onChange={setSectors}
              type="number"
              hint={t('create_sectors_hint')}
            />
          </div>

          <Checkbox
            label={t('create_force')}
            description={t('create_force_hint')}
            checked={force}
            onChange={setForce}
          />

          <div className={createStyles.actions}>
            <Button
              variant="primary"
              icon={<FilePlus2 size={12} />}
              loading={loading}
              disabled={!path}
              onClick={handleCreate}
            >
              {t('create_run')}
            </Button>
            {status !== 'idle' && (
              <StatusIndicator
                status={status}
                label={status === 'success' ? t('create_success') : t('create_error')}
              />
            )}
          </div>
        </div>
      </Card>
    </div>
  );
}
