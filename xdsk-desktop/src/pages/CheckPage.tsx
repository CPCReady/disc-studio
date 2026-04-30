// MIT License
// Copyright (c) Destroyer 2026.
// @ts-nocheck
import { useState, useCallback } from 'react';
import { ShieldCheck, FolderOpen, Play } from 'lucide-react';
import { useAppStore } from '../store/appStore';
import { useDiscCommand } from '../hooks/useDiscCommand';
import { useDiskFile } from '../hooks/useDiskFile';
import { parseCheck } from '../utils/parsers';
import type { CheckResult } from '../types/disc';
import { SectionHeader } from '../components/SectionHeader';
import { Card } from '../components/Card';
import { Button } from '../components/Button';
import { EmptyState } from '../components/EmptyState';
import styles from './FormPage.module.css';
import checkStyles from './CheckPage.module.css';

const SECTIONS: Array<{ key: keyof Pick<CheckResult, 'header' | 'directory' | 'bitmap'>; label: string }> = [
  { key: 'header',    label: 'Header' },
  { key: 'directory', label: 'Directory' },
  { key: 'bitmap',    label: 'Bitmap' },
];

export function CheckPage() {
  const { currentDiskPath } = useAppStore();
  const { openDisk } = useDiskFile();
  const { execute, loading } = useDiscCommand();
  const [result, setResult] = useState<CheckResult | null>(null);

  const handleCheck = useCallback(async () => {
    if (!currentDiskPath) return;
    const r = await execute(['check', currentDiskPath]);
    if (r) setResult(parseCheck(r.stdout + r.stderr));
  }, [currentDiskPath, execute]);

  if (!currentDiskPath) {
    return (
      <EmptyState
        icon={<ShieldCheck size={36} />}
        title="No disk image open"
        action={<Button variant="primary" icon={<FolderOpen size={13} />} onClick={openDisk}>Open DSK Image</Button>}
      />
    );
  }

  return (
    <div className={styles.page}>
      <SectionHeader title="Check Disk" description="Verify the integrity of the current DSK image." />

      <div className={styles.footer} style={{ marginBottom: 'var(--space-6)' }}>
        <Button variant="primary" icon={<Play size={13} />} loading={loading} onClick={handleCheck}>
          Run Check
        </Button>
      </div>

      {result && SECTIONS.map(({ key, label }) => {
        const issues = result[key];
        if (issues.length === 0) return null;
        return (
          <Card key={key} title={label} className={checkStyles.sectionCard}>
            {issues.map((issue, i) => (
              <div key={i} className={`${checkStyles.issue} ${issue.type === 'ok' ? checkStyles.ok : checkStyles.fail}`}>
                <span className={checkStyles.icon}>{issue.type === 'ok' ? '✓' : '✗'}</span>
                <span className={checkStyles.text}>{issue.message}</span>
              </div>
            ))}
          </Card>
        );
      })}
    </div>
  );
}
