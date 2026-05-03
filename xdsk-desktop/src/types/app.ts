// MIT License
// Copyright (c) Destroyer 2026.
export type Theme = 'dark' | 'light';

export type BottomTabId = 'view';

export type ConsoleEntryType = 'command' | 'stdout' | 'stderr' | 'success' | 'error' | 'info';

export interface ConsoleEntry {
  id: string;
  type: ConsoleEntryType;
  content: string;
  timestamp: Date;
}

export interface OpenDisk {
  id: string;
  path: string;
  label: string;
  tabOpen: boolean;
}

export type DiskHealthLevel = 'unknown' | 'ok' | 'warning' | 'error';

export interface DiskHealth {
  level: DiskHealthLevel;
  warnings: number;
  errors: number;
  checkedAt: string | null;
}

export interface CompareResult {
  id: string;
  disk1: string;
  disk2: string;
  label: string;
  entries: import('./xdsk').DiffEntry[];
}

export interface NavItem {
  label: string;
  section: 'disk' | 'files' | 'tools';
}
