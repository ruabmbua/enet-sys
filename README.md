Raw rust bindings to the [enet](http://enet.bespin.org/) C library.

# Dependencies

* C, or C cross compiler for your target platform

## Ubuntu

Dependencies for Ubuntu can be installed with

```bash
sudo apt update
sudo apt install build-essential clang cmake
```

## Archlinux 

```bash
pacman -S base-devel clang cmake
```

# Cloning
`enet-sys` uses git submodules, either clone it with the `--recursive` option, or run:
``` git
git submodule init
git submodule update
```
after cloning it.

**Note:** If you add enet-sys as a dependency to your `Cargo.toml`, cargo will do this for you automatically.
