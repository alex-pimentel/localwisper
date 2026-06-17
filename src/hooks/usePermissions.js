import { useState, useCallback, useEffect, useRef } from 'react'
import { useTranslation } from 'react-i18next'

function getPlatform() {
  if (typeof navigator !== 'undefined') {
    const ua = navigator.userAgent.toLowerCase()
    if (ua.includes('win')) return 'win32'
    if (ua.includes('mac')) return 'darwin'
    if (ua.includes('linux')) return 'linux'
  }
  return 'darwin'
}

export function usePermissions(showAlertDialog) {
  const { t } = useTranslation()
  const [micPermissionGranted, setMicPermissionGranted] = useState(
    localStorage.getItem('micPermissionGranted') === 'true'
  )
  const [accessibilityPermissionGranted, setAccessibilityPermissionGranted] = useState(
    localStorage.getItem('accessibilityPermissionGranted') === 'true'
  )
  const [pasteToolsInfo, setPasteToolsInfo] = useState(null)

  const requestMicPermission = useCallback(async () => {
    if (!navigator?.mediaDevices?.getUserMedia) {
      const message = t('hooks.permissions.micUnavailable')
      if (showAlertDialog) {
        showAlertDialog({
          title: t('hooks.permissions.titles.microphoneUnavailable'),
          description: message,
        })
      }
      return
    }

    try {
      const stream = await navigator.mediaDevices.getUserMedia({ audio: true })
      stream.getTracks().forEach((t) => t.stop())
      setMicPermissionGranted(true)
      localStorage.setItem('micPermissionGranted', 'true')
    } catch {
      if (showAlertDialog) {
        showAlertDialog({
          title: t('hooks.permissions.titles.microphonePermissionRequired'),
          description: t('hooks.permissions.micErrors.permissionDenied'),
        })
      }
    }
  }, [showAlertDialog, t])

  const requestAccessibilityPermission = useCallback(async () => {
    const platform = getPlatform()
    if (platform === 'darwin') {
      setAccessibilityPermissionGranted(true)
      localStorage.setItem('accessibilityPermissionGranted', 'true')
    } else if (platform === 'win32') {
      setAccessibilityPermissionGranted(true)
      localStorage.setItem('accessibilityPermissionGranted', 'true')
    } else if (platform === 'linux') {
      setAccessibilityPermissionGranted(true)
      localStorage.setItem('accessibilityPermissionGranted', 'true')
    }
  }, [])

  return {
    micPermissionGranted,
    accessibilityPermissionGranted,
    pasteToolsInfo,
    requestMicPermission,
    requestAccessibilityPermission,
    setMicPermissionGranted,
    setAccessibilityPermissionGranted,
  }
}
