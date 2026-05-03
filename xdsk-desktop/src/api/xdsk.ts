// MIT License
// Copyright (c) Destroyer 2026.
import { invoke } from '@tauri-apps/api/core';
import type {
  CommandOutput,
  ImportOptions,
  ExportOptions,
  RemoveOptions,
  CreateOptions,
  ViewFormat,
  OutputFormat,
} from '../types/xdsk';

/** Low-level: call the Tauri run_xdsk command. Always passes --no-color. */
export async function runXdsk(args: string[]): Promise<CommandOutput> {
  return invoke<CommandOutput>('run_xdsk', { args: ['--no-color', ...args] });
}

export async function runXcart(args: string[]): Promise<CommandOutput> {
  return invoke<CommandOutput>('run_xcart', { args });
}

export async function checkXdskAvailable(): Promise<boolean> {
  return invoke<boolean>('xdsk_available');
}

export async function getXdskVersion(): Promise<string> {
  return invoke<string>('xdsk_version');
}

export async function checkXcartAvailable(): Promise<boolean> {
  return invoke<boolean>('xcart_available');
}

export async function getXcartVersion(): Promise<string> {
  return invoke<string>('xcart_version');
}

export async function checkXcartRomsReady(path: string): Promise<boolean> {
  return invoke<boolean>('xcart_roms_ready', { path });
}

export async function writeTextFile(path: string, content: string): Promise<void> {
  return invoke<void>('write_text_file', { path, content });
}

export interface EmulatorLaunchOptions {
  emulatorPath: string;
  diskPath: string;
  machine?: string;
  runFile?: string;
}

export interface EmulatorOutput {
  launched: boolean;
  error?: string;
}

export async function launchEmulator(opts: EmulatorLaunchOptions): Promise<EmulatorOutput> {
  return invoke<EmulatorOutput>('launch_emulator', {
    emulatorPath: opts.emulatorPath,
    diskPath: opts.diskPath,
    machine: opts.machine ?? null,
    runFile: opts.runFile ?? null,
  });
}

/* ── Commands ─────────────────────────────────────────── */

export async function listFiles(
  imagePath: string,
  format: OutputFormat = 'json'
): Promise<CommandOutput> {
  return runXdsk(['list', imagePath, '--format', format]);
}

export async function importFiles(
  imagePath: string,
  files: string[],
  opts: ImportOptions = {}
): Promise<CommandOutput> {
  const args = ['import', imagePath, ...files];
  if (opts.fileType)              args.push('--file-type', opts.fileType);
  if (opts.loadAddress)           args.push('--load', opts.loadAddress);
  if (opts.execAddress)           args.push('--exec', opts.execAddress);
  if (opts.user !== undefined)    args.push('--user', String(opts.user));
  if (opts.readOnly)              args.push('--read-only');
  if (opts.system)                args.push('--system');
  if (opts.force)                 args.push('--force');
  return runXdsk(args);
}

export async function exportFiles(
  imagePath: string,
  files: string[],
  opts: ExportOptions = {}
): Promise<CommandOutput> {
  const args = ['export', imagePath, ...files];
  if (opts.outputDir)   args.push('--output', opts.outputDir);
  if (opts.stripHeader) args.push('--strip-header');
  return runXdsk(args);
}

export async function removeFiles(
  imagePath: string,
  files: string[],
  opts: RemoveOptions
): Promise<CommandOutput> {
  const args = ['remove', imagePath, ...files];
  if (opts.force) args.push('--force');
  return runXdsk(args);
}

export async function createDisk(
  imagePath: string,
  opts: CreateOptions
): Promise<CommandOutput> {
  const args = [
    'create', imagePath,
    '--tracks', String(opts.tracks),
    '--sectors', String(opts.sectors),
  ];
  if (opts.force) args.push('--force');
  return runXdsk(args);
}

export async function viewFile(
  imagePath: string,
  fileName: string,
  format: ViewFormat = 'auto'
): Promise<CommandOutput> {
  return runXdsk(['view', imagePath, fileName, '--format', format]);
}

export async function checkDisk(imagePath: string): Promise<CommandOutput> {
  return runXdsk(['check', imagePath]);
}

export async function diskInfo(imagePath: string): Promise<CommandOutput> {
  return runXdsk(['info', imagePath]);
}

export async function diffDisks(
  image1: string,
  image2: string
): Promise<CommandOutput> {
  return runXdsk(['diff', image1, image2]);
}
