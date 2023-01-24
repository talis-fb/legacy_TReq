<p align="center">
<img src="./dino.png" height="120px" />
</p>

# Treq

![GitHub Workflow Status](https://img.shields.io/github/actions/workflow/status/talis-fb/treq/cd.yml)
![GitHub repo size](https://img.shields.io/github/repo-size/talis-fb/treq)
![AUR version](https://img.shields.io/aur/version/treq-bin)

#### TReq is a extremely tiny and fast Http/API Rest Client in Terminal. It is inspired by software like [Insominia](https://insomnia.rest/download), [Postman](https://www.postman.com/) and [Neovim](https://github.com/neovim/neovim)
### 
#### It integrates with your favorite text editor, letting you edit the body of request as you prefer, and quickly get the response. 

<br />

![](showcase.gif)

# Installation

### Recommended

In any Linux environment, it is possible to download TReq with the [Crates.io](https://crates.io/crates/treq) repository. Using this method requires having [Rust](https://www.rust-lang.org/pt-BR/tools/install) installed. 

After Rust is installed, run this command to download and compile Treq:

```sh
cargo install treq
```

This can to take a while, due to needing to compile the project and all of its dependencies.

### Debian, Ubuntu, Linux Mint, PopOS, Zorin
It is possible to download a prebuild .deb file [HERE](https://github.com/talis-fb/TReq/releases). After downloading the .deb file, you can unpackage it with the command below.

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

# Setup

To make use of the full set of feature provided by TReq, the`EDITOR` environment variable needs to be set. TReq will use this variable when opening the text editor.

TReq doesn't have a built in way to edit a Body of Requests. It delegates this work to the text editor of your choice. It is recommend you use a terminal text editor like 'vim', 'neovim', 'nano', or even 'emacs' to edit these.

If you do not already have an EDITOR set (to check, type `echo $EDITOR`, into your terminal. The output is the editor that will be used by TReq), you can set up the EDITOR environment variable in a terminal session using the command below. However, after you close the terminal session (window) this configuration will be dropped.
```sh
# instead vim you can use 'code', 'nano', 'emacs', etc..
export EDITOR=vim 
```

To set this up permanent it is necessary to edit your shell configuration. For example:
```sh
# for bash users
echo 'export EDITOR=vim' >> ~/.bashrc 

# for zsh users
echo 'export EDITOR=vim' >> ~/.zshrc

# for fish users
echo 'export EDITOR=vim' >> ~/.config/fish/config.fish 
```

However, it is quite commmon for this editor to already be set, but beware of non-terminal editors being set (like `writer` from LibreOffice Writer), because they will not work smoothly with TReq. It is recommended to use a terminal text editor like _vi_ or _nano_.

However, TReq will work fine with some graphical editors, like VS Code or Atom. In these cases, when you hit the command to 'edit body', TReq will open the graphical editor up, and you will need to hit reload the file with 'r' in TReq ever time you make some change.
