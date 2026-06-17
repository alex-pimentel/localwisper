export const API_ENDPOINTS = {
  OPENAI_BASE: 'https://api.openai.com/v1',
  ANTHROPIC: 'https://api.anthropic.com/v1/messages',
  GEMINI: 'https://generativelanguage.googleapis.com/v1beta',
  GROQ_BASE: 'https://api.groq.com/openai/v1',
  XAI_BASE: 'https://api.x.ai/v1',
  MISTRAL_BASE: 'https://api.mistral.ai/v1',
}

export const MODEL_CONSTRAINTS = {
  MIN_FILE_SIZE: 1000000,
  MODEL_TEST_TIMEOUT: 5000,
  INFERENCE_TIMEOUT: 30000,
}

export const TOKEN_LIMITS = {
  MIN_TOKENS: 512,
  MAX_TOKENS: 2048,
}

export const CACHE_CONFIG = {
  API_KEY_TTL: 3600000,
  MODEL_CACHE_SIZE: 3,
  AVAILABILITY_CHECK_TTL: 30000,
  PASTE_DELAY_MS: 50,
}

export const RETRY_CONFIG = {
  MAX_RETRIES: 3,
  INITIAL_DELAY: 1000,
  MAX_DELAY: 10000,
  BACKOFF_MULTIPLIER: 2,
}

export const WHISPER_MODELS = ['tiny', 'base', 'small', 'medium', 'large', 'turbo']

export const DEFAULT_HOTKEYS = {
  dictation: 'Alt+R',
  agent: 'Alt+Shift+A',
  voiceAgent: null,
  meeting: null,
}
