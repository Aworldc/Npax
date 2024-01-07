<div align = center>
    <picture>
        <source
            srcset="dark.png"
            width="200"
            media="(prefers-color-scheme: dark)"
        >
        <img width="200" src="light.png">
    </picture>

An npm client written in rust, with an emphasis on scripting and offline usability

![GitHub top language](https://img.shields.io/github/languages/top/Aworldc/npax)
![GitHub commit activity (branch)](https://img.shields.io/github/commit-activity/w/Aworldc/npax)
</div>

## Summary
In most npm clients, like npm and pnpm,
dependencies are the core of it - packages have dependencies, and they have scripts.

Npax thinks differently.
In Npax, scripts are the core of everything. Individual scripts can depend on specific packages, and they can depend on other scripts.

## Features/Roadmap
`🟢 Done`
`🟡 In progress`
`🔴 Not started`

- 🟢 Project initialization
- 🟢 Scripts
- 🟡 Scripts that depend on packages
- 🟡 Scripts that depend on other scripts
- 🔴 Offline usage/cacheing
- 🔴 Export/compile npax.toml files to package.json files
- 🔴 Import/convert package.json files to npax.toml files
- 🔴 Scaffholding/degit-like command
- 🔴 Modules
- 🔴 Subpackages/monorepo system
- 🔴 Scripts that depend on scripts of subpackages
- 🔴 Concurrency/threading
- 🔴 Global dependencies/npx-like command
- 🔴 Node version management
- 🔴 Python/Pypi support
- 🔴 Python version management

## Usage
The heart of npax is the npax.toml file, where scripts are stored.

TODO: write more here
