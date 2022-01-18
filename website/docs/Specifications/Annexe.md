# 🔓 Zone Annexe

Par besoin de faire varier certains éléments du code 2D-DOC, la version `04` intègre une **zone annexe** à la fin du code après la zone de signature. Le format de l’annexe est la même que la zone de message, cependant puisque celle-ci n’est pas prise en compte dans la signature ses informations **ne peuvent pas être vérifiées**. Ainsi il faut prendre des précautions avec ce message annexe car les données pourraient être frauduleuses. 

Un exemple d’utilité de l’annexe est dans le cas où le 2D-DOC contient les caractéristiques d’un produit et son numéro de série. On souhaite créer un 2D-DOC pour chaque produit mais cela est coûteux si l’on doit intégrer à chaque fois le numéro de série alors que les caractéristiques du produit restent les mêmes. 

Ainsi, en ajoutant le numéro de série dans l’annexe on peut le faire varier pour chaque élément produit. 
Pour insérer un bloc de données annexe en format C40 il faut insérer le séparateur `<GS>` à la fin de la signature mais pas en format binaire.


