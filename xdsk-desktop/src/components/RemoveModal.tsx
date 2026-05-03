// MIT License
// Copyright (c) Destroyer 2026.
import { Trash2, TriangleAlert } from 'lucide-react';
import { message } from '@tauri-apps/plugin-dialog';
import { Modal } from './Modal';
import { Button } from './Button';
import { useDiscCommand } from '../hooks/useDiscCommand';
import { useI18n } from '../i18n/useI18n';
import styles from './RemoveModal.module.css';

interface Props {
  open: boolean;
  diskPath: string;
  files: string[];
  onClose: () => void;
  onDone: () => void;
}

export function RemoveModal({ open, diskPath, files, onClose, onDone }: Props) {
  const { t } = useI18n();
  const { execute, loading } = useDiscCommand();

  const handleRemove = async () => {
    let failures = 0;
    for (const fileName of files) {
      const result = await execute(['remove', diskPath, fileName, '--force']);
      if (!result?.success) failures += 1;
    }
    if (failures === 0) {
      await message(t('remove_done_ok'), { title: t('success'), kind: 'info' });
      onDone();
      return;
    }
    await message(`${t('remove_done_fail')} (${failures}/${files.length})`, { title: t('error'), kind: 'error' });
  };

  return (
    <Modal
      open={open}
      onClose={onClose}
      title={t('remove_title')}
      size="sm"
      footer={
        <div className={styles.footer}>
          <Button variant="ghost" onClick={onClose}>{t('cancel')}</Button>
          <Button variant="danger" onClick={handleRemove} loading={loading}>
            <Trash2 size={13} />
            {t('remove_submit')}
          </Button>
        </div>
      }
    >
      <div className={styles.body}>
        {/* Icon badge */}
        <div className={styles.iconBadge}>
          <Trash2 size={22} />
        </div>

        {/* Message */}
        <p className={styles.message}>
          {files.length === 1
            ? t('remove_message_single').replace('{name}', files[0])
            : t('remove_message_multi').replace('{n}', String(files.length))}
        </p>

        {/* File list (only for multi) */}
        {files.length > 1 && (
          <ul className={styles.fileList}>
            {files.map((f) => (
              <li key={f} className={styles.fileItem}>{f}</li>
            ))}
          </ul>
        )}

        {/* Warning */}
        <div className={styles.warningRow}>
          <TriangleAlert size={12} className={styles.warningIcon} />
          <span className={styles.warning}>{t('remove_warning')}</span>
        </div>
      </div>
    </Modal>
  );
}
