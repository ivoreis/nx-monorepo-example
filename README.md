# NxMonorepoExample

<a alt="Nx logo" href="https://nx.dev" target="_blank" rel="noreferrer"><img src="https://raw.githubusercontent.com/nrwl/nx/master/images/nx-logo.png" width="45"></a>

✨ Your new, shiny [Nx workspace](https://nx.dev) is ready ✨.

Run `nx graph` to visually explore what got created. Now, let's get you up to speed!

## Project

This monorepo currently has 3 projects:

- `cancellations-api`: Rust API powered by [`axum`](https://docs.rs/axum/latest/axum/)
- `cancellations-ui`: [React](https://react.dev/) frontend. Testing is covered by [`vitest`](https://vitest.dev/). Exposes and loads a microfrontend component using [module-federation](https://module-federation.io/)
- `cancellations-e2e`: End to end testing powered by [`playwright`](https://playwright.dev/)

## Install

Ensure that you have installed:

- `pnpm`: https://pnpm.io/installation
- `rustup`: https://www.rust-lang.org/tools/install

To install the project dependencies run:

```sh
pnpm install
```

## Development

This project also includes a docker-compose file to allow seemless and cross-environments build. You have two ways to run these projects locally:

Natively:

```js
    nx run cancellations-api:run // run the API
    nx run cancellations-ui:dev // run the UI
```

Docker compose:

```js
    docker compose up
```

By default, the UI runs on `http://localhost:4200` and the API runs on `http://localhost:3000`.

## Run tasks

To run tasks with Nx use:

```sh
nx <target> <project-name>
```

For example:

```sh
nx build myproject
```

These targets are either [inferred automatically](https://nx.dev/concepts/inferred-tasks) or defined in the `project.json` or `package.json` files.

[More about running tasks in the docs &raquo;](https://nx.dev/features/run-tasks)

## Add new projects

While you could add new projects to your workspace manually, you might want to leverage [Nx plugins](https://nx.dev/concepts/nx-plugins) and their [code generation](https://nx.dev/features/generate-code) feature.

To install a new plugin you can use the `nx add` command. Here's an example of adding the React plugin:

```sh
nx add @nx/react
```

Use the plugin's generator to create new projects. For example, to create a new React app or library:

```sh
# Generate an app
nx g @nx/react:app demo

# Generate a library
nx g @nx/react:lib some-lib
```

You can use `nx list` to get a list of installed plugins. Then, run `nx list <plugin-name>` to learn about more specific capabilities of a particular plugin. Alternatively, [install Nx Console](https://nx.dev/getting-started/editor-setup) to browse plugins and generators in your IDE.

[Learn more about Nx plugins &raquo;](https://nx.dev/concepts/nx-plugins) | [Browse the plugin registry &raquo;](https://nx.dev/plugin-registry)

[Learn more about Nx on CI](https://nx.dev/ci/intro/ci-with-nx#ready-get-started-with-your-provider)

## Install Nx Console

Nx Console is an editor extension that enriches your developer experience. It lets you run tasks, generate code, and improves code autocompletion in your IDE. It is available for VSCode and IntelliJ.

[Install Nx Console &raquo;](https://nx.dev/getting-started/editor-setup)

## Useful links

Learn more:

- [Learn about Nx on CI](https://nx.dev/ci/intro/ci-with-nx)
- [Releasing Packages with Nx release](https://nx.dev/features/manage-releases)
- [What are Nx plugins?](https://nx.dev/concepts/nx-plugins)

And join the Nx community:

- [Discord](https://go.nx.dev/community)
- [Follow us on X](https://twitter.com/nxdevtools) or [LinkedIn](https://www.linkedin.com/company/nrwl)
- [Our Youtube channel](https://www.youtube.com/@nxdevtools)
- [Our blog](https://nx.dev/blog)
