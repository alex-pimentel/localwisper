import dePrompts from './de/prompts.json'
import enPrompts from './en/prompts.json'
import esPrompts from './es/prompts.json'
import frPrompts from './fr/prompts.json'
import itPrompts from './it/prompts.json'
import jaPrompts from './ja/prompts.json'
import ptPrompts from './pt/prompts.json'
import ruPrompts from './ru/prompts.json'
import zhCNPrompts from './zh-CN/prompts.json'
import zhTWPrompts from './zh-TW/prompts.json'

export const PROMPTS_BY_LOCALE = {
  en: enPrompts,
  es: esPrompts,
  fr: frPrompts,
  de: dePrompts,
  pt: ptPrompts,
  it: itPrompts,
  ru: ruPrompts,
  ja: jaPrompts,
  'zh-CN': zhCNPrompts,
  'zh-TW': zhTWPrompts,
}
