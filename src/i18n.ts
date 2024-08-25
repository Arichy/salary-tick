import i18n from 'i18next';
import { initReactI18next } from 'react-i18next';

export const I18N_KEYS = {
  title: 'title',
  settings: 'settings',
  dailySalary: 'dailySalary',
  startTime: 'startTime',
  endTime: 'endTime',
  currencySymbol: 'currencySymbol',
  language: 'language',
};

const resources = {
  en: {
    translation: {
      [I18N_KEYS.title]: 'Congratulations! You’ve earned {{currencySymbol}} {{earnings}} today!',
      [I18N_KEYS.settings]: 'Settings',
      [I18N_KEYS.dailySalary]: 'Daily Salary',
      [I18N_KEYS.startTime]: 'Start Time',
      [I18N_KEYS.endTime]: 'End Time',
      [I18N_KEYS.currencySymbol]: 'Currency Symbol',
      [I18N_KEYS.language]: 'Language',
    },
  },
  zh: {
    translation: {
      [I18N_KEYS.title]: '恭喜！您今天赚了 {{currencySymbol}}{{earnings}}！',
      [I18N_KEYS.settings]: '设置',
      [I18N_KEYS.dailySalary]: '每日工资',
      [I18N_KEYS.startTime]: '开始时间',
      [I18N_KEYS.endTime]: '结束时间',
      [I18N_KEYS.currencySymbol]: '货币符号',
      [I18N_KEYS.language]: '语言',
    },
  },
};

i18n.use(initReactI18next).init({
  resources,
  lng: 'en',
  interpolation: {
    escapeValue: false,
  },
});

export default i18n;
