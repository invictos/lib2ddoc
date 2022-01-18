# 🎩 En-têtes

Nous avons créé la structure correspondante Headers : 

``` rust
pub struct Headers {
    pub marqueur : String,
    pub version : String,
    pub identifiant_de_ac : String,
    pub identifiant_du_certificat : String,
    pub date_emission : String,
    pub date_signature: String,
    pub type_document: String,
    pub perimetre : Option<String>, // version "03"     
    pub pays_emetteur : Option<String>, // version "04"
}
```

:::tip
On utilise le type **Option** de rust, permettant d'ajouter une valeur par la suite, uniquement pour les documents ayant une version supérieure ou égale à 2
:::

Donc au début, on a la fonction initialiser, qui parse le code et remplit chaque champs de la structure.

Ensuite, en fonction de la version de l'en-tête du 2D-Doc à decrypter on remplit soit le champs perimetre, soit le champs pays-emetteur grâce à la fonction **remplir cas particuliers**.

``` rust
let mut entete = Headers ::initialiser(str); 
entete.remplir_cas_particuliers(str);
```
