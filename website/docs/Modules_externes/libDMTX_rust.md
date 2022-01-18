# 🧰 Utilisation de libDMTX en rust

## 🔨 Librairie
Pour pouvoir utiliser la librairie: 
```rust
mod dmtx;

//Pour que le compileur ajoute -ldmtx
#[link(name = "dmtx")] 
extern {}
```

On obtient des signatures rust de cette forme:

```rust
pub unsafe fn dmtxImageCreate(pxl: *mut std::os::raw::c_uchar, width: std::os::raw::c_int, height: std::os::raw::c_int, pack: std::os::raw::c_int) -> *mut DmtxImage
```


## 🌮 Wrapper
Attention, les signatures et les types sont au format c (avec pointeurs...)

On devrait donc faire une "safe interface", suivant les bonnes pratiques rust.

Par simplicité, on a choisit d'utiliser libDMTX de façon brut, en gardant tout le code "unsafe" dans une seule fonction. (**/src/libdmtx/mod.rs**)

## 🔧 Utilisation
```rust
pub fn dmtx_read(pxl: &[u8], width: u32, height: u32, pack: DmtxPackOrder) -> Vec<u8>
``` 

Notre interface rust pour libdmtx prends donc en entrée un vecteur plat de pixels, la largeur et hauteur de l'image, ainsi que son packOrder. 

On retourne un vecteur rust contenant les données du Datamatrix , ou un vecteur vide en cas d'abscence de matrice (on donne 1s a libdmtx pour trouver la matrice)

