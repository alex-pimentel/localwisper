import { useState, useCallback } from 'react'

export function useDialogs() {
  const [confirmDialog, setConfirmDialog] = useState({
    open: false,
    title: '',
    onConfirm: () => {},
  })
  const [alertDialog, setAlertDialog] = useState({ open: false, title: '' })

  const showConfirmDialog = useCallback((options) => {
    setConfirmDialog({ ...options, open: true })
  }, [])

  const showAlertDialog = useCallback((options) => {
    setAlertDialog({ ...options, open: true })
  }, [])

  const hideConfirmDialog = useCallback(() => {
    setConfirmDialog((prev) => ({ ...prev, open: false }))
  }, [])

  const hideAlertDialog = useCallback(() => {
    setAlertDialog((prev) => ({ ...prev, open: false }))
  }, [])

  return {
    confirmDialog,
    alertDialog,
    showConfirmDialog,
    showAlertDialog,
    hideConfirmDialog,
    hideAlertDialog,
  }
}
