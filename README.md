# Arcanist
Arcanist is a multi-language function CLI runner. It can target functions written in different languages and execute them within the CLI.

Current supported languages:
- Python (.py)
- Sh and Bash (.sh)

Coming next:
- JavaScript (.js)
- Rust (.rs)

## Usage
Create an `arcanist.[ext]` using a supported language, define a function, and call `arcanist FUNCTION_NAME`. The `arcanist` file should be in the current or child directories. Example `arcanist.py` file:

```python
def example():
    print("Executed with arcanist!")
```
To call `example` function, run `arcanist example`.

### Call function with arguments
You can pass arguments to a function by specifying as much possitional arguments as required after the function name. For example, to call a python function named `example` and pass `1, 2, 3` as the function arguments, run `arcanist example 1 2 3`. All arguments passed will be strings. Full example:

```python
def example(*args):
    print(args)

arcanist example 1 2 3
```

## Limitations
### Multiple definitions of the same function across `arcanist.[ext]` files
If a function is defined multiple times across different `arcanist.[ext]` files, the function will be called multiple times. This will be solved with the definition of `arcanist.[yml|yaml]` files.

### Arguments types
All arguments passed to a function will be considered strings. There are currently no mechanisms to define each argument type.

## Installation
Refer for the `Installation` section of the release you want to install. To install the latest version, visit the [latest installation instructions from the latest release](https://github.com/dloez/arcanist/releases/latest).

## Supported platforms
This is the current list of platforms which have pre-built binaries. If your platform is missing, open a new issue and/or visit the [installing from source section from the latest installation instructions](https://github.com/dloez/arcanist/releases/latest).

- Linux (x32-x64-aarch64)
- macOS (x64)
- Windows (x32-x64)
