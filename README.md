> ![IMPORTANT]
> Uses released UniFFI version `0.28.0`.
> Working bindings tests in Swift and Kotlin.

# Works

Simple demo of workspace setup with two UniFFI consuming crates:

- `one`
- `two`

```sh
$ cargo tree -i one --format "{lib}"
one
└── two
```

Crate `two` contains bindings tests (`uniffi::build_foreign_language_testcases`)

## Tests

```sh
cargo test
```

# Setup

## Swift

### Xcode

Install Xcode 15

## Kotlin

### JNA

Install JNA:

```sh
curl https://repo1.maven.org/maven2/net/java/dev/jna/jna/5.14.0/jna-5.14.0.jar --output jna-5.14.0.jar
```

### Direnv

Install direnv to automatically load ENV variables when standing in project root.

```sh
brew install direnv
```

Then, standing in project root, run:

```sh
direnv allow .
```

# Design

Crate `one` exports a Rust struct using `uniffi::Object` which is then "imported"
in crate `two`'s udl file like so:

```webudl
[ExternalInterfaceExport="one"]
typedef extern OneObject;
```
