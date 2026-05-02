// MIT License
// Copyright (c) Destroyer 2026.
import { create } from 'zustand';
import { persist } from 'zustand/middleware';
import type { Language } from '../i18n/index';

export type FontSize = '11' | '12' | '13' | '14' | '15';
export type FontFamily = 'ibm-plex' | 'manrope' | 'inter' | 'nunito' | 'system';

interface SettingsStore {
  language: Language;
  setLanguage: (l: Language) => void;
  emulatorPath: string;
  setEmulatorPath: (p: string) => void;
  fontSize: FontSize;
  setFontSize: (s: FontSize) => void;
  fontFamily: FontFamily;
  setFontFamily: (f: FontFamily) => void;
  // Layout persistence
  rightSidebarWidth: number;
  setRightSidebarWidth: (w: number) => void;
  bottomPanelHeight: number;
  setBottomPanelHeight: (h: number) => void;
  windowWidth: number;
  windowHeight: number;
  windowX: number | null;
  windowY: number | null;
  setWindowBounds: (w: number, h: number, x: number, y: number) => void;
}

export const useSettingsStore = create<SettingsStore>()(
  persist(
    (set) => ({
      language: 'en',
      setLanguage: (language) => set({ language }),
      emulatorPath: '',
      setEmulatorPath: (emulatorPath) => set({ emulatorPath }),
      fontSize: '13',
      setFontSize: (fontSize) => set({ fontSize }),
      fontFamily: 'ibm-plex',
      setFontFamily: (fontFamily) => set({ fontFamily }),
      // Layout persistence defaults
      rightSidebarWidth: 220,
      setRightSidebarWidth: (rightSidebarWidth) => set({ rightSidebarWidth }),
      bottomPanelHeight: 200,
      setBottomPanelHeight: (bottomPanelHeight) => set({ bottomPanelHeight }),
      windowWidth: 1100,
      windowHeight: 700,
      windowX: null,
      windowY: null,
      setWindowBounds: (windowWidth, windowHeight, windowX, windowY) =>
        set({ windowWidth, windowHeight, windowX, windowY }),
    }),
    { name: 'xdsk-settings' }
  )
);
