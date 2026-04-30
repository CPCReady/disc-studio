// MIT License
// Copyright (c) Destroyer 2026.
import { useState } from 'react';
import { FolderOpen } from 'lucide-react';
import { Modal } from './Modal';
import { TextInput } from './TextInput';
import { Button } from './Button';
import { useDiscCommand } from '../hooks/useDiscCommand';
import { useDiskFile } from '../hooks/useDiskFile';
import { parseDiff } from '../utils/parsers';
import { useAppStore } from '../store/appStore';
import { useI18n } from '../i18n/useI18n';
import styles from './DiffModal.module.css';

interface Props {
  open: boolean;
  onClose: () => void;
}

export function DiffModal({ open, onClose }: Props) {
  const { t } = useI18n();
  const { execute, loading } = useDiscCommand();
  const { pickDisk } = useDiskFile();
  const addCompareResult = useAppStore((s) => s.addCompareResult);
  const [disk1, setDisk1] = useState('');
  const [disk2, setDisk2] = useState('');

  const browse1 = async () => { const p = await pickDisk(); if (p) setDisk1(p); };
  const browse2 = async () => { const p = await pickDisk(); if (p) setDisk2(p); };

  const handleRun = async () => {
    if (!disk1 || !disk2) return;
    const r = await execute(['diff', disk1, disk2]);
    if (r) {
      const entries = parseDiff(r.stdout + r.stderr);
      addCompareResult(disk1, disk2, entries);
      onClose();
    }
  };

  const canRun = disk1.trim() !== '' && disk2.trim() !== '';

  return (
    <Modal
      open={open}
      onClose={onClose}
      title={t('diff_title')}
      size="md"
      footer={
        <div className={styles.footer}>
          <Button variant="ghost" onClick={onClose}>{t('cancel')}</Button>
          <Button
            variant="primary"
            onClick={handleRun}
            loading={loading}
            disabled={!canRun}
          >
            {t('diff_run')}
          </Button>
        </div>
      }
    >
      <div className={styles.body}>
        <div className={styles.diskRow}>
          <TextInput
            label={t('diff_disk_a')}
            value={disk1}
            onChange={setDisk1}
            placeholder={t('diff_disk_placeholder')}
            className={styles.input}
            suffix={
              <button className={styles.browseBtn} onClick={browse1} title={t('diff_browse')}>
                <FolderOpen size={13} />
              </button>
            }
          />
        </div>
        <div className={styles.diskRow}>
          <TextInput
            label={t('diff_disk_b')}
            value={disk2}
            onChange={setDisk2}
            placeholder={t('diff_disk_placeholder')}
            className={styles.input}
            suffix={
              <button className={styles.browseBtn} onClick={browse2} title={t('diff_browse')}>
                <FolderOpen size={13} />
              </button>
            }
          />
        </div>
      </div>
    </Modal>
  );
}
