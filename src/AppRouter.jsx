import { Suspense, lazy, useEffect, useState } from 'react'
import { useTranslation } from 'react-i18next'
import { useSettingsStore } from './stores/settingsStore'
import { getCurrentWebviewWindow } from '@tauri-apps/api/webviewWindow'
import App from './App'

const ControlPanel = lazy(() => import('./components/ControlPanel'))
const AgentOverlay = lazy(() => import('./components/AgentOverlay'))

function LoadingFallback() {
  const { t } = useTranslation()
  return (
    <div className="min-h-screen bg-background flex items-center justify-center">
      <div className="flex flex-col items-center gap-4">
        <p className="text-[13px] font-medium text-muted-foreground">
          {t('common.loading')}
        </p>
      </div>
    </div>
  )
}

export default function AppRouter() {
  const [label, setLabel] = useState(null)

  useEffect(() => {
    useSettingsStore.getState().hydrate()
    const win = getCurrentWebviewWindow()
    setLabel(win.label)
  }, [])

  if (label === null) return <LoadingFallback />

  if (label === 'settings') {
    return (
      <Suspense fallback={<LoadingFallback />}>
        <div className="min-h-screen bg-background text-foreground overflow-auto">
          <ControlPanel />
        </div>
      </Suspense>
    )
  }

  if (label === 'agent') {
    return (
      <Suspense fallback={<LoadingFallback />}>
        <div className="min-h-screen bg-background text-foreground overflow-auto">
          <AgentOverlay />
        </div>
      </Suspense>
    )
  }

  return <App />
}
