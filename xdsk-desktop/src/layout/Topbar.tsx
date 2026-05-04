// MIT License
// Copyright (c) Destroyer 2026.
import { useEffect, useMemo, useState } from 'react';
import { Cpu, Download, Gamepad2, Trash2, Upload } from 'lucide-react';
import { checkXcartAvailable, checkXcartRomsReady, launchEmulator } from '../api/xdsk';
import { useAppStore } from '../store/appStore';
import { useSettingsStore } from '../store/settingsStore';
import { IconButton } from '../components/IconButton';
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
  const canRunEmulator = hasActiveDisk && Boolean(emulatorPath);
  const canExportCpr = hasActiveDisk && xcartAvailable && xcartRomsReady;

  return (
    <header className={styles.topbar}>
      <div className={styles.actions}>
        <IconButton
          icon={<Upload size={13} />}
          variant="default"
          title={t('explorer_import')}
          onClick={triggerTopbarImport}
          disabled={!hasActiveDisk}
        />
        <IconButton
          icon={<Download size={13} />}
          variant="default"
          title={t('explorer_export')}
          onClick={triggerTopbarExport}
          disabled={!hasSelection}
        />
        <IconButton
          icon={<Trash2 size={13} />}
          variant="danger"
          title={t('explorer_remove')}
          onClick={triggerTopbarRemove}
          disabled={!hasSelection}
        />
        <IconButton
          icon={<Cpu size={13} />}
          variant="default"
          onClick={async () => {
            if (!activeDisk || !emulatorPath) return;
            await launchEmulator({ emulatorPath, diskPath: activeDisk.path, runFile: emulatorRunFile });
          }}
          disabled={!canRunEmulator}
          title={!emulatorPath ? 'Configure emulator path in Settings' : t('explorer_emulator')}
        />
        <IconButton
          icon={<Gamepad2 size={13} />}
          variant="default"
          onClick={triggerTopbarExportCpr}
          disabled={!canExportCpr}
          title={
            !xcartAvailable
              ? t('explorer_export_cpr_unavailable')
              : !xcartRomsReady
                ? t('explorer_export_cpr_roms_missing')
                : t('explorer_export_cpr')
          }
        />
      </div>
    </header>
  );
}
