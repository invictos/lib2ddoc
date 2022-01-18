/** @type {import('@docusaurus/types').DocusaurusConfig} */
module.exports = {
  title: 'Lib2D-Doc',
  tagline: '',
  url: 'https://pages.gitlab.io',


  plugins: [require.resolve('docusaurus-lunr-search')],

  baseUrl: '/lib2ddoc/',

  onBrokenLinks: 'throw',
  onBrokenMarkdownLinks: 'warn',
  favicon: 'img/favicon.ico',
  organizationName: 'YassineBA', // Usually your GitHub org/user name.
  projectName: 'lib 2D-Doc', // Usually your repo name.

  themeConfig: {
    hideableSidebar: true,
    image: "img/logo.svg",
    prism: {
      additionalLanguages: ['rust'],
    },


    navbar: {
      title: 'Lib2D-Doc',
      logo: {
        alt: 'My Site Logo',
        src: 'img/logo.svg',
      },

      items: [
        {
          label: 'Projet',
          position: 'left',
          to: 'docs/project/README',
          items: [
            {
              label: '📖 README',
              to: 'docs/project/README',
            },
            {
              label: '📚 Librairie',
              to: 'docs/project/FFI_out',
            },
            {
              label: '💭 UML',
              to: 'docs/project/UML',
            },
            {
              label: '📈 Amélioration',
              to: 'docs/project/Ameliorations_possibles',
            },
            {
              label: '🌌 Site Web',
              to: 'docs/project/Ameliorations_possibles',
            },
            {
              label: '⚡ GitLab CI/CD',
              to: 'docs/project/Ameliorations_possibles',
            },
            {
              label: '🔥 Démonstration',
              to: 'docs/project/Ameliorations_possibles',
            },
          ],
        },

        {
          label: 'Spécifications',
          position: 'left',
          items: [
            {
              label: '⚓ Introduction',
              to: 'docs/Specifications/Introduction',
            },

            {
              label: '📒 Format général',
              to: 'docs/Specifications/Format_2DDoc',
            },

            {
              label: '🎩 En-tête',
              to: 'docs/Specifications/Entete',
            },

            {
              label: '📨 Zone de Message',
              to: 'docs/Specifications/Message',
            },

            {
              label: '🔑 Signature',
              to: 'docs/Specifications/Signature',
            },
            
            {
              label: '🔓 Zone Annexe',
              to: 'docs/Specifications/Annexe',
            },
            
            {
              label: '📚 Types de documents',
              to: 'docs/Specifications/Types_de_documents',
            },

          ],
        },
        
        {
          label: 'Implémentation',
          position: 'left',
          items: [
            {
              label: '📝 Spécifications',
              to: 'docs/Implementation/Specifications/GDoc+GDoc->JSON',
            },
            
            {
              label: '🛠 Décodeur',
              to: 'docs/Implementation/decoder/Entete',
            },
          ],
        },
        
        
        
        
        {
          label: 'Modules externes',
          position: 'left',
          items: [
            {
              label: '🔐 Sécurité',
              to: 'docs/Modules_externes/securite/securite_specs',
            },
            {
              label: '🚀 libDMTX',
              to: 'docs/Modules_externes/ffi',
            },
            {
              label: '💻 Bindgen',
              to: 'docs/Modules_externes/bindgen',
            },
            {
              label: '📎 Sérialiser',
              to: 'docs/Modules_externes/serialiser',
            },
          ],
        },

      ],
    },



    footer: {
      style: 'light',
      copyright: `By <a href='https://www.linkedin.com/in/yassine-ben-abderrahmane/'>Yassine BenAbderrahmane</a>, <a href='https://github.com/invictos'>Antoine Camusat</a>, Florin Croitoru. Made with ❤ at <a href='https://www.insa-rouen.fr/'>INSA Rouen</a>`,
    },
  },



  presets: [
    [
      '@docusaurus/preset-classic',
      {
        docs: {
          routeBasePath: 'docs',
          path: 'docs',
          sidebarPath: require.resolve('./sidebars.js'),
          lastVersion: 'current',
          onlyIncludeVersions: ['current'],
          // Please change this to your repo.
          editUrl:
            'https://gitlab.insa-rouen.fr/acamusat/lib2ddoc/-/edit/SiteWeb/website/',
        },

        blog: {
          showReadingTime: true,
          // Please change this to your repo.
          editUrl:
            'https://gitlab.insa-rouen.fr/acamusat/lib2ddoc/-/edit/SiteWeb/website/',
        },
        theme: {
          customCss: require.resolve('./src/css/custom.css'),
        },
      },
    ],
  ],
};
