# Sycamore Rocket Template

This is template for [Sycamore](https://github.com/sycamore-rs/sycamore) + [Rocket](https://rocket.rs) + [Trunk](https://trunkrs.dev/) on [gitpod.io](https://www.gitpod.io).

[![Open in Gitpod](https://gitpod.io/button/open-in-gitpod.svg)](https://gitpod.io/#https://github.com/sycamore-rs/sycamore-rocket)

## Usage

For a more thorough explanation of Trunk and its features, please head over to the [repository](https://github.com/thedodd/trunk).

### Running

Just click on the button above to start a workspace. Trunk will automatically be installed and a dev server will automatically be started.

If you are using this template on your local machine, use the following command to start the Rocket server.
```bash
cargo watch -x run # Or cargo run
```
And this command to automatically rebuild the client.
```bash
cd app && trunk watch
```

### Release

```bash
cargo build --release # Build the server in release mode
cd app && trunk build --release # Build the client in release mode
```

## Using this template

This template is intended to be used as a quick way to test out something using Sycamore without having to setup a local environnement.

If you do not intend to use gitpod.io, simply remove `.gitpod.yml` and `.gitignore.Dockerfile` from source control.
