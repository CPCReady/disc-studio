// MIT License
// Copyright (c) Destroyer 2026.
import { useState, useEffect, useCallback, useRef } from 'react';
import { RefreshCw, Lock, Monitor } from 'lucide-react';
import { useAppStore } from '../store/appStore';
import { useSettingsStore } from '../store/settingsStore';
import { useDiscCommand } from '../hooks/useDiscCommand';
import { parseListJson } from '../utils/parsers';
import { formatBytes, fileTypeLabel } from '../utils/formatters';
import { launchEmulator } from '../api/xdsk';
import { useI18n } from '../i18n/useI18n';
import { Toolbar, ToolbarSeparator, ToolbarSpacer } from './Toolbar';
import { IconButton } from './IconButton';
import { DataTable, type Column } from './DataTable';
import { EmptyState } from './EmptyState';
import { StatusIndicator } from './StatusIndicator';
import { PositionedContextMenu } from './PositionedContextMenu';
import { ExportModal } from './ExportModal';
import { ExportCprModal } from './ExportCprModal';
import { ImportModal } from './ImportModal';
import { RemoveModal } from './RemoveModal';
import type { DiskFile } from '../types/xdsk';
import styles from './DskExplorer.module.css';

interface Props {
  diskId: string;
  diskPath: string;
}

interface CtxMenu {
  x: number;
  y: number;
  file: DiskFile;
}

