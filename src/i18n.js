import i18n from 'i18next'
import { initReactI18next } from 'react-i18next'
import { PROMPTS_BY_LOCALE } from './locales/prompts'
import { TRANSLATIONS_BY_LOCALE } from './locales/translations'

export const SUPPORTED_UI_LANGUAGES = ['en', 'es', 'fr', 'de', 'pt', 'it', 'ru', 'ja', 'zh-CN', 'zh-TW']

export function normalizeUiLanguage(language) {
  const candidate = (language || '').trim()
  const normalized = candidate.replace('_', '-')
  const fullMatch = SUPPORTED_UI_LANGUAGES.find(
    (lang) => lang.toLowerCase() === normalized.toLowerCase()
  )
  if (fullMatch) return fullMatch
  const base = candidate.split('-')[0].split('_')[0].toLowerCase()
  if (SUPPORTED_UI_LANGUAGES.includes(base)) return base
  return 'en'
}

const resources = {}
for (const [lang, translation] of Object.entries(TRANSLATIONS_BY_LOCALE)) {
  resources[lang] = {
    translation,
    prompts: PROMPTS_BY_LOCALE[lang],
  }
}

const browserLanguage = typeof navigator !== 'undefined' ? navigator.language || navigator.languages?.[0] : undefined
const storageLanguage = typeof window !== 'undefined' ? window.localStorage.getItem('uiLanguage') : undefined
const initialLanguage = normalizeUiLanguage(storageLanguage || browserLanguage || 'en')

void i18n.use(initReactI18next).init({
  resources,
  lng: initialLanguage,
  fallbackLng: 'en',
  ns: ['translation', 'prompts'],
  defaultNS: 'translation',
  interpolation: {
    escapeValue: false,
  },
  returnEmptyString: true,
  returnNull: false,
})

export default i18n
