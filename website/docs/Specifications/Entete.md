# 🎩 En-tête

## 📄 Description
Il y a différentes versions d'en-tête utilisées pour le dispositif 2D-DOC. L’en-tête contient les informations qui permettent de décoder et de vérifier les informations du 2D-DOC. La longueur de l’en-tête est de 22 bits pour les versions ‘01’, ‘02’, 24 bits pour la version ‘03’ et 26 bits pour la version ‘04’  car on ajoute des informations en plus dans les versions ‘03’ et ‘04’.

L’en-tête contient tout ce qui est relié à l'identification du document. Ces informations permettent d’interpréter l’information contenu dans un 2D-DOC car elles fournissent le type de Code 2D-DOC, la version du code, les identifiants etc.

- Tout d’abord , on retrouve dans l'en-tête un **marqueur** d’identification du document. Celui-ci prend toujours la valeur `DC`. 
- Ensuite on a la **version** du 2D-DOC (eg. `02`)

- Puis on a l’**Identifiant de l’Autorité de Certification** ayant délivré le certificat utilisé par l’émetteur du document pour signer les Données. En fonction du format de codage, celui-ci peut être codé sur 4 caractères alphanumériques (format C40) ou sur 5 caractères hexadécimaux (format BINAIRE). Un exemple d’Identifiant de l’Autorité de Certification est Certigna `FR03` (Service de sécurité informatique) ou Ariadnext `FR04` (Entreprise de logiciels). 

- Après, on a l’**Identifiant du Certificat** utilisé pour signer les données sur quatre caractères alphanumériques. Ainsi on récupère les certificats via des *Annuaire de certificats* qui suivent la norme [RFC 4387](https://datatracker.ietf.org/doc/html/rfc4387). Ce certificat contient une clé publique qui est distribuée aux tierces parties intéressées par la vérification du code 2D-Doc. Ce certificat ne peut être stocké directement dans le 2D-DOC car il y aurait trop d’informations à coder dans le code à barres et celui-ci serait beaucoup trop grand. (eg. `AB04`)

- on a également la **date d'émission** et la **date de signature** du document codé en hexadécimal depuis le premier janvier 2000. Par exemple, la date du 29 avril 2021 correspond à `1E6D`.

- Enfin, le code d’identification du **type de document** (**[détails](Types_de_documents.md)**) sur deux caractères alphanumériques.


- La version ‘03’ du 2D-DOC se différencie des versions précédentes par l’ajout d’un champ pour l’**identifiant du périmètre** sur lequel le type de document est défini. Pour l'instant, l'ensemble des documents est sous l’identifiant `01` (périmètre ANTS).

- La version ‘04’ du 2D-DOC on ajoute  l’information **Pays émetteur du document**. Pour la France, cela correspond au code `FR`.

:::note
On obtient un code de la forme suivante pour l’en-tête des versions C40 ‘01’ et ‘02’ :
`DC02ANTSXT4A1E6D1E6D01`
:::


## ⚒ Récapitulatif
En-tête d’un code 2D-Doc en C40 pour la version 04 :

![En-tête d’un code 2D-Doc](/img/images/entete.PNG)
