module.exports = {
  docs: [
    {
      type: 'category',
      label: '🏠 Projet',
      items: [
        'project/README',
        'project/FFI_out',
        'project/UML',
        'project/Ameliorations_possibles',
        'project/website',
        'project/CI_CD',
        'project/Demo',
      ],
    },   

    {
      type: 'category',
      label: ' 📜 Spécifications',
      items: [
        'Specifications/Introduction',
        'Specifications/Format_2DDoc',
        'Specifications/Entete',
        'Specifications/Message',
        'Specifications/Signature',
        'Specifications/Annexe',
        'Specifications/Types_de_documents',
        ,
      ],
    },   



    {
      type: 'category',
      label: ' ⌨ Implémentation',
      items: [
        {

          "📝 Spécifications": [
            'Implementation/Specifications/GDoc+GDoc->JSON',
            'Implementation/Specifications/SpecificationStore',
            'Implementation/Specifications/Fonctions_validations'
          ],
        },

        {
           "🛠 Décodeur" : [
            'Implementation/decoder/Entete',
            'Implementation/decoder/Message',
            'Implementation/decoder/Builder',
            'Implementation/decoder/Validator',
            'Implementation/decoder/Signer',
            'Implementation/decoder/Error',
          ],
        },


      ],
    },  





    {
      type: 'category',
      label: '🧪 Modules externes',
      items: [
        {
          '🔐 Sécurité': [
            'Modules_externes/securite/securite_specs',
            'Modules_externes/securite/securite_impl',
          ]
        },
        'Modules_externes/ffi',
        'Modules_externes/bindgen',
        'Modules_externes/libDMTX_rust',
        'Modules_externes/serialiser',
      ],
    },  


  ],
};

    





    




