# dprint-plugin-tailwindcss

A [dprint](https://dprint.dev) WASM plugin that sorts [Tailwind CSS](https://tailwindcss.com) v4+ class names in JSX/TSX/HTML files, following the [official Prettier plugin's sorting algorithm](https://tailwindcss.com/blog/automatic-class-sorting-with-prettier#how-classes-are-sorted).

## Styles & Constraints

- **Rust 2024 edition** — flat module files (`class_finder.rs`, not `class_finder/mod.rs`)
- **Lifetimes over ownership** — use `<'_>` borrows (`&Path`, `&str`, `&Configuration`) instead of `Arc`/`PathBuf`/`String` for function inputs
- **Fully-qualified errors** — write `anyhow::Result<…>` directly in return types, no `use anyhow::Result;` imports
- **Hard-coded separator** — `pub const SEPARATOR: &str = ":";` in `parsed_class.rs`, not configurable
- **Minimal dependencies** — prefer byte-scanning over pulling in full parser crates (HTML)
- **Zero warnings** — `cargo check` must produce no warnings
- **All tests pass** — `cargo test` green before any commit
- **Pure `Ordering` pipeline** — sorting uses `.then_with()` chains with `Ord`-deriving enums

## Architecture

```
src/
├── lib.rs                  # Module declarations, WASM gate, re-exports
├── wasm_plugin.rs          # SyncPluginHandler + generate_plugin_code!
├── configuration.rs        # Plugin config schema + ResolveConfiguration
├── parsed_class.rs         # Token → ParsedClass { variants, base, value, … } + SEPARATOR constant
├── sort_order.rs           # UtilityGroup enum (derives Ord) + base→group mapping
├── css_config.rs           # Scan user CSS for @utility, @custom-variant, breakpoints
├── class_finder.rs         # JSX/TSX via deno_ast + HTML via byte-scanning → ClassSpan[]
├── class_sorter.rs         # 6-rule comparator (pure Ordering pipeline) + sort_class_string()
└── format_text.rs          # Orchestrator: find→sort→reconstruct → anyhow::Result<Option<String>>

tests/
├── spec_test.rs            # dprint-development file_test_runner
└── specs/                  # 4 spec fixtures (11 test cases)
```

## Dependencies

`anyhow`, `dprint-core 0.67.4`, `dprint-core-macros 0.1.0`, `serde` (derive), `serde_json` (optional), `deno_ast 0.53.0` (view).

## Plugin Configuration

```jsonc
// in dprint.json
{
  "tailwindcss": {
    "cssFile": "src/app.css"  // optional — path to Tailwind v4 CSS for @utility discovery
  }
}
```

Configuration struct (`src/configuration.rs`):

```rust
#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Configuration {
    pub css_file: Option<String>, // e.g. "src/app.css"
}
```

File matching targets: `.tsx`, `.jsx`, `.html` (configured in `wasm_plugin.rs`).

## Sort Order Algorithm

The plugin sorts class names identically to the official [Tailwind CSS Prettier plugin](https://tailwindcss.com/blog/automatic-class-sorting-with-prettier#how-classes-are-sorted).

### ParsedClass

Each class name token is parsed into a `ParsedClass`. Given `"dark:[&:nth-child(3)]:hover:md:!-w-[300px]"`:

```rust
ParsedClass {
    variants: vec![
        Variant::State("dark"),
        Variant::ArbitrarySelector("[&:nth-child(3)]"),
        Variant::State("hover"),
        Variant::Screen("md"),
    ],
    importance: true,                     // "!"
    negative: true,                       // -w
    base: String::from("w"),              // utility root
    value: Some(String::from("[300px]")), // value portion
    arbitrary: true,                      // value uses [...]
    arbitrary_property: false,            // bare [property:value]
    is_known: true,                       // base maps to a UtilityGroup
}
```

Arbitrary value forms recognized:

| Form | Example | Base | Arbitrary |
|---|---|---|---|
| Arbitrary value | `w-[300px]` | `w` | true |
| Arbitrary property | `[color:red]` | (empty) | true (arbitrary_property) |
| Negative arbitrary | `-mt-[10px]` | `mt` | true |
| Important arbitrary | `!bg-[#bada55]` | `bg` | true |

### Comparator (6 rules, pure `std::cmp::Ordering` pipeline)

```rust
fn cmp(a: &ParsedClass, b: &ParsedClass, config: &CssConfig) -> Ordering {
    // Rule 1: Unknown first — unrecognized classes sort BEFORE all known Tailwind utilities
    a.is_known.cmp(&b.is_known)

        // Rule 2: Variant level (None < State < Screen)
        .then_with(|| a.variant_level().cmp(&b.variant_level()))

        // Rule 2b: Within screen, breakpoint order (sm < md < lg < xl < 2xl)
        .then_with(|| a.breakpoint_index(config).cmp(&b.breakpoint_index(config)))

        // Rule 3: Canonical utility group (UtilityGroup derives Ord)
        .then_with(|| a.utility_group().cmp(&b.utility_group()))

        // Rule 4: Override order — broader before specific (same base only)
        //         p-4 before pt-2; border before border-gray-300
        .then_with(|| a.specificity().cmp(&b.specificity()))

        // Rule 5: Stable — Equal falls through, stable sort preserves original order
}
```

### Sort Examples

**Example A — basic reorder:**
```
Before:  text-white px-4 sm:px-8 py-2 sm:py-3 bg-sky-700 hover:bg-sky-800
After:   bg-sky-700 px-4 py-2 text-white hover:bg-sky-800 sm:px-8 sm:py-3
```

**Example B — override (broader before specific):**
```
Before:  pt-2 p-4
After:   p-4 pt-2
```

**Example C — state variant grouping:**
```
Before:  hover:opacity-75 opacity-50 hover:scale-150 scale-125
After:   scale-125 opacity-50 hover:scale-150 hover:opacity-75
```

**Example D — responsive grouping (small → large):**
```
Before:  lg:grid-cols-4 grid sm:grid-cols-3 grid-cols-2
After:   grid grid-cols-2 sm:grid-cols-3 lg:grid-cols-4
```

**Example E — unknown classes to front:**
```
Before:  p-3 shadow-xl select2-dropdown
After:   select2-dropdown p-3 shadow-xl
```

**Example F — arbitrary values:**
```
Before:  [color:red] flex w-[300px] bg-[#bada55] p-[10px] -mt-[5px] hover:[mask-type:luminance]
After:   flex w-[300px] p-[10px] -mt-[5px] bg-[#bada55] [color:red] hover:[mask-type:luminance]
```

**Example G — same-base utilities preserve order:**
```
Before:  sm:text-lg md:text-xl
After:   sm:text-lg md:text-xl   (unchanged — same UtilityGroup::Text, stable sort preserves cascade)
```

### UtilityGroup Enum (`src/sort_order.rs`)

An `Ord`-deriving enum mapping every standard Tailwind v4 base utility to its position in the CSS output order. Custom utilities from `@utility` blocks map to `UtilityGroup::Custom`. Unrecognized classes map to `UtilityGroup::Unknown` and are sorted first (Rule 1).

Derive order (top → bottom) matches Tailwind v4 CSS output:

```
Container, Prose,                    // Base & Components
SrOnly, NotSrOnly, ...,              // Accessibility
Static, Fixed, Absolute, ...,        // Positioning
Inset, InsetX, InsetY, Top, ...,     // Inset
Z, Isolate, Isolation,               // Z-Index
Block, Inline, Flex, Grid, ...,      // Display
Overflow, OverflowX, ...,            // Overflow
FlexBasis, FlexDirection, ...,       // Flexbox
GridTemplate, GridCols, ...,         // Grid
Order,                               // Order
W, MinW, MaxW, H, MinH, ...,         // Sizing
P, Px, Py, Pt, Pr, ..., M, ...,      // Spacing
Font, Text, Tracking, ...,           // Typography
Bg, GradientFrom, ...,               // Backgrounds
Border, BorderX, ..., Rounded, ...,  // Borders
Opacity, Shadow, Blur, ...,          // Effects
Filter, Brightness, ...,             // Filters
TableLayout, BorderCollapse, ...,    // Tables
Transition, Animate, ...,            // Transitions
Transform, Scale, Rotate, ...,       // Transforms
Cursor, PointerEvents, ...,          // Interactivity
Fill, Stroke, StrokeWidth,           // SVG
Custom,                              // @utility from CSS config
Unknown,                             // Unrecognized (sorted first via Rule 1)
```

### Supporting Enums (also derive `Ord`)

```rust
#[derive(PartialEq, Eq, PartialOrd, Ord)]
enum VariantLevel { None, State, Screen }

#[derive(PartialEq, Eq, PartialOrd, Ord)]
enum SideOrder { Shorthand, Axis, Direction }
// Shorthand = p, m, border, rounded
// Axis      = px, py, mx, my, border-x, rounded-t
// Direction = pt, mt, border-t, rounded-tl
```

## CSS Config Parsing (`src/css_config.rs`)

Scans the user's Tailwind v4 CSS file (line-based, not full CSS AST) to extract:

| Pattern | Extracts | Purpose |
|---|---|---|
| `@import "tailwindcss"` | — | Confirm v4 project |
| `@theme { ... }` | `--breakpoint-sm`, `--breakpoint-md`, ... | Breakpoint sort order |
| `@utility <name> { ... }` | Custom utility name | Map to `UtilityGroup::Custom` |
| `@custom-variant <name> (...) { ... }` | Custom variant name | Recognized state variant |

## Class Finder (`src/class_finder.rs`)

Finds class-containing attributes in source files, returning byte-offset spans:

```rust
pub struct ClassSpan {
    pub start: usize,      // byte offset of class string start
    pub end: usize,        // byte offset of class string end
    pub classes: String,   // the raw class string content
}

pub fn find_classes(text: &str, extension: &str) -> anyhow::Result<Vec<ClassSpan>>;
```

| Extension | Parser | Target |
|---|---|---|
| `.tsx`, `.jsx` | `deno_ast` (SWC ECMA) | `JSXAttr` with name `className` or `class` |
| `.html` | byte-scanning | `class="…"` and `class='…'` attributes |

Handles:
- String literals: `className="flex p-4"` → extracts the full string content
- Template literals: `` className={`flex ${expr} p-4`} `` → extracts text segments + preserves expression spans

## Format Text (`src/format_text.rs`)

```rust
pub struct FormatTextOptions<'a> {
    pub path: &'a Path,
    pub extension: Option<&'a str>,
    pub text: &'a str,
    pub config: &'a Configuration,
}

pub fn format_text(options: FormatTextOptions<'_>) -> anyhow::Result<Option<String>>;
```

Flow:
1. Load CSS file (if `config.css_file` is set) → `parse_css_config()` → `CssConfig`
2. Detect file extension → `find_classes()` → `Vec<ClassSpan>`
3. For each span: `sort_class_string()` — compare before/after
4. 0 replacements → `Ok(None)` (no changes, dprint skips write)
5. Otherwise → apply replacements end→start (preserving byte offsets) → `Ok(Some(new_text))`

## Tests

`tests/spec_test.rs` uses `dprint_development::run_specs` (custom harness, `harness = false`). Each fixture is a single file with input `==` expected output.

| Spec file | Test cases |
|---|---|
| `JSX_Basic.txt` | no-change, basic reorder, override order, unknown front (4) |
| `Variants.txt` | state grouping, responsive breakpoint order, mixed plain/state/screen (3) |
| `HTML_Simple.txt` | html class sorting, html no change (2) |
| `Arbitrary.txt` | arbitrary values, important modifier (2) |

Total: 4 spec files, 11 test cases, plus 4 unit tests in `class_sorter.rs`.
