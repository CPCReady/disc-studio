// MIT License
// Copyright (c) Destroyer 2026.
import { create } from 'zustand';
import { persist } from 'zustand/middleware';
import type { Language } from '../i18n/index';

export type FontSize = 'sm' | 'md' | 'lg';

interface SettingsStore {
  language: Language;
  setLanguage: (l: Language) => void;
  emulatorPath: string;
  setEmulatorPath: (p: string) => void;
  fontSize: FontSize;
  setFontSize: (s: FontSize) => void;
}

export const useSettingsStore = create<SettingsStore>()(
  persist(
    (set) => ({
      language: 'en',
      setLanguage: (language) => set({ language }),
      emulatorPath: '',
      setEmulatorPath: (emulatorPath) => set({ emulatorPath }),
      fontSize: 'md',
      setFontSize: (fontSize) => set({ fontSize }),
    }),
    { name: 'xdsk-settings' }
  )
);
