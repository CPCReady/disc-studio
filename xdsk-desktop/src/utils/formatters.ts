// MIT License
// Copyright (c) Destroyer 2026.
export function formatBytes(bytes: number): string {
  if (bytes === 0) return '0 B';
  if (bytes < 1024) return `${bytes} B`;
  if (bytes < 1024 * 1024) return `${(bytes / 1024).toFixed(1)} KB`;
  return `${(bytes / (1024 * 1024)).toFixed(2)} MB`;
}

export function formatPercent(value: number, total: number): string {
  if (total === 0) return '0.0%';
  return `${((value / total) * 100).toFixed(1)}%`;
}

export function formatHex(n: number): string {
  return `0x${n.toString(16).toUpperCase().padStart(4, '0')}`;
}

export function basename(path: string): string {
  return path.replace(/\\/g, '/').split('/').pop() ?? path;
}

export function truncatePath(path: string, maxLen = 48): string {
  if (path.length <= maxLen) return path;
  const normalized = path.replace(/\\/g, '/');
  const parts = normalized.split('/');
  const filename = parts[parts.length - 1];
  const prefix = parts.slice(0, 2).join('/');
  return `${prefix}/…/${filename}`;
}

export function formatTimestamp(date: Date): string {
  return date.toLocaleTimeString('en-US', {
    hour12: false,
    hour: '2-digit',
    minute: '2-digit',
    second: '2-digit',
  });
}

export function fileTypeLabel(type: string): string {
  const map: Record<string, string> = {
    BASIC: 'BASIC',
    BINARY: 'Binary',
    ASCII: 'ASCII',
    RAW: 'Raw',
  };
  return map[type?.toUpperCase()] ?? type ?? '—';
}

/** Tiny classname helper — replaces clsx for simple cases */
export function cx(...classes: (string | undefined | null | false)[]): string {
  return classes.filter(Boolean).join(' ');
}

export function uid(): string {
  return `${Date.now()}-${Math.random().toString(36).slice(2, 7)}`;
}
