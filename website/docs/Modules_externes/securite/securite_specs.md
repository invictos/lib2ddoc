# 📝 Spécifications

## ⚙ Principe
Les 2DDoc sont signés pour permettre leur vérification.

La liste des TLS (Trust Service Provider) : Autoritées ayant un certificat RACINE (*cert_racine*) (identifié par leur **Identifiant de l’Autorité de Certification** ex: *FR01*). Elles sont listées dans un fichier XML sur le site de l'ANTS (ex *20210927-TLS_2.xml*)
- Pour chaque autorité, le fichier XML contient un URL pour récupérer le certificat (*cert_fils*) ayant signé un 2DDoc, identifié par un **Identifiant du certificat** ex: *AV01*
- On peut vérifier l'authenticité de *cert_fils* à l'aide de *cert_racine*. Il est donc nécessaire d'avoir les *cert_racine* dans le binaire du programme
- Il existe aussi pour chaque autorité une liste de *cert_fils* révoqués

## 🧰 Méthode
- Extraire la signature
- Extraire l’en-tête
- Récupérer l'**Identifiant de l’Autorité de Certification** (*AC*)
    - Récupérer le *cert_racine* correspondant à *AC*
    - Récupérer l'URL pour trouver le *cert_fils*
- Récupérer l'**Identifiant du certificat** (*AC_NUM*)
    - Récupérer le *cert_fils* correspondant à *AC_NUM*
    - Valider *cert_fils* avec *cert_racine*
    - Vérifier si *cert_fils* est revoqué
- Vérifier la signature avec *cert_fils*

On obtient le certificat *cert_fils*, et on a vérifier que celui-ci est bien authentique.

## 🔐 Vérifier la signature

- Récupérer la signature du message
- Récuperer la clée publique de *cert_fils* (ie. extraire du certificat)
- (Calculer un Digest (=hash) du message)
- A partir de la clée publique, de la signature ( et du digest OU message ), valider la signature

## 🔏 Format des données de certification
- **.cer .crt .der** => certificat DER au format binaire
- **.pem** => certificat DER encodé en Base64, encadré par les mentions "-----BEGIN CERTIFICATE-----" et "-----END CERTIFICATE-----"

:::tip

Pour manipuler les certificats "a la main", ou préparer les tests:

```sh
# Certificat X509: PEM -> DER
openssl x509 -in AV01.der -inform DER > AV01.pem

# SI DER: -inform DER
    # Certificat X509 PEM: Infos
    openssl x509 -text -noout

    # Certificat PEM -> Clée publique
    openssl x509 -pubkey -noout

```
:::