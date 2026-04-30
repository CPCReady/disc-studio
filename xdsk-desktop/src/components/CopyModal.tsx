// MIT License
// Copyright (c) Destroyer 2026.
import { useState } from 'react';
import { FolderOpen } from 'lucide-react';
import { Modal } from './Modal';
import { TextInput } from './TextInput';
import { Button } from './Button';
import { useDiskFile } from '../hooks/useDiskFile';
import { useDiscCommand } from '../hooks/useDiscCommand';
import { useI18n } from '../i18n/useI18n';
import styles from './CopyModal.module.css';

interface Props {
  open: boolean;
  sourceDiskPath: string;
  files: string[];
  onClose: () => void;
  onDone: () => void;
}

export function CopyModal({ open, sourceDiskPath, files, onClose, onDone }: Props) {
  const { t } = useI18n();
  const { pickSaveDsk } = useDiskFile();
  const { execute, loading } = useDiscCommand();
  const [destDisk, setDestDisk] = useState('');

  const handleBrowse = async () => {
    const path = await pickSaveDsk();
    if (path) setDestDisk(path);
  };

  const handleCopy = async () => {
    if (!destDisk) return;
    for (const fileName of files) {
      await execute(['copy', sourceDiskPath, fileName, destDisk]);
    }
    onDone();
  };

  const handleClose = () => {
    setDestDisk('');
    onClose();
  };

  return (
    <Modal
      open={open}
      onClose={handleClose}
      title={t('copy_title')}
      size="sm"
      footer={
        <div className={styles.footer}>
          <Button variant="ghost" onClick={handleClose}>{t('cancel')}</Button>
          <Button
            variant="primary"
            onClick={handleCopy}
            loading={loading}
            disabled={!destDisk}
          >
            {t('copy_confirm')}
          </Button>
        </div>
      }
    >
      <div className={styles.body}>
        <p className={styles.desc}>
          {files.length === 1
            ? t('copy_desc_single').replace('{name}', files[0])
            : t('copy_desc_multi').replace('{n}', String(files.length))}
        </p>

        <div className={styles.dirRow}>
          <TextInput
            label={t('copy_dest_disk')}
            value={destDisk}
            onChange={setDestDisk}
            placeholder="path/to/target.dsk"
            className={styles.dirInput}
          />
          <button className={styles.browseBtn} onClick={handleBrowse} title={t('browse')}>
            <FolderOpen size={14} />
          </button>
        </div>
      </div>
    </Modal>
  );
}
