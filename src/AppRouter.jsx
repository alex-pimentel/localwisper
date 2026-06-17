import { Suspense, lazy, useEffect } from 'react'
import { useTranslation } from 'react-i18next'
import { useSettingsStore } from './stores/settingsStore'
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
  useEffect(() => {
    useSettingsStore.getState().hydrate()
  }, [])

  const params = new URLSearchParams(window.location.search)

  if (params.get('panel') === 'true' || window.location.pathname.includes('control')) {
    return (
      <Suspense fallback={<LoadingFallback />}>
        <ControlPanel />
      </Suspense>
    )
  }

  if (params.get('agent') === 'true') {
    return (
      <Suspense fallback={<LoadingFallback />}>
        <AgentOverlay />
      </Suspense>
    )
  }

  return <App />
}
