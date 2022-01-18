# 📚 Librairie

Notre librairie est disponible de deux façons: 

## ❤ Rust
Via **libr2ddoc.rlib**

```rust
pub fn data_to_document(data: &[u8]) -> Result<Document, ErrorDocument>
pub fn data_to_json(data: &[u8]) -> String
pub fn image_to_document(pxls: &[u8], width: u32, height: u32, format: PackOrder) -> Result<Document, ErrorDocument>
pub fn image_to_json(pxls: &[u8], width: u32, height: u32, format: PackOrder) -> String
```

## 💔 C
Via **libr2ddoc.so**

```c
//libr2ddoc.h
char *lib2ddoc_data_to_json(unsigned char *data, int length);
char *lib2ddoc_image_to_json(unsigned char *pxls, int width, int height, int PackOrder);
void lib2ddoc_free_json(char *json);
```

:::tip
Pour faire créer une librairie **.so** a cargo, il modifier **Cargo.toml**
```
[lib]
name = "r2ddoc"
crate-type = ["cdylib", "rlib"]
```
:::


:::info
La librairie s'appelle r2ddoc car on ne peut pas faire une librairie qui commence par un chiffre 🤯
:::