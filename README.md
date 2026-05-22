# dprint-plugin-tailwindcss

A [dprint](https://dprint.dev/) plugin that sorts [Tailwind CSS](https://tailwindcss.com/) v4+ class names in JSX/TSX/HTML files, following the [official class sorting algorithm](https://tailwindcss.com/blog/automatic-class-sorting-with-prettier#how-classes-are-sorted).

---

## Installation

Add the plugin via dprint:

```sh
dprint config add liuhq/dprint-plugin-tailwindcss
```

Or manually add the plugin URL to your dprint configuration's `plugins` array. Replace `x.x.x` with the desired version:

```
https://plugins.dprint.dev/liuhq/dprint-plugin-tailwindcss-x.x.x.wasm
```

### Usage with `typescript` plugin

If you are using the `typescript` plugin, ensure `tailwindcss` is listed **after** `typescript`. This ensures classname sorting runs after other code formatting.

Also, add the files you want to format to the `associations` of both `typescript` and `tailwindcss`.

```json
{
  "typescript": {
    "associations": ["**/*.tsx", "**/*.jsx"]
  },
  "tailwindcss": {
    "associations": ["**/*.tsx", "**/*.jsx", "**/*.html"]
  },
  "plugins": [
    "https://plugins.dprint.dev/typescript-x.x.x.wasm",
    "https://plugins.dprint.dev/liuhq/dprint-plugin-tailwindcss-x.x.x.wasm"
  ]
}
```

### Usage with `classname-wrap` plugin

If you use both `dprint-plugin-tailwindcss` and [`dprint-plugin-classname-wrap`](https://github.com/liuhq/dprint-plugin-classname-wrap), the sort plugin must be placed **before** the wrap plugin. This ensures class names are sorted before they are wrapped.

```json
{
  "tailwindcss": {
    "associations": ["**/*.tsx", "**/*.jsx", "**/*.html"]
  },
  "classnameWrap": {
    "associations": ["**/*.tsx", "**/*.jsx"]
  },
  "plugins": [
    "https://plugins.dprint.dev/liuhq/dprint-plugin-tailwindcss-x.x.x.wasm",
    "https://plugins.dprint.dev/liuhq/classname-wrap-x.x.x.wasm"
  ]
}
```

### All three together

When using `typescript`, `tailwindcss`, and `classname-wrap` together:

```json
{
  "typescript": { "associations": ["**/*.tsx", "**/*.jsx"] },
  "tailwindcss": { "associations": ["**/*.tsx", "**/*.jsx", "**/*.html"] },
  "classnameWrap": { "associations": ["**/*.tsx", "**/*.jsx"] },
  "plugins": [
    "https://plugins.dprint.dev/typescript-x.x.x.wasm",
    "https://plugins.dprint.dev/liuhq/dprint-plugin-tailwindcss-x.x.x.wasm",
    "https://plugins.dprint.dev/liuhq/classname-wrap-x.x.x.wasm"
  ]
}
```

## Configuration

| Option | Type | Description | Default |
|---|---|---|---|
| `cssFile` | `string?` | Path to Tailwind v4 CSS file for `@utility` discovery | `null` |

### Example

```jsonc
// dprint.json
{
  "tailwindcss": {
    "cssFile": "src/app.css"  // optional
  }
}
```

## Example

**Before:**

```jsx
<button className="text-white px-4 sm:px-8 py-2 sm:py-3 bg-sky-700 hover:bg-sky-800">
  Click me
</button>
```

**After:**

```jsx
<button className="bg-sky-700 px-4 py-2 text-white hover:bg-sky-800 sm:px-8 sm:py-3">
  Click me
</button>
```

## Build

Using [just](https://github.com/casey/just):

```sh
just build-release
```

Or with `cargo`:

```sh
cargo build --target wasm32-unknown-unknown --features "wasm" --release
```

## License

[MIT](./LICENSE)
