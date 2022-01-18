# 🧰 FieldZoneDecoder

Un message est constitué d’une séquence de blocs de données. Chaque bloc de données est constitué des éléments suivants :

- D'un **Identifiant de Donnée** (ID) sur deux caractères.
- De la **Donnée** (D).
- D’un éventuel caractère de fin de donnée **GS** ou de troncature de donnée **RS**.

Selon l’ID, la Donnée peut-être :
- De longueur fixe
- De longueur variable, avec une borne supérieure
- De longueur variable avec des bornes inférieure et supérieure
- De longueur variable non bornée

On récupère ces informations à l'aide du **FieldSpecificationStore** 

```rust
FieldSpecificationsStore::get(&id)
```

On cherche ensuite **RS** ou **GS** dans la zone \[B_inf, B_sup\], et on consomme la sous-chaine.

:::info
C'est aussi ici que l'on ajoute les informations de troncature
:::

A la fin, on retourne une FieldZone complète, mais pas validé.