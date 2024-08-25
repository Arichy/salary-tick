import { NavLink } from 'react-router-dom';
import type { IconProps } from '@tabler/icons-react';
import { IconSettings } from '@tabler/icons-react';
import classNames from 'classnames';
import styles from './NavLinks.module.scss';
import { useTranslation } from 'react-i18next';

export default function NavLinks() {
  const { t } = useTranslation();

  const navlinks = [
    {
      title: t('settings'),
      href: '/settings',
      renderIcon: (props: IconProps) => <IconSettings {...props} />,
    },
  ];

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
