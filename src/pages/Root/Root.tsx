import { AppShell, MantineProvider } from '@mantine/core';
import '@mantine/core/styles.css';
import '@mantine/dates/styles.css';
import { Outlet } from 'react-router-dom';
import NavLinks from './components/NavLinks/NavLinks';
import './Root.scss';

export default function Root() {
  return (
    <MantineProvider>
      <AppShell navbar={{ width: 200, breakpoint: 'sm' }}>
        <AppShell.Navbar>
          <NavLinks />
        </AppShell.Navbar>
        <AppShell.Main>
          <Outlet />
        </AppShell.Main>
      </AppShell>
    </MantineProvider>
  );
}
