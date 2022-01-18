# 🏗 Builder

On utiliser un Builder pour composer notre structure **DocumentDecoded**, il est en charge des fonctions suivantes: 
- Headers
- FieldZone
- Validation de document (**Validator**)
- Validation de signature (**Signer**)

```rust

let (headers, headers_size) = Headers::new(str);

let document_raw = DocumentRaw::new(str, headers_size)?;

let mut document_decoded = DocumentDecodedBuilder::new()
                            .add_headers(headers)
                            .decode_message(&document_raw.message)?
                            .collect().to_error(ErrorKind::Decoder, String::from("Error decoding"))?;

let signing = Signing::verify_signature(&document_raw, &document_decoded)?;

let validity = Validity::validate(&mut document_decoded);

Ok(Document { 
    raw: document_raw,
    decoded: document_decoded,
    signing,
    validity
})

```