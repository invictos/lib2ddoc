# 📖 README

![02](/img/images/01-v2.png)

## 🚀 Introduction

[**📚 documentation**](http://acamusat.pages.insa-rouen.fr/lib2ddoc/)

`FR` Le 2D-DOC est une spécification de datamatrix permettant à l’administration française d’embarquer des informations certifiés dans un document papier.
On peut citer comme exemple les justificatifs de domiciles ou les attestations vaccinales.

Le but de ce projet est de créer une librairie en rust permettant d’interpréter le flux d’octets provenant d’un datamatrix en données structurées suivant les
spécifications de 2D-DOC.

---

`EN` The 2D-DOC is a datamatrix specification allowing the French administration to embed certified information in a paper document.
We can quote as example the proofs of residence or the vaccine certificates.

The goal of this project is to create a library in rust allowing to interpret the flow of bytes coming from a datamatrix in a structured data format, according to the 2D-DOC specifications.

## 🛠 Compilation

Cargo va créer une librairie **.rlib** et **.so** dans le dossier `target/`

### Installation rust
```curl https://sh.rustup.rs -sSf | sh```

### Installation dépendances

```apt install build-essential pkg-config libssl-dev libdmtx-dev```

### Compilation librairie
```cargo build --release```

## 📚 Documentation
[Une documentation complète est disponible **ici**](http://acamusat.pages.insa-rouen.fr/lib2ddoc/)

On détaille le projet, les spécifications 2DDoc, l'implémentation et les modules externes utilisés
```
cd website
npm run build
npm run serve
```