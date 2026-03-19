import type {SidebarsConfig} from '@docusaurus/plugin-content-docs';

const sidebars: SidebarsConfig = {
  tutorialSidebar: [
    'intro',
    {
      type: 'category',
      label: 'Getting Started',
      link: {
        type: 'doc',
        id: 'getting-started/index',
      },
      items: [
        'getting-started/installation',
        'getting-started/create-project',
        'getting-started/run-project',
        'getting-started/project-structure',
      ],
    },
    {
      type: 'category',
      label: 'CLI',
      link: {
        type: 'doc',
        id: 'cli/index',
      },
      items: ['cli/overview', 'cli/new', 'cli/run'],
    },
    {
      type: 'category',
      label: 'Scripting',
      link: {
        type: 'doc',
        id: 'scripting/index',
      },
      items: [
        'scripting/lifecycle',
        'scripting/modules',
        'scripting/types',
        'scripting/game-object',
      ],
    },
    {
      type: 'category',
      label: 'API Reference',
      link: {
        type: 'doc',
        id: 'api/index',
      },
      items: [
        'api/core',
        'api/math',
        'api/color',
        'api/shapes',
        'api/input',
        'api/text',
        'api/texture',
      ],
    },
    {
      type: 'category',
      label: 'Roadmap',
      link: {
        type: 'doc',
        id: 'roadmap/index',
      },
      items: ['roadmap/v1'],
    },
  ],
};

export default sidebars;
