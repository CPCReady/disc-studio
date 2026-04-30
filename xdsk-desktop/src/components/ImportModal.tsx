// MIT License
// Copyright (c) Destroyer 2026.
import { useState } from 'react';
import { FolderOpen } from 'lucide-react';
import { open as openDialog } from '@tauri-apps/plugin-dialog';
import { Modal } from './Modal';
import { Button } from './Button';
import { useDiscCommand } from '../hooks/useDiscCommand';
import { useI18n } from '../i18n/useI18n';
import styles from './ImportModal.module.css';

interface Props {
  open: boolean;
  diskPath: string;
  onClose: () => void;
  onDone: () => void;
}

type FileType = 'auto' | 'binary' | 'ascii';

interface ImportState {
  filePaths: string[];
  fileType: FileType;
  loadAddress: string;
  execAddress: string;
  user: string;
  readOnly: boolean;
  system: boolean;
  force: boolean;
}

const INITIAL: ImportState = {
  filePaths: [],
  fileType: 'auto',
  loadAddress: '',
  execAddress: '',
  user: '0',
  readOnly: false,
  system: false,
  force: false,
};

export function ImportModal({ open, diskPath, onClose, onDone }: Props) {
  const { t } = useI18n();
  const { execute, loading } = useDiscCommand();
  const [state, setState] = useState<ImportState>(INITIAL);

  const set = <K extends keyof ImportState>(key: K, value: ImportState[K]) =>
    setState((prev) => ({ ...prev, [key]: value }));

  const handleBrowse = async () => {
    const picked = await openDialog({ multiple: true, directory: false });
    if (!picked) return;
    const paths = Array.isArray(picked) ? picked : [picked];
    if (paths.length > 0) set('filePaths', paths);
  };

  const buildArgs = (filePath: string): string[] => {
    const args: string[] = ['import', diskPath, filePath];
    if (state.fileType !== 'auto') args.push('--file-type', state.fileType);
    if (state.fileType !== 'ascii') {
      if (state.loadAddress) args.push('--load', state.loadAddress);
      if (state.execAddress) args.push('--exec', state.execAddress);
    }
    if (state.user !== '0' && state.user !== '') args.push('--user', state.user);
    if (state.readOnly) args.push('--read-only');
    if (state.system) args.push('--system');
    if (state.force) args.push('--force');
    return args;
  };

  const handleImport = async () => {
    if (state.filePaths.length === 0) return;
    for (const fp of state.filePaths) {
      await execute(buildArgs(fp));
    }
    setState(INITIAL);
    onDone();
  };

  const handleClose = () => {
    setState(INITIAL);
    onClose();
  };

  const showBinaryFields = state.fileType === 'auto' || state.fileType === 'binary';

  return (
    <Modal
      open={open}
      onClose={handleClose}
      title={t('import_title')}
      size="md"
      footer={
        <div className={styles.footer}>
          <Button variant="ghost" onClick={handleClose}>{t('cancel')}</Button>
          <Button
            variant="primary"
            onClick={handleImport}
            loading={loading}
            disabled={state.filePaths.length === 0}
          >
            {t('import_submit')}
          </Button>
        </div>
      }
    >
      <div className={styles.body}>
        {/* File picker */}
        <div className={styles.field}>
          <label className={styles.label}>{t('import_files')}</label>
          <div className={styles.fileRow}>
            <div className={styles.fileNames}>
              {state.filePaths.length === 0
                ? <span className={styles.placeholder}>{t('import_browse')}</span>
                : state.filePaths.map((f) => (
                  <span key={f} className={styles.fileName}>{f.split('/').pop()}</span>
                ))
              }
            </div>
            <button className={styles.browseBtn} onClick={handleBrowse}>
              <FolderOpen size={13} />
              {t('import_add_files')}
            </button>
          </div>
        </div>

        {/* File type */}
        <div className={styles.row2}>
          <div className={styles.field}>
            <label className={styles.label}>{t('import_file_type')}</label>
            <select
              className={styles.select}
              value={state.fileType}
              onChange={(e) => set('fileType', e.target.value as FileType)}
            >
              <option value="auto">{t('import_file_type_auto')}</option>
              <option value="binary">{t('import_file_type_binary')}</option>
              <option value="ascii">{t('import_file_type_ascii')}</option>
            </select>
          </div>

          <div className={styles.field}>
            <label className={styles.label}>{t('import_user')}</label>
            <input
              className={styles.input}
              type="number"
              min={0}
              max={15}
              value={state.user}
              onChange={(e) => set('user', e.target.value)}
            />
          </div>
        </div>

        {/* Load / Exec address — only for binary/auto */}
        {showBinaryFields && (
          <div className={styles.row2}>
            <div className={styles.field}>
              <label className={styles.label}>{t('import_load_address')}</label>
              <input
                className={styles.input}
                type="text"
                placeholder="0x4000"
                value={state.loadAddress}
                onChange={(e) => set('loadAddress', e.target.value)}
              />
            </div>
            <div className={styles.field}>
              <label className={styles.label}>{t('import_exec_address')}</label>
              <input
                className={styles.input}
                type="text"
                placeholder="0x4000"
                value={state.execAddress}
                onChange={(e) => set('execAddress', e.target.value)}
              />
            </div>
          </div>
        )}

        {/* Checkboxes */}
        <div className={styles.checkRow}>
          <label className={styles.check}>
            <input type="checkbox" checked={state.readOnly} onChange={(e) => set('readOnly', e.target.checked)} />
            {t('import_read_only')}
          </label>
          <label className={styles.check}>
            <input type="checkbox" checked={state.system} onChange={(e) => set('system', e.target.checked)} />
            {t('import_system')}
          </label>
          <label className={styles.check}>
            <input type="checkbox" checked={state.force} onChange={(e) => set('force', e.target.checked)} />
            {t('import_force')}
          </label>
        </div>
      </div>
    </Modal>
  );
}
