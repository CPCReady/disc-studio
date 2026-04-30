// MIT License
// Copyright (c) Destroyer 2026.
import { useSettingsStore } from '../store/settingsStore';
import { getT } from './index';

export function useI18n() {
  const { language, setLanguage } = useSettingsStore();
  const t = getT(language);
  return { t, language, setLanguage };
}
