# 📎 Serialiser
La sérialisation de document est implémentée dans le module "serialize"

## 📝 Interface

On propose l'interface suivante pour sérialiser un Document ou une Erreur: 

> pub fn serialize(doc: Result<Document, ErrorDocument>) -> String

Chacun est libre d'implémenter le "sérialiseur" qu'il préfère.

## 🧰Implémentation

Nous avons implémenté une serialisation vers JSON. En effet, ce format de données est compris par la plupart des plateformes et permet donc une grande interopérabilité

Nous avons utilisé la crate rust serde, qui permet de sérialiser vers de nombreux formats.

On a une "vue" correspondant aux informations du Document que l'on souhaite afficher

```rust
struct DocumentJSON<'a>{
    is_ok: bool,
    headers: HeadersJSON<'a>,
    message: HashMap<&'a str, FieldJSON<'a>>,
    is_signature_valid: bool,
    annex: &'a str,
    is_document_valid: bool
}


struct FieldJSON<'a>{
    name: &'a str,
    value: &'a str,
    is_valid: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    errors: Option<Vec<String>>
}

struct HeadersJSON<'a>{
    marqueur : &'a str,
    version : u8,
    identifiant_de_ac : &'a str,
    identifiant_du_certificat : &'a str,
    date_emission : &'a str,
    date_signature: &'a str,
    type_document: &'a str,
    type_document_description: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    perimetre : Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pays_emetteur : Option<&'a str>,
}

struct ErrorJSON<'a>{
    is_ok: bool,
    kind: String,
    value: &'a str
}
```

:::info
Des exemples de sortie sont disponible  [ici](../project/Demo.md)
:::