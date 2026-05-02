// MIT License
// Copyright (c) Destroyer 2026.
import { useState } from 'react';
import { FilePlus2, FolderOpen } from 'lucide-react';
import { Modal } from './Modal';
import { TextInput } from './TextInput';
import { Select } from './Select';
import { Button } from './Button';
import { Checkbox } from './Checkbox';
import { useDiscCommand } from '../hooks/useDiscCommand';
import { useDiskFile } from '../hooks/useDiskFile';
import { useAppStore } from '../store/appStore';
import { useI18n } from '../i18n/useI18n';
import styles from './CreateModal.module.css';

interface Props {
  open: boolean;
  onClose: () => void;
}

const TRACKS_OPTIONS = [{ value: '40', label: '40' }];
const SECTORS_OPTIONS = [{ value: '9', label: '9' }];

export function CreateModal({ open, onClose }: Props) {
  const { t } = useI18n();
  const { execute, loading } = useDiscCommand();
  const { pickSaveDsk } = useDiskFile();
  const addOpenDisk = useAppStore((s) => s.addOpenDisk);

  const [path, setPath]       = useState('');
  const [tracks, setTracks]   = useState('40');
  const [sectors, setSectors] = useState('9');
  const [force, setForce]     = useState(false);
  const [error, setError]     = useState<string | null>(null);

  const browse = async () => {
    const p = await pickSaveDsk('new-disk.dsk');
    if (p) setPath(p);
  };

  const handleCreate = async () => {
    if (!path) return;
    setError(null);
    const args = ['create', path, '--tracks', tracks, '--sectors', sectors];
    if (force) args.push('--force');
    const result = await execute(args);
    if (result?.success) {
      addOpenDisk(path);
      setPath('');
      setTracks('40');
      setSectors('9');
      setForce(false);
      onClose();
    } else {
      setError(result?.stderr ?? t('create_error'));
    }
  };

  const canCreate = path.trim() !== '';

  return (
    <Modal
      open={open}
      onClose={onClose}
      title={t('create_title')}
      size="md"
      footer={
        <div className={styles.footer}>
          <Button variant="ghost" onClick={onClose}>{t('cancel')}</Button>
          <Button
            variant="primary"
            icon={<FilePlus2 size={13} />}
            onClick={handleCreate}
            loading={loading}
            disabled={!canCreate}
          >
            {t('create_submit')}
          </Button>
        </div>
      }
    >
      <div className={styles.body}>
        <div className={styles.pathRow}>
          <TextInput
            label={t('create_output')}
            value={path}
            onChange={setPath}
            placeholder={t('create_output_placeholder')}
            className={styles.input}
            suffix={
              <button className={styles.browseBtn} onClick={browse} title={t('create_browse')}>
                <FolderOpen size={13} />
              </button>
            }
          />
        </div>

        <div className={styles.numbersRow}>
          <Select
            label={t('create_tracks')}
            value={tracks}
            onChange={setTracks}
            options={TRACKS_OPTIONS}
          />
          <Select
            label={t('create_sectors')}
            value={sectors}
            onChange={setSectors}
            options={SECTORS_OPTIONS}
          />
        </div>

        <Checkbox
          label={t('create_force')}
          description={t('create_force_hint')}
          checked={force}
          onChange={setForce}
        />

        {error && <p className={styles.errorMsg}>{error}</p>}
      </div>
    </Modal>
  );
}
