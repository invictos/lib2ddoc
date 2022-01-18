# ❌ Erreur 
On dispose d'un type d'erreur propre pour les erreurs de décodage:

```rust
pub enum ErrorKind {
    General,
    Impossible,
    NoMatrix,
    FieldZoneDecoder,
    Decoder,
    Signing
}

pub struct Error{
    pub kind: ErrorKind,
    pub value: String
}

```

Nous avons à peu près un type par module, et on ajoute une description détaillé dans le champ **value**. 