# 🔨 Implémentation
L'implémentation de notre module sécurité à été pensée pour être interchangeable (pour une utilisation de openSSL par ex.)

On a donc une interface unique, sous la forme d'une fonction, qui valide ou non un document:

```rust
pub fn check_signature(data_zone: &str, signature: &str, authority_id: &str, certificate_id: &str) -> Result<bool, String>
```

La fonction renvoie donc un booléen correspondant à la validité du document, ou un message d'erreur.

:::info
- La vérification nécessitant l'accès aux Annuaire de certificats, notre implémentation actuelle nécessite une connexion internet.

- Une évolution possible est l'utilisation d'un "cache" de certificats, pour une utilisation hors ligne.
:::