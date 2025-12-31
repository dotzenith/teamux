<h1 align="center"> ━━━━  ❖  ━━━━ </h1>

<!-- BADGES -->
<div align="center">
   <p></p>

   <img src="https://img.shields.io/github/stars/dotzenith/teamux?color=F8BD96&labelColor=302D41&style=for-the-badge">
   <img src="https://img.shields.io/github/commit-activity/y/dotzenith/teamux?color=96CDFB&labelColor=302D41&style=for-the-badge&label=COMMITS"/>
   <br>
</div>

<p/>

---

## ❖ teamux

`teamux` makes joining and creating tmux sessions as easy as sipping tea.
It lets you fuzzy match on tmux sessions that already exist, and it'll create sessions that don't

---

## ❖ Requirements

- Ensure [tmux](https://github.com/tmux/tmux) and [fzf](https://github.com/junegunn/fzf) are installed

---

## ❖ Installation

#### Shell
```sh
curl --proto '=https' --tlsv1.2 -LsSf https://github.com/dotzenith/teamux/releases/latest/download/teamux-installer.sh | sh
```

#### Brew
```sh
brew install dotzenith/tap/teamux
```

#### Cargo
```sh
cargo install teamux
```

#### Binaries
Pre-Compiled binaries for linux and mac are available in [Releases](https://github.com/dotzenith/teamux/releases)

#### Source
- First, install [rust](https://rustup.rs/)
```sh
git clone https://github.com/dotzenith/teamux.git
cd teamux
cargo build --release
./target/release/mux
```

---

## ❖ Usage

```
Create and Join tmux sessions with ease

Usage: mux [NAME]

Arguments:
  [NAME]  Name of the session mux will attach to or create

Options:
  -h, --help     Print help
  -V, --version  Print version
```

### Join a session, or create one if it doesn't exist

```sh
mux session-name
```

### Use `fzf` to select an existing session

```sh
mux
```
---

## ❖ What's New?

1.0.0 - Initial Release

---

<div align="center">

   <img src="https://img.shields.io/static/v1.svg?label=License&message=MIT&color=F5E0DC&labelColor=302D41&style=for-the-badge">

</div>
