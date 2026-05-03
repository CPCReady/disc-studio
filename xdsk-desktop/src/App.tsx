// MIT License
// Copyright (c) Destroyer 2026.
import './styles/variables.css';
import './styles/reset.css';
import './styles/typography.css';
import './styles/global.css';
import { AppShell } from './layout/AppShell';
import { useAppStore } from './store/appStore';
import { DskExplorer } from './components/DskExplorer';
import { DiffResultView } from './components/DiffResultView';
import { EmptyState } from './components/EmptyState';
import { Save } from 'lucide-react';

function App() {
  const { activeDiskId, openDisks, activeCompareId } = useAppStore();
  const hasActiveDisk = openDisks.some((d) => d.id === activeDiskId);

  return (
    <AppShell>
      {activeCompareId ? (
        <DiffResultView key={activeCompareId} compareId={activeCompareId} />
      ) : hasActiveDisk ? (
        <>
          {openDisks
            .filter((d) => d.tabOpen)
            .map((disk) => (
              <div
                key={disk.id}
                style={{ display: disk.id === activeDiskId ? 'contents' : 'none' }}
              >
                <DskExplorer diskId={disk.id} diskPath={disk.path} />
              </div>
            ))}
        </>
      ) : (
        <EmptyState
          icon={<Save size={36} />}
          title="No disk image open"
          description='Click "+" in the sidebar to open a DSK image.'
        />
      )}
    </AppShell>
  );
}

export default App;
