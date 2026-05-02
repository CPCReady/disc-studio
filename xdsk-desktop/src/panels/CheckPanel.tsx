// MIT License
// Copyright (c) Destroyer 2026.
import { useState, useCallback, useEffect } from 'react';
import { ShieldCheck } from 'lucide-react';
import { useAppStore } from '../store/appStore';
import { useDiscCommand } from '../hooks/useDiscCommand';
import { parseCheck } from '../utils/parsers';
import type { CheckResult } from '../types/xdsk';
import { Card } from '../components/Card';
import { EmptyState } from '../components/EmptyState';
import { useI18n } from '../i18n/useI18n';
import styles from './Panel.module.css';
import checkStyles from './CheckPanel.module.css';

const SECTIONS: Array<{ key: keyof Pick<CheckResult, 'header' | 'directory' | 'bitmap'>; label: string }> = [
  { key: 'header',    label: 'Header' },
  { key: 'directory', label: 'Directory' },
  { key: 'bitmap',    label: 'Bitmap' },
];

export function CheckPanel() {
  const { activeDiskId, openDisks, checkTrigger } = useAppStore();
  const { t } = useI18n();
  const { execute } = useDiscCommand();
  const [result, setResult] = useState<CheckResult | null>(null);

  const activeDisk = openDisks.find((d) => d.id === activeDiskId);

  const handleCheck = useCallback(async () => {
    if (!activeDisk) return;
    const r = await execute(['check', activeDisk.path]);
    if (r) setResult(parseCheck(r.stdout + r.stderr));
  }, [activeDisk, execute]);

  // Auto-run when the active disk changes (tab switch or new tab)
  useEffect(() => {
    setResult(null);
    handleCheck();
  // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [activeDiskId]);

  // Also run when triggered manually from the toolbar button
  useEffect(() => {
    if (checkTrigger > 0) handleCheck();
  // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [checkTrigger]);

  if (!activeDisk) {
    return (
      <EmptyState icon={<ShieldCheck size={28} />} title={t('check_no_disk')} />
    );
  }

  return (
    <div className={styles.panel}>
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
