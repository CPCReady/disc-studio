// MIT License
// Copyright (c) Destroyer 2026.
import { create } from 'zustand';
import { persist } from 'zustand/middleware';
import type { Theme } from '../types/app';

interface ThemeStore {
  theme: Theme;
  setTheme: (t: Theme) => void;
  toggleTheme: () => void;
}

function applyTheme(t: Theme) {
  document.documentElement.dataset.theme = t;
}

export const useThemeStore = create<ThemeStore>()(
  persist(
    (set, get) => ({
      theme: 'dark',
      setTheme: (t) => {
        applyTheme(t);
        set({ theme: t });
      },
      toggleTheme: () => {
        const next: Theme = get().theme === 'dark' ? 'light' : 'dark';
        applyTheme(next);
        set({ theme: next });
      },
    }),
    {
      name: 'xdsk-theme',
      onRehydrateStorage: () => (state) => {
        if (state) applyTheme(state.theme);
      },
    }
  )
);
