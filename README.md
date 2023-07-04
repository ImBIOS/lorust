# Lorust - API Documentation

Lorust is the Rust version of **Lodash**, which is a modern Javascript utilty library delivering modularity, performance & extras.

## Usage

Depend on `lorust` in `Cargo.toml`:

```toml
[dependencies]
lorust = "0.1.0"
```

## Functions Categorization

We follow the flat function structure of **Lodash**, but the functions can be categorized into following areas:

| Category     | Description                                                  |
|--------------|--------------------------------------------------------------|
| `array`      | Utility functions to deal with Arrays (Not yet support)      |
| `collection` | Utility functions to deal with Collections (Not yet support) |
| `date`       | Utility functions to deal with Dates (Not yet support)       |
| `function`   | Utility functions to deal with Functions (Not yet support)   |
| `lang`       | Utility functions to deal with Languages (Not yet support)   |
| `math`       | Utility functions to deal with Math (Not yet support)        |
| `number`     | Utility functions to deal with Numbers (Not yet support)     |
| `object`     | Utility functions to deal with Objects (On Progress) 🏗️🧱🛠️   |
| `seq`        | Utility functions to deal with Sequences (Not yet support)   |
| `string`     | Utility functions to deal with Strings (On Progress) 🏗️🧱🛠️   |
| `util`       | Utility functions to deal with Utilities (Not yet support)   |
| `properties` | Utility functions to deal with Properties (Not yet support)  |
| `methods`    | Utility functions to deal with Methods (Not yet support)     |

## Contributing

### Git Hooks

We use [.githooks](./githooks) to manage git hooks. To enable the git hooks, run:

```bash
git config --local core.hooksPath .githooks/
```
