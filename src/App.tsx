import { useEffect, useLayoutEffect, useState } from 'react';
import { invoke } from '@tauri-apps/api/tauri';
import { listen } from '@tauri-apps/api/event';
import { AppShell, Group, MantineProvider, Text } from '@mantine/core';
import { Outlet } from 'react-router-dom';
import NavLinks from './pages/Root/components/NavLinks/NavLinks';
import { SettingsProvider } from './context/settings';
import { Settings as ISettings, NativeSettingsJSON } from './types';
import { IconMoodWink2 } from '@tabler/icons-react';
import './i18n';

import '@mantine/core/styles.css';
import './App.css';
import { useTranslation } from 'react-i18next';
import { I18N_KEYS } from './i18n';

function App() {
  const [settings, setSettings] = useState<ISettings | null>(null);

  const { i18n, t } = useTranslation();

  useEffect(() => {
    if (settings) {
      i18n.changeLanguage(settings.language);
    }
  }, [settings?.language]);

  useLayoutEffect(() => {
    invoke<NativeSettingsJSON>('load_settings')
      .then(res => {
        setSettings({
          dailySalary: res.daily_salary || 0,
          startTime: res.start_time || '09:00',
          endTime: res.end_time || '17:00',
          currencySymbol: res.currency_symbol || '$',
          language: res.language || 'en',
        });
      })
      .catch(err => {
        console.log(err);
      });
  }, []);

  const [earnings, setEarnings] = useState('0');
  useEffect(() => {
    const unlisten = listen<number>('update-earnings', ({ payload }) => {
      setEarnings(payload.toFixed(4));
    });

    return () => {
      unlisten.then(fn => fn());
    };
  }, []);

  useEffect(() => {
    const unlisten = listen<string>('update-language', ({ payload }) => {
      setSettings(prev => (prev ? { ...prev, language: payload } : null));
    });

    return () => {
      unlisten.then(fn => fn());
    };
  }, []);

  useEffect(() => {
    const unlisten = listen<NativeSettingsJSON>('update-settings', ({ payload }) => {
      setSettings({
        dailySalary: payload.daily_salary || 0,
        startTime: payload.start_time || '09:00',
        endTime: payload.end_time || '17:00',
        currencySymbol: payload.currency_symbol || '$',
        language: payload.language || 'en',
      });
    });

    return () => {
      unlisten.then(fn => fn());
    };
  }, []);

  return settings ? (
    <SettingsProvider value={settings}>
      <MantineProvider>
        <AppShell header={{ height: 60 }} navbar={{ width: 200, breakpoint: 'sm' }} padding="md">
          <AppShell.Header p="md">
            <Group gap="xs">
              <IconMoodWink2 />
              <Text>{t(I18N_KEYS.title, { earnings, currencySymbol: settings.currencySymbol })}</Text>
            </Group>
          </AppShell.Header>
          <AppShell.Navbar>
            <NavLinks />
          </AppShell.Navbar>
          <AppShell.Main>
            <Outlet />
          </AppShell.Main>
        </AppShell>
      </MantineProvider>
    </SettingsProvider>
  ) : null;
}

export default App;
