// MIT License
// Copyright (c) Destroyer 2026.
import { useState } from 'react';
import { FolderOpen } from 'lucide-react';
import { message } from '@tauri-apps/plugin-dialog';
import { Modal } from './Modal';
import { TextInput } from './TextInput';
import { Button } from './Button';
import { useDiskFile } from '../hooks/useDiskFile';
import { useDiscCommand } from '../hooks/useDiscCommand';
import { useI18n } from '../i18n/useI18n';
import styles from './ExportModal.module.css';

interface Props {
  open: boolean;
  diskPath: string;
  files: string[];
  onClose: () => void;
  onDone: () => void;
}

export function ExportModal({ open, diskPath, files, onClose, onDone }: Props) {
  const { t } = useI18n();
  const { pickDirectory } = useDiskFile();
  const { execute, loading } = useDiscCommand();
  const [outputDir, setOutputDir] = useState('');
  const [stripHeader, setStripHeader] = useState(false);

  const handleBrowse = async () => {
    const dir = await pickDirectory();
    if (dir) setOutputDir(dir);
  };

  const handleExport = async () => {
    if (!outputDir) return;
    let failures = 0;
    for (const fileName of files) {
      const args = ['export', diskPath, fileName, '--output', outputDir];
      if (stripHeader) args.push('--strip-header');
      const result = await execute(args);
      if (!result?.success) failures += 1;
    }
    if (failures === 0) {
      await message(t('export_done_ok'), { title: t('success'), kind: 'info' });
      onDone();
      return;
    }
    await message(`${t('export_done_fail')} (${failures}/${files.length})`, { title: t('error'), kind: 'error' });
  };

  return (
    <Modal
      open={open}
      onClose={onClose}
      title={t('export_title')}
      size="sm"
      footer={
        <div className={styles.footer}>
          <Button variant="ghost" onClick={onClose}>{t('cancel')}</Button>
          <Button
            variant="primary"
            onClick={handleExport}
            loading={loading}
            disabled={!outputDir}
          >
            {t('export_confirm')}
          </Button>
        </div>
      }
    >
      <div className={styles.body}>
        <p className={styles.fileList}>
          {files.length === 1
            ? files[0]
            : t('export_files_count').replace('{n}', String(files.length))}
        </p>

        <div className={styles.dirRow}>
          <TextInput
            label={t('export_output_dir')}
            value={outputDir}
            onChange={setOutputDir}
            placeholder="/path/to/folder"
            className={styles.dirInput}
          />
          <button className={styles.browseBtn} onClick={handleBrowse} title={t('browse')}>
            <FolderOpen size={14} />
          </button>
        </div>

        <label className={styles.checkRow}>
          <input
            type="checkbox"
            checked={stripHeader}
            onChange={(e) => setStripHeader(e.target.checked)}
            className={styles.checkbox}
          />
          <div className={styles.checkInfo}>
            <span className={styles.checkLabel}>{t('export_strip_header')}</span>
            <span className={styles.checkDesc}>{t('export_strip_header_desc')}</span>
          </div>
        </label>
      </div>
    </Modal>
  );
}
