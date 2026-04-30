// MIT License
// Copyright (c) Destroyer 2026.
import en, { type TranslationKey } from './locales/en';
import es from './locales/es';
import de from './locales/de';
import fr from './locales/fr';

export type Language = 'en' | 'es' | 'de' | 'fr';

const locales: Record<Language, Record<TranslationKey, string>> = { en, es, de, fr };

export function getT(language: Language) {
  const locale = locales[language] ?? locales.en;
  return function t(key: TranslationKey): string {
    return locale[key] ?? en[key] ?? key;
  };
}

export type { TranslationKey };
