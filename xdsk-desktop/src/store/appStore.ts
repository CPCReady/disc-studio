// MIT License
// Copyright (c) Destroyer 2026.
import { create } from 'zustand';
import { persist } from 'zustand/middleware';
import type { ConsoleEntry, BottomTabId, OpenDisk, CompareResult, DiskHealth } from '../types/app';
import type { DiffEntry } from '../types/xdsk';
import { uid, basename } from '../utils/formatters';

interface ViewTarget {
  diskId: string;
  diskPath: string;
  fileName: string;
}

interface AppStore {
  // Disks open in sidebar
  openDisks: OpenDisk[];
  activeDiskId: string | null;

  // Compare results (tabs in main area)
  compareResults: CompareResult[];
  activeCompareId: string | null;

  // Bottom panel
  activeBottomTab: BottomTabId;
  isBottomPanelOpen: boolean;

  // View target (file to display in view panel)
  viewTarget: ViewTarget | null;

  // Currently selected file in the explorer (1 selection)
  selectedFileName: string | null;
  selectedFilesByDisk: Record<string, string[]>;

  // disc CLI status
  discVersion: string | null;
  isDiscAvailable: boolean;

  // Console
  consoleEntries: ConsoleEntry[];

  // Disk health by open disk id
  diskHealth: Record<string, DiskHealth>;

  // Actions — disks
  addOpenDisk: (path: string) => string;
  removeOpenDisk: (id: string) => void;
  setActiveDisk: (id: string | null) => void;
  closeTab: (id: string) => void;

  // Actions — compare
  addCompareResult: (disk1: string, disk2: string, entries: DiffEntry[]) => void;
  removeCompareResult: (id: string) => void;
  setActiveCompareId: (id: string | null) => void;

  // Actions — panel
  setActiveBottomTab: (tab: BottomTabId) => void;
  toggleBottomPanel: () => void;
  setBottomPanelOpen: (o: boolean) => void;

  // Actions — view target
  setViewTarget: (target: ViewTarget | null) => void;

  // Actions — selection
  setSelectedFileName: (name: string | null) => void;
  setSelectedFilesForDisk: (diskId: string, names: string[]) => void;
  clearSelectedFilesForDisk: (diskId: string) => void;

  // Actions — cli status
  setDiscVersion: (v: string | null) => void;
  setDiscAvailable: (a: boolean) => void;

  // Check trigger
  checkTrigger: number;
  triggerCheck: () => void;

  // Topbar action triggers
  topbarImportTrigger: number;
  topbarExportTrigger: number;
  topbarRemoveTrigger: number;
  topbarExportCprTrigger: number;
  triggerTopbarImport: () => void;
  triggerTopbarExport: () => void;
  triggerTopbarRemove: () => void;
  triggerTopbarExportCpr: () => void;

  // View output clear trigger
  viewClearTrigger: number;
  triggerViewClear: () => void;

  // Actions — console
  addConsoleEntry: (type: ConsoleEntry['type'], content: string) => void;
  clearConsole: () => void;

  // Actions — health
  setDiskHealth: (diskId: string, health: DiskHealth) => void;
  clearDiskHealth: (diskId: string) => void;
}