export function DskExplorer({ diskId, diskPath }: Props) {
  const {
    activeDiskId,
    topbarImportTrigger,
    topbarExportTrigger,
    topbarRemoveTrigger,
    topbarExportCprTrigger,
    setViewTarget,
    setActiveBottomTab,
    setSelectedFileName,
    setSelectedFilesForDisk,
    triggerCheck,
  } = useAppStore();
  const { emulatorPath, xcartRomsPath } = useSettingsStore();
  const { t } = useI18n();
  const [files, setFiles] = useState<DiskFile[]>([]);
  const [totalSize, setTotalSize] = useState(0);
  const [freeSpace, setFreeSpace] = useState(0);
  const [selectedKeys, setSelectedKeys] = useState<string[]>([]);
  const [ctxMenu, setCtxMenu] = useState<CtxMenu | null>(null);
  const [exportTarget, setExportTarget] = useState<DiskFile | null>(null);
  const [exportOpen, setExportOpen] = useState(false);
  const [exportCprOpen, setExportCprOpen] = useState(false);
  const [importOpen, setImportOpen] = useState(false);
  const [removeTarget, setRemoveTarget] = useState<DiskFile[]>([]);
  const { execute, loading } = useDiscCommand();
  const explorerRef = useRef<HTMLDivElement>(null);
  const lastImportTriggerRef = useRef(0);
  const lastExportTriggerRef = useRef(0);
  const lastRemoveTriggerRef = useRef(0);
  const lastExportCprTriggerRef = useRef(0);

  const handleSelectionChange = (keys: string[]) => {
    setSelectedKeys(keys);
    setSelectedFilesForDisk(diskId, keys);
    setSelectedFileName(keys.length === 1 ? keys[0] : null);
  };

  const refresh = useCallback(async () => {
    const result = await execute(['list', diskPath, '--format', 'json']);
    if (result?.success) {
      const parsed = parseListJson(result.stdout);
      setFiles(parsed.files);
      setTotalSize(parsed.total_size);
      setFreeSpace(parsed.free_space);
      setSelectedKeys([]);
      setSelectedFilesForDisk(diskId, []);
    }
  }, [diskId, diskPath, execute, setSelectedFilesForDisk]);

  useEffect(() => { refresh(); }, [refresh]);

  // Close ctx menu on outside click
  useEffect(() => {
    const handler = () => setCtxMenu(null);
    if (ctxMenu) window.addEventListener('click', handler);
    return () => window.removeEventListener('click', handler);
  }, [ctxMenu]);

  const handleRowRightClick = (file: DiskFile, e: React.MouseEvent) => {
    e.preventDefault();
    e.stopPropagation();
    setCtxMenu({ x: e.clientX, y: e.clientY, file });
  };

  const selectedFiles = files.filter((f) => selectedKeys.includes(f.name));

  const handleOperationDone = () => {
    refresh();
    triggerCheck();
  };

  useEffect(() => {
    if (topbarImportTrigger === lastImportTriggerRef.current) return;
    lastImportTriggerRef.current = topbarImportTrigger;
    if (activeDiskId !== diskId) return;
    setImportOpen(true);
  }, [activeDiskId, diskId, topbarImportTrigger]);

  useEffect(() => {
    if (topbarExportTrigger === lastExportTriggerRef.current) return;
    lastExportTriggerRef.current = topbarExportTrigger;
    if (activeDiskId !== diskId || selectedFiles.length === 0) return;
    setExportTarget(selectedFiles.length === 1 ? selectedFiles[0] : null);
    setExportOpen(true);
  }, [activeDiskId, diskId, topbarExportTrigger, selectedFiles]);

  useEffect(() => {
    if (topbarRemoveTrigger === lastRemoveTriggerRef.current) return;
    lastRemoveTriggerRef.current = topbarRemoveTrigger;
    if (activeDiskId !== diskId || selectedFiles.length === 0) return;
    setRemoveTarget(selectedFiles);
  }, [activeDiskId, diskId, topbarRemoveTrigger, selectedFiles]);

  useEffect(() => {
    if (topbarExportCprTrigger === lastExportCprTriggerRef.current) return;
    lastExportCprTriggerRef.current = topbarExportCprTrigger;
    if (activeDiskId !== diskId) return;
    setExportCprOpen(true);
  }, [activeDiskId, diskId, topbarExportCprTrigger]);

  const formatHex = (v: number | undefined | null) =>
    v != null ? `0x${v.toString(16).toUpperCase().padStart(4, '0')}` : '—';

  const columns: Column<DiskFile & Record<string, unknown>>[] = [
    {
      key: 'name',
      header: t('explorer_name'),
      width: '30%',
      render: (v) => (
        <span style={{ fontFamily: 'var(--font-mono)', fontSize: '12px' }}>{String(v)}</span>
      ),
    },
    {
      key: 'type',
      header: t('explorer_type'),
      width: '12%',
      render: (v) => fileTypeLabel(String(v)),
    },
    {
      key: 'size',
      header: t('explorer_size'),
      width: '12%',
      align: 'right',
      render: (v) => formatBytes(Number(v)),
    },
    {
      key: 'load_address',
      header: t('explorer_load_addr'),
      width: '12%',
      align: 'right',
      render: (v) => (
        <span style={{ fontFamily: 'var(--font-mono)', fontSize: '11px', color: 'var(--text-secondary)' }}>
          {formatHex(v as number | undefined)}
        </span>
      ),
    },
    {
      key: 'exec_address',
      header: t('explorer_exec_addr'),
      width: '12%',
      align: 'right',
      render: (v) => (
        <span style={{ fontFamily: 'var(--font-mono)', fontSize: '11px', color: 'var(--text-secondary)' }}>
          {formatHex(v as number | undefined)}
        </span>
      ),
    },
    {
      key: 'read_only',
      header: t('explorer_readonly'),
      width: '9%',
      align: 'center',
      render: (v) => (
        <span style={{ display: 'flex', justifyContent: 'center', alignItems: 'center' }}>
          {v
            ? <Lock size={11} style={{ color: 'var(--accent-amber)' }} />
            : <span style={{ color: 'var(--text-muted)' }}>—</span>}
        </span>
      ),
    },
    {
      key: 'system',
      header: t('explorer_system'),
      width: '9%',
      align: 'center',
      render: (v) => (
        <span style={{ display: 'flex', justifyContent: 'center', alignItems: 'center' }}>
          {v
            ? <Monitor size={11} style={{ color: 'var(--accent-blue)' }} />
            : <span style={{ color: 'var(--text-muted)' }}>—</span>}
        </span>
      ),
    },
    {
      key: 'user',
      header: t('explorer_user'),
      width: '7%',
      align: 'center',
      render: (v) => (
        <span style={{ fontFamily: 'var(--font-mono)', fontSize: '11px', color: 'var(--text-secondary)' }}>
          {Number(v) > 0 ? String(v) : <span style={{ color: 'var(--text-muted)' }}>0</span>}
        </span>
      ),
    },
  ];

  const usedSpace = totalSize;
  const cprAutoCommand = (() => {
    if (selectedKeys.length !== 1) return '';
    const selected = selectedKeys[0] ?? '';
    const clean = selected.replace(/"/g, '').trim();
    if (!clean) return '';
    return `run"${clean}"`;
  })();
  const cprDefaultPath = /\.[^/.]+$/.test(diskPath)
    ? diskPath.replace(/\.[^/.]+$/, '.cpr')
    : `${diskPath}.cpr`;

  return (
    <div className={styles.explorer} ref={explorerRef}>
      {/* Toolbar */}
      <Toolbar>
        <ToolbarSpacer />
        <span className={styles.stats}>
          {files.length} {t('explorer_files')} · {formatBytes(usedSpace)} {t('explorer_used')} · {formatBytes(freeSpace)} {t('explorer_free')}
        </span>
        <ToolbarSeparator />
        <StatusIndicator
          status={loading ? 'loading' : 'idle'}
          label={loading ? t('loading') : t('ready')}
          size="sm"
        />
        <IconButton icon={<RefreshCw size={13} />} title={t('refresh')} onClick={refresh} />
      </Toolbar>

      {/* Table */}
      <div className={styles.tableWrap}>
        {files.length === 0 && !loading ? (
          <EmptyState
            icon={<Monitor size={32} />}
            title={t('explorer_empty')}
            description={t('explorer_empty_hint')}
          />
        ) : (
          <DataTable
            data={files as unknown as (DiskFile & Record<string, unknown>)[]}
            columns={columns}
            keyField="name"
            selectable
            selectedKeys={selectedKeys}
            onSelectionChange={handleSelectionChange}
            onRowClick={(row) => {
              const name = (row as unknown as DiskFile).name;
              setViewTarget({ diskId, diskPath, fileName: name });
              setActiveBottomTab('view');
              setSelectedFileName(name);
            }}
            onRowContextMenu={(row, e) => handleRowRightClick(row as unknown as DiskFile, e)}
          />
        )}
      </div>

      {/* Context menu */}
      {ctxMenu && (
        <PositionedContextMenu
          x={ctxMenu.x}
          y={ctxMenu.y}
          onClose={() => setCtxMenu(null)}
          items={[
            {
              label: t('ctx_view'),
              onClick: () => {
                setViewTarget({ diskId, diskPath, fileName: ctxMenu.file.name });
                setActiveBottomTab('view');
                setCtxMenu(null);
              },
            },
            {
              label: t('ctx_export'),
              onClick: () => {
                setExportTarget(ctxMenu.file);
                setExportOpen(true);
                setCtxMenu(null);
              },
            },
            { type: 'separator' },
            {
              label: `${ctxMenu.file.read_only ? '\u2713 ' : '  '}${t('ctx_attr_readonly')}`,
              onClick: async () => {
                const flag = ctxMenu.file.read_only ? '--no-read-only' : '--read-only';
                await execute(['attr', diskPath, ctxMenu.file.name, flag]);
                refresh();
                setCtxMenu(null);
              },
            },
            {
              label: `${ctxMenu.file.system ? '\u2713 ' : '  '}${t('ctx_attr_system')}`,
              onClick: async () => {
                const flag = ctxMenu.file.system ? '--no-system' : '--system';
                await execute(['attr', diskPath, ctxMenu.file.name, flag]);
                refresh();
                setCtxMenu(null);
              },
            },
            {
              label: `${t('ctx_user')} (${ctxMenu.file.user ?? 0})`,
              submenu: Array.from({ length: 16 }, (_, n) => ({
                label: `${(ctxMenu.file.user ?? 0) === n ? '\u2713 ' : '  '}${n}`,
                onClick: async () => {
                  await execute(['chuser', diskPath, ctxMenu.file.name, String(n)]);
                  refresh();
                  setCtxMenu(null);
                },
              })),
            },
            { type: 'separator' },
            {
              label: t('ctx_remove'),
              danger: true,
              onClick: () => {
                setRemoveTarget([ctxMenu.file]);
                setCtxMenu(null);
              },
            },
            { type: 'separator' },
            {
              label: t('ctx_execute'),
              disabled: !emulatorPath,
              onClick: async () => {
                if (!emulatorPath) return;
                await launchEmulator({ emulatorPath, diskPath, runFile: ctxMenu.file.name });
                setCtxMenu(null);
              },
            },
          ]}
        />
      )}

      {/* Modals */}
      <ExportModal
        open={exportOpen}
        diskPath={diskPath}
        files={exportTarget ? [exportTarget.name] : selectedKeys}
        onClose={() => { setExportOpen(false); setExportTarget(null); }}
        onDone={() => { setExportOpen(false); setExportTarget(null); handleOperationDone(); }}
      />

      <ExportCprModal
        open={exportCprOpen}
        diskPath={diskPath}
        defaultOutputPath={cprDefaultPath}
        initialAutoCommand={cprAutoCommand}
        romsPath={xcartRomsPath}
        onClose={() => setExportCprOpen(false)}
        onDone={() => { setExportCprOpen(false); triggerCheck(); }}
      />

      <ImportModal
        open={importOpen}
        diskPath={diskPath}
        onClose={() => setImportOpen(false)}
        onDone={() => { setImportOpen(false); handleOperationDone(); }}
      />

      <RemoveModal
        open={removeTarget.length > 0}
        diskPath={diskPath}
        files={removeTarget.map((f) => f.name)}
        onClose={() => setRemoveTarget([])}
        onDone={() => { setRemoveTarget([]); handleOperationDone(); }}
      />
    </div>
  );
}
