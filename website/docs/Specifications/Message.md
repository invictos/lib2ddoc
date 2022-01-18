# 📨 Zone de Message

La zone de message contient toutes les données et l’information encodée. Chaque donnée est  précédée d’un identifiant de donnée (**ID**). L’ID de la donnée est nécessaire pour préciser si la donnée est de longueur fixe, variable, borné inférieurement et/ou supérieurement.  

La zone de message diffère légèrement selon le format d’encodage du 2D-Doc (C40 ou Binaire) et le nombre de champs peut également varier en fonction des données facultatives.
On peut encoder différents types de données:
- Pour le format C40 cela comprend des données **textuelles, numériques ou des dates/heures**.
- Pour le format Binaire on peut encoder des **données binaires** ainsi que des références à des données externes au Code 2D-DOC. 

## 🔠 Zone de message C40

En C40, l’en-tête a une taille fixe de 26 caractères alphanumériques avant encodage. Un message est constitué d’une suite de blocs de données. Les blocs de données contiennent les éléments suivants: 
- un identifiant de donnée **ID** codé sur deux caractères
- la **donnée** elle-même encodée au format C40
- un caractère de fin de donnée appelé **séparateur** (`<GS>`) lorsqu’un bloc de données n’a pas une taille fixe

Finalement, le caractère `<RS>` indique la **fin** de la zone de message

![En-tête d’un code 2D-Doc](/img/images/message.PNG)

## 🔢 Zone de message Binaire

L’identifiant de donnée (**ID**) est codé sur un octet (La valeur `0xFF` n’est pas autorisée: elle est réservée pour indiquer le début de la signature) puis suivi d’un ou trois octets pour indiquer la taille de la donnée qui suit. Si la donnée est nulle alors l’octet prend la valeur de `0x00` 
