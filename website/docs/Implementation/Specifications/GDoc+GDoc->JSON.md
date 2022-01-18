# 🏣 ANTS -> JSON

## 📝 Principe
Pour pouvoir décoder des 2DDOC, nous avons besoin de certaines informations du fichier de spécification de l'ANTS:
- Section 6 : Types de documents
- Section 7 : Identifiants (ID)
- Section 8 : ID dans chaque type de documents

Ces informations permettent notamment de vérifier la validité d'un ID (taille, valeur ...), ou encore les DI obligatoires pour chaque document. 

Nous avons d'abord rassemblé ces données dans un tableur (depuis le fichier PDF). 

Ensuite, nous avons développé un script permettant de rassembler ces données dans un fichier JSON propre à chaque section.

## 🛠 Exemple pour la section S6:

```javascript
function subarray_push(array, i, j, value) {
  let al = array.length;
  for(var k=0;k<(i-al+1);k++){
    array.push([]);
  }

  al = array[i].length;
  for(var k=0;k<(j-al+1);k++){
    array[i].push([]);
  }

  array[i][j].push(value);
}

function s6() {
  var ss = SpreadsheetApp.getActiveSpreadsheet();
  var sheet = ss.getSheetByName("Section 6");
  var range = sheet.getDataRange();
  var values = range.getValues();

  var data = {};
  values.forEach((row, i) => {

    if(row[0] == '')
      return;
    
    data[row[1]] = {
      'type': row[0],
      'date_emission': row[2] == 'O' ? true : false,
      'description': row[3]
    };
  })
  sheet.appendRow(['', 'JSON: ', JSON.stringify(data)]);
}
```

:::info
Le script complet ainsi que le tableur sont disponible en Annexe.
:::

## Structure des fichiers de specs

```json
Section6.json
[
    {
        "type": "Justificatif de domicile",
        "code": "00",
        "date_emission": true,
        "description": "Document émis spécifiquement pour servir de justificatif de domicile."
    }
]
```

```json
Section7.json
[
    {
        "id": "01",
        "nom": "Identifiant unique du document.",
        "taille_min": 0,
        "taille_max": -1,
        "type": "Alphanumérique",
        "description": "Cet identifiant permet en fonction de l’émetteur (si celui-ci fournit le service) de récupérer le document correspondant. Cette donnée est encodée en utilisant uniquement des lettres majuscules non accentuées [A-Z] et des chiffres [0-9].",
        "conditions": "MAJ+CHIFFRES"
    }
]
```

```json
Section8.json
{
    "<S6>": {
        "obligatoire": ["<S7>", "20","21","22","23","24","25","26"],
        "facultatif": ["<S7>", "1K","1J","1I","1G"],
        "conditions": [
            [ //A OU B
              ["<S7>", "DK ", "DL"],
              ["DM", "DN", "DO", "DP"]
            ],
            [ // C OU D
              ["DQ", "DR","DS"],
              ["DT"]
            ]
        ]
    }
}
```
