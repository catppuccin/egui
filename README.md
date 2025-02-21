<h3 align="center">
	<img src="https://raw.githubusercontent.com/catppuccin/catppuccin/main/assets/logos/exports/1544x1544_circle.png" width="100" alt="Logo"/><br/>
	<img src="https://raw.githubusercontent.com/catppuccin/catppuccin/main/assets/misc/transparent.png" height="30" width="0px"/>
	Catppuccin for <a href="https://github.com/emilk/egui">egui</a>
	<img src="https://raw.githubusercontent.com/catppuccin/catppuccin/main/assets/misc/transparent.png" height="30" width="0px"/>
</h3>

<p align="center">
	<a href="https://github.com/catppuccin/egui/stargazers"><img src="https://img.shields.io/github/stars/catppuccin/egui?colorA=363a4f&colorB=b7bdf8&style=for-the-badge"></a>
	<a href="https://github.com/catppuccin/egui/issues"><img src="https://img.shields.io/github/issues/catppuccin/egui?colorA=363a4f&colorB=f5a97f&style=for-the-badge"></a>
	<a href="https://github.com/catppuccin/egui/contributors"><img src="https://img.shields.io/github/contributors/catppuccin/egui?colorA=363a4f&colorB=a6da95&style=for-the-badge"></a>
</p>

<p align="center">
	<a href="https://github.com/catppuccin/egui/actions/workflows/ci.yml"><img src="https://img.shields.io/github/actions/workflow/status/catppuccin/egui/ci.yml?colorA=363a4f&colorB=b7bdf8&style=for-the-badge"></a>
	<a href="https://crates.io/crates/catppuccin-egui"><img src="https://img.shields.io/crates/v/catppuccin-egui?colorA=363a4f&colorB=f5a97f&style=for-the-badge"></a>
	<a href="https://docs.rs/catppuccin-egui"><img src="https://img.shields.io/docsrs/catppuccin-egui?colorA=363a4f&colorB=a6da95&style=for-the-badge"></a>
</p>

<p align="center">
	<img src="https://raw.githubusercontent.com/catppuccin/egui/main/assets/previews/preview.webp"/>
</p>

## Previews

<details>
<summary>🌻 Latte</summary>
<img src="https://raw.githubusercontent.com/catppuccin/egui/main/assets/previews/latte.png"/>
</details>
<details>
<summary>🪴 Frappé</summary>
<img src="https://raw.githubusercontent.com/catppuccin/egui/main/assets/previews/frappe.png"/>
</details>
<details>
<summary>🌺 Macchiato</summary>
<img src="https://raw.githubusercontent.com/catppuccin/egui/main/assets/previews/macchiato.png"/>
</details>
<details>
<summary>🌿 Mocha</summary>
<img src="https://raw.githubusercontent.com/catppuccin/egui/main/assets/previews/mocha.png"/>
</details>

## Usage

catppuccin-egui uses Cargo features to add support for new egui versions without breaking backwards compatibility.

Add the crate to your `Cargo.toml`:

<!-- x-release-please-start-version -->

```toml
[dependencies]
catppuccin-egui = { version = "5.4.0", default-features = false, features = ["egui29"] }
```

<!-- x-release-please-end -->

To use a theme, call the `set_theme` function with a theme and the egui context:

```rust
catppuccin_egui::set_theme(&ctx, catppuccin_egui::MOCHA);
```

To run the example app, run `cargo run -p todo`.

See the full documentation at https://docs.rs/catppuccin-egui.

[Whiskers](https://github.com/catppuccin/toolbox/tree/main/whiskers) is required as a build-time development dependency.

## 💝 Thanks to

- [Stonks3141](https://github.com/Stonks3141)

&nbsp;

<p align="center">
	<img src="https://raw.githubusercontent.com/catppuccin/catppuccin/main/assets/footers/gray0_ctp_on_line.svg?sanitize=true" />
</p>

<p align="center">
	Copyright &copy; 2023-present <a href="https://github.com/catppuccin" target="_blank">Catppuccin Org</a>
</p>

<p align="center">
	<a href="https://github.com/catppuccin/egui/blob/main/LICENSE"><img src="https://img.shields.io/static/v1.svg?style=for-the-badge&label=License&message=MIT&logoColor=d9e0ee&colorA=363a4f&colorB=b7bdf8"/></a>
</p>
