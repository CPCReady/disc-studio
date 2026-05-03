// MIT License
// Copyright (c) Destroyer 2026.
import { useI18n } from '../i18n/useI18n';
import { Modal } from './Modal';
import { Select } from './Select';
import { open as openDialog } from '@tauri-apps/plugin-dialog';
import { useSettingsStore, type FontSize, type FontFamily } from '../store/settingsStore';
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

const FONT_SIZE_OPTIONS: { value: FontSize; label: string }[] = [
  { value: '11', label: '11 px' },
  { value: '12', label: '12 px' },
  { value: '13', label: '13 px' },
  { value: '14', label: '14 px' },
  { value: '15', label: '15 px' },
];

const FONT_FAMILY_OPTIONS: { value: FontFamily; label: string }[] = [
  { value: 'ibm-plex', label: 'IBM Plex Sans' },
  { value: 'manrope',  label: 'Manrope' },
  { value: 'inter',    label: 'Inter' },
  { value: 'nunito',   label: 'Nunito Sans' },
  { value: 'system',   label: 'System UI' },
];

export function SettingsModal({ isOpen, onClose }: Props) {
  const { t, language, setLanguage } = useI18n();
  const {
    emulatorPath,
    setEmulatorPath,
    xcartRomsPath,
    setXcartRomsPath,
    fontSize,
    setFontSize,
    fontFamily,
    setFontFamily,
  } = useSettingsStore();

  const handleBrowseEmulator = async () => {
    const selected = await openDialog({ multiple: false, directory: false });
    if (selected && typeof selected === 'string') {
      setEmulatorPath(selected);
    }
  };

  const handleBrowseXcartRoms = async () => {
    const selected = await openDialog({ multiple: false, directory: true });
    if (selected && typeof selected === 'string') {
      setXcartRomsPath(selected);
    }
  };

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

        {/* Font family */}
        <div className={styles.row}>
          <div className={styles.rowInfo}>
            <label className={styles.label}>{t('settings_font_family')}</label>
            <span className={styles.desc}>{t('settings_font_family_desc')}</span>
          </div>
          <Select
            value={fontFamily}
            onChange={(v) => setFontFamily(v as FontFamily)}
            options={FONT_FAMILY_OPTIONS}
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
            options={FONT_SIZE_OPTIONS}
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

        {/* xcart ROMs path */}
        <div className={styles.emulatorRow}>
          <div className={styles.rowInfo}>
            <label className={styles.label}>{t('settings_xcart_roms_path')}</label>
            <span className={styles.desc}>{t('settings_xcart_roms_path_desc')}</span>
          </div>
          <div className={styles.pathInput}>
            <input
              className={styles.pathField}
              type="text"
              value={xcartRomsPath}
              onChange={(e) => setXcartRomsPath(e.target.value)}
              placeholder="/path/to/roms"
              spellCheck={false}
            />
            <button className={styles.browseBtn} onClick={handleBrowseXcartRoms}>
              {t('settings_xcart_roms_browse')}
            </button>
          </div>
        </div>
      </div>
    </Modal>
  );
}