export const useAppStore = create<AppStore>()(
  persist(
    (set, get) => ({
      openDisks: [],
      activeDiskId: null,
      compareResults: [],
      activeCompareId: null,
      activeBottomTab: 'view',
      isBottomPanelOpen: false,
      viewTarget: null,
      selectedFileName: null,
      selectedFilesByDisk: {},
      discVersion: null,
      isDiscAvailable: false,
      consoleEntries: [],
      diskHealth: {},
      checkTrigger: 0,
      topbarImportTrigger: 0,
      topbarExportTrigger: 0,
      topbarRemoveTrigger: 0,
      topbarExportCprTrigger: 0,
      viewClearTrigger: 0,

      addOpenDisk: (path) => {
        const existing = get().openDisks.find((d) => d.path === path);
        if (existing) {
          set((s) => ({
            activeDiskId: existing.id,
            openDisks: s.openDisks.map((d) => d.id === existing.id ? { ...d, tabOpen: true } : d),
          }));
          return existing.id;
        }
        const id = uid();
        const label = basename(path);
        set((s) => ({
          openDisks: [...s.openDisks, { id, path, label, tabOpen: true }],
          activeDiskId: id,
        }));
        return id;
      },

      removeOpenDisk: (id) =>
        set((s) => {
          const disks = s.openDisks.filter((d) => d.id !== id);
          const openTabs = disks.filter((d) => d.tabOpen);
          const activeDiskId =
            s.activeDiskId === id ? (openTabs[openTabs.length - 1]?.id ?? disks[disks.length - 1]?.id ?? null) : s.activeDiskId;
          const nextHealth = { ...s.diskHealth };
          delete nextHealth[id];
          const nextSelection = { ...s.selectedFilesByDisk };
          delete nextSelection[id];
          return { openDisks: disks, activeDiskId, diskHealth: nextHealth, selectedFilesByDisk: nextSelection };
        }),

      setActiveDisk: (id) => set((s) => ({
        activeDiskId: id,
        activeCompareId: null,
        openDisks: id ? s.openDisks.map((d) => d.id === id ? { ...d, tabOpen: true } : d) : s.openDisks,
      })),

      closeTab: (id) =>
        set((s) => {
          const openTabs = s.openDisks.filter((d) => d.tabOpen && d.id !== id);
          const activeDiskId = s.activeDiskId === id
            ? (openTabs[openTabs.length - 1]?.id ?? null)
            : s.activeDiskId;
          return {
            openDisks: s.openDisks.map((d) => d.id === id ? { ...d, tabOpen: false } : d),
            activeDiskId,
          };
        }),

      addCompareResult: (disk1, disk2, entries) => {
        const id = uid();
        const label = `${basename(disk1)} vs ${basename(disk2)}`;
        set((s) => ({
          compareResults: [...s.compareResults, { id, disk1, disk2, label, entries }],
          activeCompareId: id,
          activeDiskId: null,
        }));
      },

      removeCompareResult: (id) =>
        set((s) => {
          const results = s.compareResults.filter((r) => r.id !== id);
          const activeCompareId = s.activeCompareId === id
            ? (results[results.length - 1]?.id ?? null)
            : s.activeCompareId;
          return { compareResults: results, activeCompareId };
        }),

      setActiveCompareId: (id) => set({ activeCompareId: id, activeDiskId: null }),

      setActiveBottomTab: (tab) => set({ activeBottomTab: tab, isBottomPanelOpen: true }),
      toggleBottomPanel: () => set((s) => ({ isBottomPanelOpen: !s.isBottomPanelOpen })),
      setBottomPanelOpen: (o) => set({ isBottomPanelOpen: o }),

      setViewTarget: (target) => set({ viewTarget: target }),
      setSelectedFileName: (name) => set({ selectedFileName: name }),
      setSelectedFilesForDisk: (diskId, names) =>
        set((s) => ({
          selectedFilesByDisk: {
            ...s.selectedFilesByDisk,
            [diskId]: names,
          },
        })),
      clearSelectedFilesForDisk: (diskId) =>
        set((s) => {
          const next = { ...s.selectedFilesByDisk };
          delete next[diskId];
          return { selectedFilesByDisk: next };
        }),

      setDiscVersion: (v) => set({ discVersion: v }),
      setDiscAvailable: (a) => set({ isDiscAvailable: a }),

      triggerCheck: () => set((s) => ({ checkTrigger: s.checkTrigger + 1 })),
      triggerTopbarImport: () => set((s) => ({ topbarImportTrigger: s.topbarImportTrigger + 1 })),
      triggerTopbarExport: () => set((s) => ({ topbarExportTrigger: s.topbarExportTrigger + 1 })),
      triggerTopbarRemove: () => set((s) => ({ topbarRemoveTrigger: s.topbarRemoveTrigger + 1 })),
      triggerTopbarExportCpr: () => set((s) => ({ topbarExportCprTrigger: s.topbarExportCprTrigger + 1 })),

      triggerViewClear: () => set((s) => ({ viewClearTrigger: s.viewClearTrigger + 1 })),

      addConsoleEntry: (type, content) =>
        set((s) => ({
          consoleEntries: [
            ...s.consoleEntries.slice(-199),
            { id: uid(), type, content, timestamp: new Date() },
          ],
        })),

      clearConsole: () => set({ consoleEntries: [] }),

      setDiskHealth: (diskId, health) =>
        set((s) => ({ diskHealth: { ...s.diskHealth, [diskId]: health } })),

      clearDiskHealth: (diskId) =>
        set((s) => {
          const next = { ...s.diskHealth };
          delete next[diskId];
          return { diskHealth: next };
        }),
    }),
    {
      name: 'xdsk-app-v2',
      partialize: (s) => ({
        openDisks: s.openDisks,
        activeDiskId: s.activeDiskId,
        activeBottomTab: s.activeBottomTab,
        isBottomPanelOpen: s.isBottomPanelOpen,
        diskHealth: s.diskHealth,
      }),
    }
  )
);
