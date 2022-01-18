# 🌍 Site Web
## 🛠 Construction

Docusaurus est un générateur de site web statique donc nous avons besoin de construire le site web dans un répertoire de contenu statique et de le mettre sur un serveur web pour qu'il puisse être consulté. Pour construire le site web :

```bash
cd website
npm run build
```

## 🚀 Exécution
:::tip
Docusaurus possède une fonctionalité de hot-reload !
:::

```bash
cd website
npm run start
```

## ✏ Nouveau document

Créez un fichier Markdown et placez-le dans le répertoire `docs/`:

```website # répertoire racine de votre site
├── docs
│   └── hello-world.md
├── docusaurus.config.js
├── ...
```

De la même manière, une page sera créée à l'adresse `http://localhost:3000/docs/hello`.  
Maintenant, il faut ajouter le path du markdown créé dans le fichier `sidebars.js` et `docusaurus.config.js`