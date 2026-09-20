import {themes as prismThemes} from 'prism-react-renderer';
import type {Config} from '@docusaurus/types';
import type * as Preset from '@docusaurus/preset-classic';

const config: Config = {
  title: 'Oxid',
  tagline: 'Zero-setup game engine with instant JavaScript scripting',
  favicon: 'img/oxid_favicon.ico',

  future: {
    v4: true,
  },

  url: 'https://eujandergois.github.io',
  baseUrl: '/oxid/',
  organizationName: 'EuJanderGois',
  projectName: 'oxid',

  onBrokenLinks: 'throw',
  markdown: {
    hooks: {
      onBrokenMarkdownLinks: 'throw',
    },
  },
  i18n: {
    defaultLocale: 'en',
    locales: ['en'],
  },

  presets: [
    [
      'classic',
      {
        docs: {
          routeBasePath: '/',
          sidebarPath: './sidebars.ts',
          editUrl: 'https://github.com/EuJanderGois/oxid/tree/main/website/',
          breadcrumbs: true,
        },
        blog: {
          showReadingTime: true,
          blogTitle: 'Oxid Blog',
          blogDescription: 'Notes and release updates for Oxid.',
          feedOptions: {
            type: ['rss', 'atom'],
            xslt: true,
          },
          editUrl: 'https://github.com/EuJanderGois/oxid/tree/main/website/',
          onInlineTags: 'warn',
          onInlineAuthors: 'warn',
          onUntruncatedBlogPosts: 'warn',
        },
        theme: {
          customCss: './src/css/custom.css',
        },
      } satisfies Preset.Options,
    ],
  ],

  themeConfig: {
    image: 'img/oxid.svg',
    colorMode: {
      respectPrefersColorScheme: true,
    },
    navbar: {
      title: 'Oxid',
      logo: {
        alt: 'Oxid',
        src: 'img/oxid.svg',
      },
      items: [
        {to: '/getting-started', label: 'Getting Started', position: 'left'},
        {to: '/cli', label: 'CLI', position: 'left'},
        {to: '/scripting', label: 'Scripting', position: 'left'},
        {to: '/api', label: 'API', position: 'left'},
        {to: '/roadmap', label: 'Roadmap', position: 'left'},
        {
          href: 'https://github.com/EuJanderGois/oxid',
          label: 'GitHub',
          position: 'right',
        },
      ],
    },
    footer: {
      style: 'dark',
      links: [
        {
          title: 'Docs',
          items: [
            {
              label: 'Overview',
              to: '/intro',
            },
            {
              label: 'Getting Started',
              to: '/getting-started',
            },
            {
              label: 'API Reference',
              to: '/api',
            },
          ],
        },
        {
          title: 'Project',
          items: [
            {
              label: 'GitHub',
              href: 'https://github.com/EuJanderGois/oxid',
            },
            {
              label: 'Roadmap',
              to: '/roadmap/v1',
            },
          ],
        },
        {
          title: 'Architecture',
          items: [
            {
              label: 'CLI',
              to: '/cli/overview',
            },
            {
              label: 'Scripting Modules',
              to: '/scripting/modules',
            },
          ],
        },
      ],
      copyright: `Copyright © ${new Date().getFullYear()} Oxid.`,
    },
    prism: {
      theme: prismThemes.github,
      darkTheme: prismThemes.dracula,
    },
  } satisfies Preset.ThemeConfig,
};

export default config;
