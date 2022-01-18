# 🚀 FFI libDMTX

## 📝 libDMTX

Tout d'abord, pour avoir accès a la commande **dmtxread**, on utilise la commande suivante

```
 apt install dmtx-utils
```

Maintenant, on veut créer une interface en rust de la librairie libdmtx

```
apt install libdmtx
```

## 📙 Man
Le man de libDmtx nous donne le code suivant pour décoder une matrice:

```c
int main(int argc, char * argv[]) {
    img = dmtxImageCreate(pxl, width, height, DmtxPack24bppRGB);
    assert(img != NULL);

    dec = dmtxDecodeCreate(img, 1);
    assert(dec != NULL);

    reg = dmtxRegionFindNext(dec, NULL);
    if (reg != NULL) {
        msg = dmtxDecodeMatrixRegion(dec, reg, DmtxUndefined);
        if (msg != NULL) {
            fputs("output: \"", stdout);
            fwrite(msg -> output, sizeof(unsigned char), msg -> outputIdx stdout);
            fputs("\"\n", stdout);
            dmtxMessageDestroy( & msg);
        }
        dmtxRegionDestroy( & reg);
    }

    dmtxDecodeDestroy( & dec);
    dmtxImageDestroy( & img);
    free(pxl);

    exit(0);
}
```

:::tip

> dmtxImageCreate

Cette fonction prend comme 4e argument un packOrder, cela correspond à l'arrangement des pixels dans le vecteur plat pxl.

> dmtxDecodeCreate

Le 2e argument de cette fonction est un facteur d'agrandissement, inutile dans notre cas. 

> dmtxRegionFindNext

Le 2e argument de cette fonction est un timeout pour la recherche, via la structure DmtxTime

```c
typedef struct DmtxTime_struct {
   time_t          sec;
   unsigned long   usec;
} DmtxTime;
```
:::

## 🚀 Foreign Function Interface.
Une interface de fonction étrangère (FFI) est un mécanisme par lequel un programme écrit dans un langage de programmation peut appeler des routines ou utiliser des services écrits dans un autre langage.
>https://doc.rust-lang.org/nomicon/ffi.html

```rust
extern crate libc;
use libc::size_t;

#[link(name = "snappy")]
extern {
    fn snappy_max_compressed_length(source_length: size_t) -> size_t;
}

fn main() {
    let x = unsafe { snappy_max_compressed_length(100) };
    println!("max compressed length of a 100 byte buffer: {}", x);
}
```

![02](/img/asset/FFI.svg)


## 📌 Notre cas
On veut avoir une FFI sur libDMTX, on utilise bindgen pour avoir des "headers" de libDMTX depuis les headers **.h**, et c'est cargo qui va linker la librairie lors de la compilation !