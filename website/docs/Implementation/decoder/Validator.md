# ✔ Validator

La validation effective d'un document s'effectue après son décodage, a l'aide des elements de **Specification** ([voir ici](../Specifications/Fonctions_validations.md))

On valide donc chaque **DI** (la FieldZone) ainsi que le **Document** lui même

```rust
Validity{
    valid: fz_valid && doc_valid
}
```
