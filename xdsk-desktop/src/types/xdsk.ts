// MIT License
// Copyright (c) Destroyer 2026.
/* ============================================================
   Types: xdsk CLI
   ============================================================ */

export interface CommandOutput {
  stdout: string;
  stderr: string;
  success: boolean;
  code: number;
}

export type FileType = 'ascii' | 'binary' | 'raw';
export type ViewFormat = 'auto' | 'basic' | 'hex' | 'ascii' | 'disasm';
export type OutputFormat = 'json' | 'table' | 'csv' | 'simple';

export interface DiskFile {
  name: string;
  size: number;
  type: string;
  read_only: boolean;
  system: boolean;
  user: number;
  load_address?: number;
  exec_address?: number;
}

export interface DiskListResult {
  files: DiskFile[];
  total_size: number;
  free_space: number;
}

export interface DiskInfo {
  path: string;
  format: string;
  tracks: number;
  sectors: number;
  sectorSize: number;
  capacity: string;
  entriesUsed: number;
  entriesTotal: number;
  totalBlocks: number;
  usedBlocks: number;
  freeBlocks: number;
  files: DiskFile[];
}

export interface ImportOptions {
  fileType?: FileType;
  loadAddress?: string;
  execAddress?: string;
  user?: number;
  readOnly?: boolean;
  system?: boolean;
  force?: boolean;
}

export interface ExportOptions {
  outputDir?: string;
  stripHeader?: boolean;
}

export interface RemoveOptions {
  force: boolean;
}

export interface CreateOptions {
  tracks: number;
  sectors: number;
  force: boolean;
}

export interface CopyOptions {
  force: boolean;
}

export type DiffStatus = 'only-a' | 'only-b' | 'modified' | 'identical';

export interface DiffEntry {
  name: string;
  sizeA?: number;
  sizeB?: number;
  status: DiffStatus;
}

export type CheckIssueType = 'ok' | 'error' | 'warning';

export interface CheckIssue {
  type: CheckIssueType;
  message: string;
}

export interface CheckResult {
  header: CheckIssue[];
  directory: CheckIssue[];
  bitmap: CheckIssue[];
  success: boolean;
}
