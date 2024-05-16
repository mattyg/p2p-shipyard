# How to create a holochain runtime

A **holochain runtime** is an end-user application that is able to install and open holochain apps and web-apps. Examples of existing runtimes include the [launcher](https://github.com/holochain/launcher) and [moss](https://github.com/lightningrodlabs/we).

## Scaffolding

In the repository where you want to create your new holochain runtime, run this command:

```bash
nix run github:darksoil-studio/tauri-plugin-holochain#scaffold-holochain-runtime
```

And follow along its instructions and prompts.

2. Take a look into the repository structure that the scaffold command created, specially:

- `flake.nix`: with the `tauri-plugin-holochain` input and its `devShells`.
- `package.json`: added set up scripts and some `devDependencies`.
- `src-tauri`: here is where the code for the backend of the tauri app lives.
- `index.html`: main `index.html` file that will be displayed when the app is opened.
- `src`: this is where the code for the UI lives.
  - The scaffolded template contains a very bare bones vanilla JS app. Look in `src/main.ts` to see how the frontend for your runtime can connect to the `AdminWebsocket`.

That's it! We now have a working skeleton for a holochain runtime. 

## Development Environment

The `scaffold-holochain-apps` has added the necessary nix `devShells` to your `flake.nix` file so that you don't need to follow install anything to get the tauri or Android development environment.

> [!NOTE]
> Nix `devShells` are packages that describe development environments, with all their dependencies and environment variables, so that the developer does not need to configure manually their setup.

As usual, run this command to enter the development environment:

```bash
nix develop
```

This can take a while while it builds all the required dependencies.

Next, run these commands:

::: code-group
```bash [npm]
npm install
npm run tauri dev
```

```bash [yarn]
yarn install
yarn tauri dev
```

```bash [pnpm]
pnpm install
pnpm tauri dev
```
:::

This will start an instance of the app.

Under the hood, these commands are running tauri CLI commands. As such, we should get to know Tauri a bit better to be comfortable while developing the app. Go to [Getting to know Tauri](./getting-to-know-tauri.md) to familiarize yourself with it.
