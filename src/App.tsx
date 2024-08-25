import { useEffect, useLayoutEffect, useState } from 'react';
import { invoke } from '@tauri-apps/api/tauri';
import { listen } from '@tauri-apps/api/event';
import { AppShell, Group, MantineProvider, Text } from '@mantine/core';
import { Outlet } from 'react-router-dom';
import NavLinks from './pages/Root/components/NavLinks/NavLinks';
import { SettingsProvider } from './context/settings';
import { Settings as ISettings, NativeSettingsJSON } from './types';
import { IconMoodWink2 } from '@tabler/icons-react';

import '@mantine/core/styles.css';
import './App.css';

function App() {
  const [settings, setSettings] = useState<ISettings>({
    dailySalary: 0,
    startTime: '09:00',
    endTime: '17:00',
    currencySymbol: '$',
  });

  useLayoutEffect(() => {
    invoke<NativeSettingsJSON>('load_settings')
      .then(res => {
        console.log(res);
        setSettings({
          dailySalary: res.daily_salary || 0,
          startTime: res.start_time || '09:00',
          endTime: res.end_time || '17:00',
          currencySymbol: res.currency_symbol || '$',
        });
      })
      .catch(err => {
        console.log(err);
      });
  }, []);

  const [earnings, setEarnings] = useState('0');
  useEffect(() => {
    listen<number>('update-earnings', ({ payload }) => {
      setEarnings(payload.toFixed(4));
    });
  }, []);

  return (
    <SettingsProvider value={settings}>
      <MantineProvider>
        <AppShell header={{ height: 60 }} navbar={{ width: 200, breakpoint: 'sm' }} padding="md">
          <AppShell.Header p="md">
            <Group gap="xs">
              <IconMoodWink2 />
              <Text>
                Congratulations! You’ve earned {settings.currencySymbol} {earnings} today!
              </Text>
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
  );
}

export default App;
