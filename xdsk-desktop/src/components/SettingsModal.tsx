// MIT License
// Copyright (c) Destroyer 2026.
import { useI18n } from '../i18n/useI18n';
import { Modal } from './Modal';
import { Select } from './Select';
import { open as openDialog } from '@tauri-apps/plugin-dialog';
import { useSettingsStore, type FontSize } from '../store/settingsStore';
import type { Language } from '../i18n/index';
import styles from './SettingsModal.module.css';

interface Props {
  isOpen: boolean;
  onClose: () => void;
}

const LANGUAGE_OPTIONS: { value: Language; label: string }[] = [
  { value: 'en', label: 'English' },
  { value: 'es', label: 'Español' },
  { value: 'de', label: 'Deutsch' },
  { value: 'fr', label: 'Français' },
];

const FONT_SIZE_OPTIONS: { value: FontSize; labelKey: 'settings_font_size_sm' | 'settings_font_size_md' | 'settings_font_size_lg' }[] = [
  { value: 'sm', labelKey: 'settings_font_size_sm' },
  { value: 'md', labelKey: 'settings_font_size_md' },
  { value: 'lg', labelKey: 'settings_font_size_lg' },
];

export function SettingsModal({ isOpen, onClose }: Props) {
  const { t, language, setLanguage } = useI18n();
  const { emulatorPath, setEmulatorPath, fontSize, setFontSize } = useSettingsStore();

  const handleBrowseEmulator = async () => {
    const selected = await openDialog({ multiple: false, directory: false });
    if (selected && typeof selected === 'string') {
      setEmulatorPath(selected);
    }
  };

  const fontSizeOptions = FONT_SIZE_OPTIONS.map((o) => ({ value: o.value, label: t(o.labelKey) }));

  return (
    <Modal
      open={isOpen}
      onClose={onClose}
      title={t('settings_title')}
      size="sm"
    >
      <div className={styles.body}>
        {/* Language */}
        <div className={styles.row}>
          <div className={styles.rowInfo}>
            <label className={styles.label}>{t('settings_language')}</label>
            <span className={styles.desc}>{t('settings_language_desc')}</span>
          </div>
          <Select
            value={language}
            onChange={(v) => setLanguage(v as Language)}
            options={LANGUAGE_OPTIONS}
          />
        </div>

        {/* Font size */}
        <div className={styles.row}>
          <div className={styles.rowInfo}>
            <label className={styles.label}>{t('settings_font_size')}</label>
            <span className={styles.desc}>{t('settings_font_size_desc')}</span>
          </div>
          <Select
            value={fontSize}
            onChange={(v) => setFontSize(v as FontSize)}
            options={fontSizeOptions}
          />
        </div>

        {/* Emulator path */}
        <div className={styles.emulatorRow}>
          <div className={styles.rowInfo}>
            <label className={styles.label}>{t('settings_emulator_path')}</label>
            <span className={styles.desc}>{t('settings_emulator_path_desc')}</span>
          </div>
          <div className={styles.pathInput}>
            <input
              className={styles.pathField}
              type="text"
              value={emulatorPath}
              onChange={(e) => setEmulatorPath(e.target.value)}
              placeholder="RetroVirtualMachine"
              spellCheck={false}
            />
            <button className={styles.browseBtn} onClick={handleBrowseEmulator}>
              {t('settings_emulator_browse')}
            </button>
          </div>
        </div>
      </div>
    </Modal>
  );
}
