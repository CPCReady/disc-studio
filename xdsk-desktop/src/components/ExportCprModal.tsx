// MIT License
// Copyright (c) Destroyer 2026.
import { useMemo, useState } from 'react';
import { useEffect } from 'react';
import { FolderOpen, Package } from 'lucide-react';
import { message } from '@tauri-apps/plugin-dialog';
import { Modal } from './Modal';
import { TextInput } from './TextInput';
import { Button } from './Button';
import { useDiskFile } from '../hooks/useDiskFile';
import { useXcartCommand } from '../hooks/useXcartCommand';
import { useI18n } from '../i18n/useI18n';
import styles from './ExportCprModal.module.css';

interface Props {
  open: boolean;
  diskPath: string;
  defaultOutputPath: string;
  initialAutoCommand: string;
  onClose: () => void;
  onDone: () => void;
}

const MAX_AUTOSTART_LEN = 16;

export function ExportCprModal({ open, diskPath, defaultOutputPath, initialAutoCommand, onClose, onDone }: Props) {
  const { t } = useI18n();
  const { pickSaveCpr } = useDiskFile();
  const { execute, loading, error, reset } = useXcartCommand();
  const [outputPath, setOutputPath] = useState(defaultOutputPath);
  const [autoCommand, setAutoCommand] = useState(initialAutoCommand);

  useEffect(() => {
    if (!open) return;
    setOutputPath(defaultOutputPath);
    setAutoCommand(initialAutoCommand);
    reset();
  }, [open, defaultOutputPath, initialAutoCommand, reset]);

  const commandTooLong = autoCommand.length > MAX_AUTOSTART_LEN;
  const canExport = outputPath.trim().length > 0 && !commandTooLong;

  const commandHint = useMemo(() => {
    if (commandTooLong) {
      return t('export_cpr_autostart_too_long').replace('{max}', String(MAX_AUTOSTART_LEN));
    }
    return t('export_cpr_autostart_hint').replace('{max}', String(MAX_AUTOSTART_LEN));
  }, [autoCommand.length, commandTooLong, t]);

  const handleBrowse = async () => {
    const picked = await pickSaveCpr(defaultOutputPath.split('/').pop() ?? 'export.cpr');
    if (picked) setOutputPath(picked);
  };

  const handleClose = () => {
    reset();
    setOutputPath(defaultOutputPath);
    setAutoCommand(initialAutoCommand);
    onClose();
  };

  const handleExport = async () => {
    if (!canExport) return;
    reset();
    const args = ['create', diskPath, outputPath.trim()];
    const cmd = autoCommand.trim();
    if (cmd) args.push('--command', cmd);
    const result = await execute(args);
    if (result?.success) {
      await message(t('export_cpr_done_ok'), { title: t('success'), kind: 'info' });
      onDone();
      return;
    }
    const err = result?.stderr?.trim() || result?.stdout?.trim() || error || t('error');
    await message(`${t('export_cpr_done_fail')}\n\n${err}`, { title: t('error'), kind: 'error' });
  };

  return (
    <Modal
      open={open}
      onClose={handleClose}
      title={t('export_cpr_title')}
      size="md"
      footer={
        <div className={styles.footer}>
          <Button variant="ghost" onClick={handleClose}>{t('cancel')}</Button>
          <Button
            variant="primary"
            icon={<Package size={13} />}
            onClick={handleExport}
            loading={loading}
            disabled={!canExport}
          >
            {t('export_cpr_submit')}
          </Button>
        </div>
      }
    >
      <div className={styles.body}>
        <TextInput
          label={t('export_cpr_output')}
          value={outputPath}
          onChange={setOutputPath}
          placeholder={t('export_cpr_output_placeholder')}
          suffix={
            <button className={styles.browseBtn} onClick={handleBrowse} title={t('export_cpr_browse')}>
              <FolderOpen size={13} />
            </button>
          }
        />

        <TextInput
          label={t('export_cpr_autostart')}
          value={autoCommand}
          onChange={setAutoCommand}
          placeholder={'run"disc"'}
          maxLength={MAX_AUTOSTART_LEN + 8}
          error={commandTooLong ? commandHint : undefined}
          hint={!commandTooLong ? commandHint : undefined}
        />

        {error && <p className={styles.errorMsg}>{error}</p>}
      </div>
    </Modal>
  );
}
