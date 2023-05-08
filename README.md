<p align="center">
<img src="./dino.png" height="120px" />
</p>

# TReq

![GitHub Workflow Status](https://img.shields.io/github/actions/workflow/status/talis-fb/treq/cd.yml)
![GitHub repo size](https://img.shields.io/github/repo-size/talis-fb/treq)
![AUR version](https://img.shields.io/aur/version/treq-bin)

#### TReq is a extremely tiny and fast Http/API Rest Client in Terminal. It is inspired by software like [Insominia](https://insomnia.rest/download), [Postman](https://www.postman.com/) and [Neovim](https://github.com/neovim/neovim)
### 
#### It integrates with your favorite text editor, letting you edit the body of request as you prefer, and quickly get the response. 

<br />

![showcase](https://res.cloudinary.com/dfjn94vg8/image/upload/v1677806905/TReq/showcase2_i1punq.gif)

# Installation

### Debian, Ubuntu, Linux Mint, PopOS, Zorin
It is possible to download a prebuild `.deb` file [HERE](https://github.com/talis-fb/TReq/releases/latest). After downloading the `.deb` file, you can unpackage and install it with the command below.

```sh
$ sudo dpkg -i path_to_file.deb
```

### Arch Linux, Manjaro
TReq is available in [AUR](https://aur.archlinux.org/packages/treq-bin/) Repository.

```sh
$ git clone https://aur.archlinux.org/treq-bin.git
$ cd treq
$ makepkg -si
```

Or, if you use [yay](https://github.com/Jguer/yay) just run...

```sh
$ yay -S treq-bin
```
### Cargo

In any other environment, it is possible to download TReq with the [Crates.io](https://crates.io/crates/treq) repository. Using this method only requires having [Rust](https://www.rust-lang.org/pt-BR/tools/install) installed.

After Rust is installed, run this command to download and compile Treq:

```sh
cargo install treq
```

This can to take a while, due to needing to compile the project and all of its dependencies.


# Setup external editor

TReq uses the`EDITOR` environment variable to open an external text editor when editing the Request's body or header.

TReq doesn't have a built in way to edit a Body of Requests. It delegates this work to the text editor of your choice. It is recommend you use a terminal text editor like 'vim', 'neovim', 'nano', or 'emacs'.

You can also set `TREQ_EDITOR` environment variable. If you want to use a specific editor with TReq. Without overwrite  `EDITOR` value.

You can check the [wiki page for a complete guide of editor setup](https://github.com/talis-fb/TReq/wiki/Editor)

# Usage
For a complete and simple guide how to use TReq: https://github.com/talis-fb/TReq/wiki/Usage

# Using variables inside Request

It's possible to use variables inside body or header values of a request. You can [check Wiki page for a complete guide of variables usage in TReq](https://github.com/talis-fb/TReq/wiki/Variables)

![showcase-variables](https://res.cloudinary.com/dfjn94vg8/image/upload/v1683563477/TReq/template_1_cuhrp2.gif)