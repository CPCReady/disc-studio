// MIT License
// Copyright (c) Destroyer 2026.
import { useState, useCallback } from 'react';
import { runXdsk } from '../api/xdsk';
import { useAppStore } from '../store/appStore';
import type { CommandOutput } from '../types/xdsk';

interface Options {
  onSuccess?: (output: CommandOutput) => void;
  onError?: (msg: string) => void;
}

export function useDiscCommand(opts?: Options) {
  const [loading, setLoading] = useState(false);
  const [output, setOutput] = useState<CommandOutput | null>(null);
  const [error, setError] = useState<string | null>(null);
  const { addConsoleEntry } = useAppStore();

  const execute = useCallback(
    async (args: string[]): Promise<CommandOutput | null> => {
      setLoading(true);
      setError(null);

      addConsoleEntry('command', `xdsk ${args.join(' ')}`);

      try {
      const result = await runXdsk(args);
        setOutput(result);

        if (result.stdout.trim()) {
          addConsoleEntry(result.success ? 'stdout' : 'stderr', result.stdout.trim());
        }
        if (result.stderr.trim()) {
          addConsoleEntry('stderr', result.stderr.trim());
        }

        if (result.success) {
          opts?.onSuccess?.(result);
        } else {
          const msg = result.stderr.trim() || result.stdout.trim() || 'Command failed';
          setError(msg);
          addConsoleEntry('error', msg);
          opts?.onError?.(msg);
        }

        return result;
      } catch (e) {
        const msg = e instanceof Error ? e.message : String(e);
        setError(msg);
        addConsoleEntry('error', msg);
        opts?.onError?.(msg);
        return null;
      } finally {
        setLoading(false);
      }
    },
    [addConsoleEntry, opts]
  );

  const reset = useCallback(() => {
    setOutput(null);
    setError(null);
  }, []);

  return { execute, loading, output, error, reset };
}
