import { NavLink } from 'react-router-dom';
import type { IconProps } from '@tabler/icons-react';
import { IconDeviceNintendo, IconHome, IconSettings, IconUserCircle } from '@tabler/icons-react';
import classNames from 'classnames';
import styles from './NavLinks.module.scss';

const navlinks = [
  {
    title: 'Settings',
    href: '/settings',
    renderIcon: (props: IconProps) => <IconSettings {...props} />,
  },
];

export default function NavLinks() {
  return (
    <>
      {navlinks.map(link => (
        <NavLink
          key={link.href}
          to={link.href}
          className={({ isActive }) => {
            return classNames(styles.navLink, {
              [styles.active]: isActive,
            });
          }}
        >
          {({ isActive }) => {
            return (
              <>
                {link.renderIcon({ stroke: isActive ? 2 : 1.5 })}
                {link.title}
              </>
            );
          }}
        </NavLink>
      ))}
    </>
  );
}
