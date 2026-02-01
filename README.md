<!-- markdownlint-disable -->
<div align="center">
<h1>Octoapp</h1>

[![GitHub](https://img.shields.io/badge/github-%23121011.svg?style=for-the-badge&logo=github&logoColor=white)][github]
[![Crates.io Version](https://img.shields.io/crates/v/octoapp?style=for-the-badge)][crates-io]
[![Crates.io Downloads (recent)](https://img.shields.io/crates/dr/octoapp?style=for-the-badge)][crates-io]
[![GitHub Stars](https://img.shields.io/github/stars/42ByteLabs/octoapp?style=for-the-badge)][github]
[![GitHub Issues](https://img.shields.io/github/issues/42ByteLabs/octoapp?style=for-the-badge)][github-issues]
[![Licence](https://img.shields.io/github/license/Ileriayo/markdown-badges?style=for-the-badge)][license]

</div>
<!-- markdownlint-restore -->

## Overview

[Octoapp][crates-io] is a Rust library for building [GitHub Apps][docs-github-app].
It provides a simple interface for creating GitHub Apps and [handling webhook events][docs-github-webhooks].

## ✨ Features

- Focus on simplicity and ease of use.
- Built-in support for handling GitHub webhook events.
- Uses `octocrab` for interacting with the GitHub API.
- Support for webhook events.
- Supported Libraries and Framework 
  - [`rocket`](https://rocket.rs/): A web framework for Rust gear Rust that makes it simple to write fast, type-safe, secure web applications
  - [`hyoer`](https://hyper.rs/): Blazing fast HTTP server

## 🚀 Quick Start

Run the following command to add `octoapp` to your project:

```bash
cargo add octoapp
```

## 🏃 Getting Started

```rust
use anyhow::Result;
use octoapp::prelude::*;

#[tokio::main]
async fn main() -> Result<()> {
    // [optional] Load .env file if it exists
    // dotenvy::dotenv().ok();

    // Load the configuration (from environment variables)
    // Or, you can set the configuration manually
    let config = OctoAppConfig::init()
        .app_name("My App")
        .app_id(12345)
        .client_id("client_id")
        .client_secret("client_secret")
        .webhook_secret("webhook_secret")
        .build()
        .expect("Failed to build OctoAppConfig");

    println!("{}", config);

    // Create a new Octocrab instance
    let octocrab = config.octocrab();

    if let Ok(client) = octocrab {
        let repos = client.issues("42ByteLabs", "octoapp")
            .list()
            .creator("GeekMasher")
            .send()
            .await?;
    }

    Ok(())
}
```


### Hyper

Install the Hyper features by using:

```bash
cargo add octoapp -F hyper
```

Once installed, check out [the example here](./examples/hyper/) to see how to implement it. 

### Rocket

To install the Rocket features, use the following command:

```bash
cargo add octoapp -F rocket
```

Once installed, check out [the example here](./examples/rocket/) to see how to implement it. 

## 📚 Documentation

You can find the documentation for this project on [docs.rs][docs].

## ♥️  Maintainers / Contributors

<!-- ALL-CONTRIBUTORS-LIST:START - Do not remove or modify this section -->
<!-- prettier-ignore-start -->
<!-- markdownlint-disable -->

<!-- markdownlint-restore -->
<!-- prettier-ignore-end -->

<!-- ALL-CONTRIBUTORS-LIST:END -->

## 🦸 Support

Please create [GitHub Issues][github-issues] if there are bugs or feature requests.

This project uses [Semantic Versioning (v2)][semver] and with major releases, breaking changes will occur.

## 📓 License

This project is licensed under the terms of the MIT open source license.
Please refer to [MIT][license] for the full terms.

<!-- Resources -->

[license]: ./LICENSE
[crates-io]: https://crates.io/crates/octoapp
[docs]: https://docs.rs/geekorm/latest/octoapp
[semver]: https://semver.org/
[github]: https://github.com/42ByteLabs/octoapp
[github-issues]: https://github.com/42ByteLabs/octoapp/issues

[docs-github-app]: https://docs.github.com/en/developers/apps
[docs-github-webhooks]: https://docs.github.com/en/developers/webhooks-and-events/webhooks
