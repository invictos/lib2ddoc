# ⚡ GitLab CI/CD

## 📝 Explication
GitLab CI/CD est une fonctionnalité qui permet de mettre en place des pipelines de CI/CD pour n'importe quel projet, qu'il soit nouveau ou existant, pourvu qu'il utilise Git.
Il nous permet d'automatiser les étapes :

- d'intégration continue : Build > Tests (unitaires, d'intégration, de non-régression...)

- de déploiement continu : Review > Déploiement (staging, production...)

Cette automatisation accélère la production de code : un seul commit suffit à déclencher une pipeline côté GitLab qui s'occupera de générer un build de production, lancer la suite de tests et déployer la nouvelle version en staging/production ! Cela permet également d'augmenter la confiance des développeurs et la qualité du code envoyé en production, car on a l'assurance que chaque modification est passée par ce processus.


## ⚒ Pratique

:::caution
Gitlab ne garde qu'un seul site par projet. On utilise AWS pour sauvegarder tous les sites de toutes les branches
:::

```yaml
stages:
  - build
  - test
  - deploy

build_rust:
  stage: build
  image: rust:latest
  before_script:
    - apt update && apt -y install libdmtx-dev
  script:
    - echo "Building library..."
    - cargo build

test_rust:
  stage: test
  image: rust:latest
  needs:
    - build_rust
  before_script:
    - apt update && apt -y install libdmtx-dev
  script:
    - echo "Testing library..."
    - cargo test

#Build once for gitlab pages (to ../public). And a second time for live URL.
pages:
  stage: build
  image: node:15.12-alpine3.13
  script:
  - cd website
  - yarn install
  - yarn build
  - mv ./build ../public
  - >-
    sed -i -e "s/baseUrl: '\/lib2ddoc\/',/baseUrl: '\/$CI_COMMIT_SHORT_SHA\/',/g" docusaurus.config.js
  - >-
    sed -i -e "s/url: 'https:\/\/pages.gitlab.io',/url: 'https:\/\/lib2ddoc.aws.ipv4.ovh',/g" docusaurus.config.js
  - yarn build
  - mv ./build ../public2
  artifacts:
    paths:
    - public
    - public2

deploy_website:
  stage: deploy
  needs: 
    - pages
  image: registry.gitlab.com/gitlab-org/cloud-deploy/aws-base:latest
  script:
    - >-
      aws s3 sync public2/ "s3://ovh.ipv4.aws.lib2ddoc/$CI_COMMIT_SHORT_SHA/"

```