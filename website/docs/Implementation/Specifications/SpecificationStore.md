# 🏪 SpecificationStore

## 💡 Idée
Nous avons besoin des spécifications à de nombreux moments de la procédure de décodage.

Nous avons donc décidé d'implémenter nos spécifications à travers une structure suivant un pattern Singleton:
Une hashmap est chargée durant l'initialisation, et gardée en mémoire durant la vie du programme.

Ces structures sont **FieldSpecificationStore** et **DocumentSpecificationStore**, et implémente le trait **SpecificationStore**

![SpecificationStore](/img/images/UML_SpecStore.PNG)

## 🔧 JSON -> HashMap
C'est notre toolchain rust (build.rs) qui transforme le JSON en code rust, corps de l'implémentation fill de **SpecificationStore**. 

## 🏁 Execution
Lors de l'exécution du programme, c'est une crate rust (**lazy_static**) qui crée une référence accessible de manière globale. On a donc bien une hashmap statique !