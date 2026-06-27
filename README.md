# kyro
<p align="center">
  <img src="docs/assets/icon.svg" width="120" alt="kyro logo"><br/><br>
 <a href="https://github.com/externref/kyro/blob/main/LICENSE">
    <img src="https://img.shields.io/github/license/externref/kyro?style=flat-square&logo=github&logoColor=white&color=09090b&labelColor=27272a" alt="License">
  </a>
  <a href="https://github.com/externref/kyro/stargazers">
    <img src="https://img.shields.io/github/stars/externref/kyro?style=flat-square&logo=github&logoColor=white&color=09090b&labelColor=27272a" alt="GitHub stars">
  </a>
  <a href="https://github.com/externref/kyro/commits">
    <img src="https://img.shields.io/github/last-commit/externref/kyro?style=flat-square&logo=github&logoColor=white&color=09090b&labelColor=27272a" alt="GitHub last commit">
  </a>
</p>


Kyro is a lightweight language built in Rust, inspired by the [Lox language from Crafting Interpreters](https://craftinginterpreters.com/the-lox-language.html). It features a tree-walk interpreter architecture with modern enhancements such as a static resolution pass, compiled string interpolation, and a namespace-isolated standard library.

The language is designed for simplicity and extensibility, supporting object-oriented programming (OOP), structured exception handling, and first-class collection types like lists and dictionaries

Documentation & Reference: **[kyro.externref.dev](https://externref.github.io/kyro)**

### Hello, world!
```kyro
var io = use("std:io");

fn main(){
  io.println("hello, world!");
}

main()
```