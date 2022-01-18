# 🥇 Fonctions de validation

## 📝 Principe
Pour vérifier la validité d'un document, on doit tester deux principaux points :
- Si il contient bien tous les DI obligatoires.
- Si la valeur des DI est conforme a leurs spécifications.


Pour cela, on utilise deux méthodes de validation pour chaque point: 
- Une méthode ensembliste:   
    On va tester si les valeurs sont présentes dans un ensemble prédéterminé
- Une méthode via fonction
    On va appliquer une fonction de validation sur les valeurs

On a donc 4 fichiers lié à la validation. Les appels à ces fonctions de validation sont inclus dans les Specifications.


## 🔧 Exemple avec FieldSpecification:
```rust 

pub struct FieldSpecification{
    pub id: &'static str,
    pub nom: &'static str,
    pub taille_min: i16,
    pub taille_max: i16,
    pub type_identifiant: &'static str,
    pub description: &'static str,
    pub validate: ValidatingConditionField,
}

pub enum ValidatingConditionField {
    FUNCTION(Vec<fn(&str, bool) -> Result<(), Error>>),
    SET(Vec<fn(&char, bool) -> Result<(), Error>>)
}

```

et on a une méthode de FieldSpecification qui permet d'effectuer la validation:

```rust
pub fn validate(&self, value: &str, truncated: bool) -> Result<(), Vec<Error>>
```

:::info
On peut même tester plusieurs fonction sur un même champ, c'est-à-dire tester plusieurs conditions.
:::

## ❌ Erreurs
De plus, on a crée un type d'erreur spécifique à la validation des documents:

```
pub struct Error{
    pub kind: ErrorKind,
    pub value: String
}

pub enum ErrorKind {
    General,
    NotInSet,
    FailedFunction,
    SizeNotRespected,
    InvalidCaracter,
    DocumentFields
}
```