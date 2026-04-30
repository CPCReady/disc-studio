// MIT License
// Copyright (c) Destroyer 2026.
import type {
  DiskFile,
  DiskListResult,
  DiskInfo,
  CheckResult,
  CheckIssue,
  DiffEntry,
} from '../types/xdsk';

export function parseListJson(raw: string): DiskListResult {
  try {
    return JSON.parse(raw) as DiskListResult;
  } catch {
    return { files: [], total_size: 0, free_space: 0 };
  }
}

export function parseInfo(text: string): Partial<DiskInfo> {
  const info: Partial<DiskInfo> = { files: [] };

  for (const line of text.split('\n')) {
    const t = line.trim();
    if (!t) continue;

    const kv = (prefix: RegExp) => {
      const m = t.match(prefix);
      return m ? t.slice(m[0].length).trim() : null;
    };

    const v = kv(/^Format\s+/);          if (v !== null) { info.format = v; continue; }
    const tr = kv(/^Tracks\s+/);         if (tr !== null) { info.tracks = parseInt(tr); continue; }
    const sec = kv(/^Sectors\/Track\s+/);if (sec !== null) { info.sectors = parseInt(sec); continue; }
    const ss = kv(/^Sector size\s+/);    if (ss !== null) { info.sectorSize = parseInt(ss); continue; }
    const cap = kv(/^Capacity\s+/);      if (cap !== null) { info.capacity = cap; continue; }

    if (/^Entries used\s+/.test(t)) {
      const m = t.match(/(\d+)\s*\/\s*(\d+)/);
      if (m) { info.entriesUsed = +m[1]; info.entriesTotal = +m[2]; }
      continue;
    }
    if (/^Total blocks\s+/.test(t)) {
      info.totalBlocks = parseInt(t.replace(/^Total blocks\s+/, ''));
      continue;
    }
    if (/^Used\s+\d+/.test(t)) {
      const m = t.match(/^Used\s+(\d+)/);
      if (m) info.usedBlocks = +m[1];
      continue;
    }
    if (/^Free\s+\d+/.test(t)) {
      const m = t.match(/^Free\s+(\d+)/);
      if (m) info.freeBlocks = +m[1];
      continue;
    }
  }

  // Parse file table (after separator line)
  const sepIdx = text.indexOf('──────');
  if (sepIdx !== -1) {
    const rows = text.slice(sepIdx).split('\n').slice(1);
    const files: DiskFile[] = [];
    for (const row of rows) {
      const t2 = row.trim();
      if (!t2 || t2.startsWith('─')) continue;
      const parts = t2.split(/\s+/);
      if (parts.length >= 2) {
        files.push({
          name: parts[0],
          type: parts[1] ?? '',
          size: parseInt(parts[2]) || 0,
          user: 0,
          read_only: (parts[3] ?? '').includes('R'),
          system: (parts[3] ?? '').includes('S'),
        });
      }
    }
    info.files = files;
  }

  return info;
}

export function parseCheck(text: string): CheckResult {
  const result: CheckResult = {
    header: [],
    directory: [],
    bitmap: [],
    success: true,
  };

  type Section = 'header' | 'directory' | 'bitmap';
  let section: Section = 'header';

  for (const line of text.split('\n')) {
    const t = line.trim();
    if (!t) continue;

    const tl = t.toLowerCase();
    if (tl.startsWith('header'))    { section = 'header';    continue; }
    if (tl.startsWith('directory')) { section = 'directory'; continue; }
    if (tl.startsWith('bitmap'))    { section = 'bitmap';    continue; }

    let issue: CheckIssue | null = null;
    if (t.startsWith('✓') || t.startsWith('√')) {
      issue = { type: 'ok', message: t.slice(2).trim() };
    } else if (t.startsWith('✗') || t.startsWith('×')) {
      issue = { type: 'error', message: t.slice(2).trim() };
      result.success = false;
    } else if (t.startsWith('!')) {
      issue = { type: 'warning', message: t.slice(1).trim() };
    }

    if (issue) result[section].push(issue);
  }

  return result;
}

export function parseDiff(text: string): DiffEntry[] {
  const entries: DiffEntry[] = [];
  let onlyAFilled = false;
  type Status = DiffEntry['status'];
  let current: Status | null = null;

  for (const line of text.split('\n')) {
    const t = line.trim();
    if (!t || t.startsWith('Comparing')) continue;

    if (t.startsWith('Only in') && !onlyAFilled) {
      current = 'only-a';
      continue;
    }
    if (t.startsWith('Only in') && onlyAFilled) {
      current = 'only-b';
      continue;
    }
    if (t.startsWith('Modified')) { current = 'modified'; continue; }
    if (t.startsWith('Identical')) { current = 'identical'; continue; }
    if (t.startsWith('─')) continue;

    if (current && t) {
      const parts = t.split(/\s+/);
      const name = parts[0];
      if (!name || name.length < 2) continue;

      if (current === 'modified') {
        const sizeA = parseInt(parts[1]) || 0;
        const sizeB = parseInt(parts[3]) || 0;
        entries.push({ name, status: 'modified', sizeA, sizeB });
      } else {
        const size = parseInt(parts[1]) || 0;
        entries.push({ name, status: current, sizeA: size });
        if (current === 'only-a') onlyAFilled = true;
      }
    }
  }

  return entries;
}
