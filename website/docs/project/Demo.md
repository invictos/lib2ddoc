# 🔥 Démonstration 

## Un passe sanitaire
:::note
Ce document est un document factice, dédié aux tests ! 
:::
![02](/img/images/02-v4.png)

### Données

#### Texte:
:::info
C'est ce que contient le datamatrix (2d-doc)
:::

`DC04FR0000011E581E58B201FRF0DEPOTHPRIMF1NEGF201011955F3FF4945006F5NF6070420211010MRZICP23SZTCLV2APC5P2AQVZDKHDKRG5KOIYGBKDXUA7N5O75QLWMKJBFSL3WJZYG2HHF4Z3K5E6E2GJFOVSV7E2W2XVHEXXVDXYNY`

#### JSON:
:::info
C'est ce que renvoie notre programme
:::
```json
{
  "is_ok": true,
  "headers": {
    "marqueur": "DC",
    "version": 4,
    "identifiant_de_ac": "FR00",
    "identifiant_du_certificat": "0001",
    "date_emission": "1E58",
    "date_signature": "1E58",
    "type_document": "B2",
    "type_document_description": "Test COVID",
    "perimetre": "01",
    "pays_emetteur": "FR"
  },
  "message": {
    "F6": {
      "name": "Date et heure du prélèvement",
      "value": "070420211010",
      "is_valid": true
    },
    "F3": {
      "name": "Genre",
      "value": "F",
      "is_valid": true
    },
    "F4": {
      "name": "Code analyse",
      "value": "945006",
      "is_valid": true
    },
    "F0": {
      "name": "Liste des prénoms",
      "value": "DEPOTHPRIM",
      "is_valid": true
    },
    "F2": {
      "name": "Date de naissance",
      "value": "01011955",
      "is_valid": true
    },
    "F5": {
      "name": "Résultat de l’analyse",
      "value": "N",
      "is_valid": true
    },
    "F1": {
      "name": "Nom patronymique",
      "value": "NEG",
      "is_valid": true
    }
  },
  "is_signature_valid": true,
  "annex": "",
  "is_document_valid": true
}
```