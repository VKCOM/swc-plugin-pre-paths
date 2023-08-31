# swc-plugin-pre-paths

A [SWC](https://swc.rs) plugin to use [path mapping](https://www.typescriptlang.org/docs/handbook/module-resolution.html#path-mapping).
This may be needed in other plugins.

## Installation

**npm:**

```sh
npm i -D swc-plugin-pre-paths
```

**yarn:**

```sh
yarn add -D swc-plugin-pre-paths
```

## Usage

Via `.swcrc`

```json
{
  "jsc": {
    "experimental": {
      "plugins": [
        [
          "swc-plugin-pre-paths",
          {
            "baseUrl": ".",
            "paths": {
              "app/*": ["./src/app/*"],
              "config/*": ["./src/app/_config/*"],
              "environment/*": ["./src/environments/*"],
              "shared/*": ["./src/app/_shared/*"],
              "helpers/*": ["./src/helpers/*"],
              "tests/*": ["./src/tests/*"]
            }
          }
        ]
      ]
    }
  }
}
```

## Limits

This don't support multiple fall back locations.

```jsonc
{
  "paths": {
    "*": ["*", "generated/*"] // ⛔ don't support
  }
}
```
