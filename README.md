<p align="center">
<img src="./dino.png" height="120px" />
</p>

# TReq 

![](screenshot.png)

# Instalation

For <b>Debian</b> users or variants (<b>Ubuntu, Linux Mint, PopOS</b>) is possible to download TReq using the [.deb file](https://github.com/talis-fb/TReq/releases).

After download it by link [HERE](https://github.com/talis-fb/TReq/releases) you can just run the command below pointing to file.
```sh
$ sudo dpkg -i path_to_file.deb
```

<br>

For <b>Arch Linux</b> or <b>Manjaro</b> users is possible download TReq using [AUR](https://aur.archlinux.org/packages/treq/).
```sh
$ git clone https://aur.archlinux.org/treq.git
$ cd treq
$ makepkg -si
```
Or, if you use the [yay](https://github.com/Jguer/yay) just run...
```sh
$ yay -S treq
```

<br>

For <b>anyone else</b>, you can also download TReq by [Crates.io](https://crates.io/crates/treq) repository. Using this method requires to have [Rust](https://www.rust-lang.org/pt-BR/tools/install) installed.
```sh
cargo install treq
```

# Setup

First of all, after installed TReq is necessary setup the `EDITOR` env variable.
```sh
export EDITOR=vim # instead vim you can use 'code', 'nano', 'emacs', etc..
```

To don't run this command ever time before use TReq append it in setup of your.
```sh
echo 'export EDITOR=vim' >> ~/.bashrc # to bash users
echo 'export EDITOR=vim' >> ~/.zshrc # to zsh users
echo 'export EDITOR=vim' >> ~/.config/fish/config.fish # to fish users
```

It's great use some terminal editor, like Vim or Emacs to have a smooth experience. Since you'll can open them in same terminal session of TReq.

However, it's totally fine use some Graphical editor like VS Code or Atom. In these cases, when you hit the command to 'Edit' TReq will open it up and then you'll need to hit 'r' to reload the content of file in TReq ever time you make some change. Since, TReq gets the content of opened file with editor when the command to call it ends. 
