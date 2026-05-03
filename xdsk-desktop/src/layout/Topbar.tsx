// MIT License
// Copyright (c) Destroyer 2026.
import { useEffect, useMemo, useState } from 'react';
import { Cpu, Download, Gamepad2, Trash2, Upload } from 'lucide-react';
import { checkXcartAvailable, checkXcartRomsReady, launchEmulator } from '../api/xdsk';
import { useAppStore } from '../store/appStore';
import { useSettingsStore } from '../store/settingsStore';
import { Button } from '../components/Button';
import { useI18n } from '../i18n/useI18n';
import styles from './Topbar.module.css';

export function Topbar() {
  const {
    openDisks,
    activeDiskId,
    selectedFilesByDisk,
    triggerTopbarImport,
    triggerTopbarExport,
    triggerTopbarRemove,
    triggerTopbarExportCpr,
  } = useAppStore();
  const { emulatorPath, xcartRomsPath } = useSettingsStore();
  const { t } = useI18n();
  const [xcartAvailable, setXcartAvailable] = useState(false);
  const [xcartRomsReady, setXcartRomsReady] = useState(false);

  const activeDisk = openDisks.find((d) => d.id === activeDiskId);
  const selectedFiles = useMemo(
    () => (activeDiskId ? (selectedFilesByDisk[activeDiskId] ?? []) : []),
    [activeDiskId, selectedFilesByDisk]
  );
  const emulatorRunFile = selectedFiles.length === 1 ? selectedFiles[0] : undefined;

  useEffect(() => {
    let alive = true;
    Promise.all([
      checkXcartAvailable().catch(() => false),
      checkXcartRomsReady(xcartRomsPath).catch(() => false),
    ]).then(([available, romsReady]) => {
      if (!alive) return;
      setXcartAvailable(available);
      setXcartRomsReady(romsReady);
    });
    return () => {
      alive = false;
    };
  }, [xcartRomsPath]);

  const hasActiveDisk = Boolean(activeDisk);
  const hasSelection = selectedFiles.length > 0;
  const canRunEmulator = hasActiveDisk && hasSelection && Boolean(emulatorPath);
  const canExportCpr = hasActiveDisk && xcartAvailable && xcartRomsReady;

  return (
    <header className={styles.topbar}>
      <div className={styles.actions}>
        <Button variant="primary" icon={<Upload size={12} />} onClick={triggerTopbarImport} disabled={!hasActiveDisk}>
          {t('explorer_import')}
        </Button>
        <Button variant="primary" icon={<Download size={12} />} onClick={triggerTopbarExport} disabled={!hasSelection}>
          {t('explorer_export')}
        </Button>
        <Button variant="danger" icon={<Trash2 size={12} />} onClick={triggerTopbarRemove} disabled={!hasSelection}>
          {t('explorer_remove')}
        </Button>
        <Button
          variant="primary"
          icon={<Cpu size={12} />}
          onClick={async () => {
            if (!activeDisk || !emulatorPath || !hasSelection) return;
            await launchEmulator({ emulatorPath, diskPath: activeDisk.path, runFile: emulatorRunFile });
          }}
          disabled={!canRunEmulator}
          title={!emulatorPath ? 'Configure emulator path in Settings' : t('explorer_emulator')}
        >
          {t('explorer_emulator')}
        </Button>
        <Button
          variant="primary"
          icon={<Gamepad2 size={12} />}
          onClick={triggerTopbarExportCpr}
          disabled={!canExportCpr}
          title={
            !xcartAvailable
              ? t('explorer_export_cpr_unavailable')
              : !xcartRomsReady
                ? t('explorer_export_cpr_roms_missing')
                : t('explorer_export_cpr')
          }
        >
          {t('explorer_export_cpr')}
        </Button>
      </div>
    </header>
  );
}
