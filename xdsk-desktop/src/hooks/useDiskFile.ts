// MIT License
// Copyright (c) Destroyer 2026.
import { open, save } from '@tauri-apps/plugin-dialog';
import { useAppStore } from '../store/appStore';

const DSK_FILTER = [{ name: 'DSK Images', extensions: ['dsk', 'DSK'] }];
const CPR_FILTER = [{ name: 'CPR Cartridge', extensions: ['cpr', 'CPR'] }];
const ALL_FILTER = [{ name: 'All Files', extensions: ['*'] }];

export function useDiskFile() {
  const { addOpenDisk } = useAppStore();

  /** Open a DSK file and add it to the sidebar list */
  const openDisk = async (): Promise<string | null> => {
    const result = await open({ multiple: false, filters: DSK_FILTER });
    const path = typeof result === 'string' ? result : null;
    if (path) addOpenDisk(path);
    return path;
  };

  /** Pick a DSK file WITHOUT adding it to the sidebar */
  const pickDisk = async (): Promise<string | null> => {
    const result = await open({ multiple: false, filters: DSK_FILTER });
    return typeof result === 'string' ? result : null;
  };

  /** Pick a save path for a new or exported DSK file */
  const pickSaveDsk = async (defaultName = 'new.dsk'): Promise<string | null> => {
    return await save({ filters: DSK_FILTER, defaultPath: defaultName });
  };

  /** Pick a save path for a CPR cartridge file */
  const pickSaveCpr = async (defaultName = 'export.cpr'): Promise<string | null> => {
    return await save({ filters: CPR_FILTER, defaultPath: defaultName });
  };

  /** Pick one or more source files to import */
  const pickFiles = async (): Promise<string[]> => {
    const result = await open({ multiple: true, filters: ALL_FILTER });
    if (!result) return [];
    return Array.isArray(result) ? result : [result];
  };

  /** Pick an output directory */
  const pickDirectory = async (): Promise<string | null> => {
    const result = await open({ multiple: false, directory: true });
    return typeof result === 'string' ? result : null;
  };

  return {
    openDisk,
    pickDisk,
    pickSaveDsk,
    pickSaveCpr,
    pickFiles,
    pickDirectory,
  };
}
