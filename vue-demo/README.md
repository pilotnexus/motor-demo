# vue-demo

This template should help get you started developing with Vue 3 in Vite using the PilotService.

## Quickstart

First, set the `url` and `wsUrl` in the serviceConfig file (starting with http or https for the url and ws:/ for wss:/ for the wsUrl)

If you are running this on a Raspberry Pi with PilotNode for example you can use this setup:
```
export const serviceConfigurations: ServiceConfiguration[] = [
  {
    name: 'default',
    type: 'graphql',
    url: 'http://localhost:8080/graphql',
    wsUrl: 'ws://localhost:8080/graphql',
  },
];
```

Then you can configure your subsciptions in the Vue compoents setup method.
You see an example for the motor-demo PLC project in the MachineState.vue file.

This is an example using a number (applies to i8, u8, i16, u16, i32, u32, i64 and u64 types):
```
    const state = useSubscribe<number>('state', -1, value => parseInt(value));
```
Here the value 'state' is subscribed and initialized with -1 (the value it has before an update was received from PilotNode).
We also pass a callback that transforms the value (which is string by default in GraphQL) to an integer.
'const state' is a Vue `Ref`.
In the example you will see how to use it do display the value, binding it to a Progress Bar (for the axis positions) and use it to disable or enable a button depending on the state value.

To trigger a bool variable, use `v-set="['start_demo', true]"' in a v-btn (this example uses Vuetify so therefore v-btn is used instead of button).


## Recommended IDE Setup

[VSCode](https://code.visualstudio.com/) + [Volar](https://marketplace.visualstudio.com/items?itemName=Vue.volar) (and disable Vetur) + [TypeScript Vue Plugin (Volar)](https://marketplace.visualstudio.com/items?itemName=Vue.vscode-typescript-vue-plugin).

## Type Support for `.vue` Imports in TS

TypeScript cannot handle type information for `.vue` imports by default, so we replace the `tsc` CLI with `vue-tsc` for type checking. In editors, we need [TypeScript Vue Plugin (Volar)](https://marketplace.visualstudio.com/items?itemName=Vue.vscode-typescript-vue-plugin) to make the TypeScript language service aware of `.vue` types.

If the standalone TypeScript plugin doesn't feel fast enough to you, Volar has also implemented a [Take Over Mode](https://github.com/johnsoncodehk/volar/discussions/471#discussioncomment-1361669) that is more performant. You can enable it by the following steps:

1. Disable the built-in TypeScript Extension
    1) Run `Extensions: Show Built-in Extensions` from VSCode's command palette
    2) Find `TypeScript and JavaScript Language Features`, right click and select `Disable (Workspace)`
2. Reload the VSCode window by running `Developer: Reload Window` from the command palette.

## Customize configuration

See [Vite Configuration Reference](https://vitejs.dev/config/).

## Project Setup

```sh
pnpm install
```

### Compile and Hot-Reload for Development

```sh
pnpm dev
```

### Type-Check, Compile and Minify for Production

```sh
pnpm build
```

### Run Unit Tests with [Vitest](https://vitest.dev/)

```sh
pnpm test:unit
```

### Run End-to-End Tests with [Playwright](https://playwright.dev)

```sh
# Install browsers for the first run
npx playwright install

# When testing on CI, must build the project first
pnpm build

# Runs the end-to-end tests
pnpm test:e2e
# Runs the tests only on Chromium
pnpm test:e2e --project=chromium
# Runs the tests of a specific file
pnpm test:e2e tests/example.spec.ts
# Runs the tests in debug mode
pnpm test:e2e --debug
```

### Lint with [ESLint](https://eslint.org/)

```sh
pnpm lint
```
