# 📒 Format général

## 📝 Structure
Un code 2D-Doc est constitué de plusieurs champs, deux champs principaux et un champ optionnel:
- la zone de données 
- la signature des données
- éventuellement des données annexes.

La **zone de données** des données est composé lui-même de deux sous-zones: 
- La **zone d'en-tête** contient les informations nécessaires pour décoder et vérifier le Code 2D-Doc. 
- La **zone de message** contient les informations propres à chaque code 2D-Doc. Le message est représenté  par une suite de couples `(id,données)`.


Le champ de la **signature** contient l'information qui nous permet de vérifier l'intégralité de nos données.

Enfin, le champ de **données annexe** optionnelle a la même structure que la zone de message mais se trouve après la zone de signature. Puisque c’est un champ optionnel, son contenu n’est pas pris en compte dans la signature.

![Format 2D-DOC ](/img/images/format_global_code2D-Doc.PNG)


## ⚒ Formats
Plusieurs versions du 2D-DOC ont été mises à jour dès son début:


### 🔠 C40

Le format C40 est utilisé pour encoder des données constituées essentiellement de caractères majuscules. Il transforme trois caractères alphanumériques en deux octets grâce à un tableau de caractères réduit à 40 caractères, encodés en **base 40**. 

L’avantage du format C40 c’est qu’il permet d’atteindre la capacité maximum alphanumérique en utilisant uniquement des chiffres, des espaces et des majuscules. C’est donc un codage optimisé qui limite également la taille des données. Un autre avantage du format C4O est qu’il est lisible sur différentes plateformes et interprétable par différents **lecteurs à code barre**. 

### 🔢 Binaire

Le format binaire utilise un codage binaire c'est-à-dire le système de numérotation en **base 2**, il permet par exemple d'inclure de petites (**images**) dans les 2DDocs.

Ci-dessus des exemples de format 2D-DOC. On remarque que les versions ‘01’ à ‘04’ contiennent un message C40, et la signature de la version ‘01’ est en binaire puis en C40 dans les versions ‘02’, ‘03’ et ‘04’. Enfin le quatrième bloc: l’annexe,  est optionnel.
