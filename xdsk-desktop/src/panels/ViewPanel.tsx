// MIT License
// Copyright (c) Destroyer 2026.
import { useState, useEffect, useCallback } from 'react';
import { Eye } from 'lucide-react';
import { useAppStore } from '../store/appStore';
import { useDiscCommand } from '../hooks/useDiscCommand';
import { Card } from '../components/Card';
import { TextInput } from '../components/TextInput';
import { Select } from '../components/Select';
import { Button } from '../components/Button';
import { EmptyState } from '../components/EmptyState';
import { useI18n } from '../i18n/useI18n';
import styles from './Panel.module.css';
import viewStyles from './ViewPanel.module.css';

const FORMAT_OPTIONS = [
  { value: 'auto',   label: 'Auto-detect' },
  { value: 'basic',  label: 'BASIC listing' },
  { value: 'hex',    label: 'Hex dump' },
  { value: 'ascii',  label: 'ASCII text' },
  { value: 'disasm', label: 'Disassembly' },
];

export function ViewPanel() {
  const { activeDiskId, openDisks, viewTarget, selectedFileName } = useAppStore();
  const { t } = useI18n();
  const { execute, loading, error } = useDiscCommand();
  const [diskPath, setDiskPath] = useState('');
  const [fileName, setFileName] = useState('');
  const [format, setFormat] = useState('auto');
  const [output, setOutput] = useState('');

  const activeDisk = openDisks.find((d) => d.id === activeDiskId);

  // Auto-populate from viewTarget (right-click > View) or selectedFileName (selection in explorer)
  useEffect(() => {
    if (viewTarget) {
      setDiskPath(viewTarget.diskPath);
      setFileName(viewTarget.fileName);
    } else if (activeDisk) {
      setDiskPath(activeDisk.path);
      if (selectedFileName) setFileName(selectedFileName);
    }
  }, [viewTarget, activeDisk, selectedFileName]);

  const runView = useCallback(async () => {
    if (!diskPath || !fileName) return;
    const result = await execute(['view', diskPath, fileName, '--format', format]);
    setOutput(result ? result.stdout || result.stderr : '');
  }, [diskPath, fileName, format, execute]);

  const handleView = async () => {
    await runView();
  };

  useEffect(() => {
    if (!diskPath || !fileName) return;
    void runView();
  }, [diskPath, fileName, format, runView]);

  useEffect(() => {
    if (error) setOutput(error);
  }, [error]);

  if (!activeDisk && !viewTarget) {
    return <EmptyState icon={<Eye size={28} />} title={t('view_no_disk')} />;
  }

  return (
    <div className={styles.panel}>
      <Card>
        <div className={viewStyles.form}>
          <div className={viewStyles.row}>
            <TextInput
              label={t('view_filename')}
              value={fileName}
              onChange={setFileName}
              placeholder="GAME.BAS"
              maxLength={15}
              style={{ width: 140 }}
            />
            <Select label={t('view_format')} value={format} onChange={setFormat} options={FORMAT_OPTIONS} />
            <Button
              variant="primary"
              icon={<Eye size={12} />}
              loading={loading}
              disabled={!fileName || !diskPath}
              onClick={handleView}
            >
              {t('view_run')}
            </Button>
          </div>
        </div>
      </Card>

      {output && (
        <Card title={t('view_output')}>
          <pre className={viewStyles.output}>
            {output.split('\n').filter((l) => l.trim() !== '').join('\n')}
          </pre>
        </Card>
      )}
    </div>
  );
}
