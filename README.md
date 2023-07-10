# Arcanist
Arcanist is a multi-language function CLI runner. It can target functions written in different languages and execute them within the CLI.

Current supported languages:
- Python (.py)

Coming next:
- Sh and Bash (.sh)
- Rust (.rs)
- JavaScript (.js)

## Usage
Create an `arcanist.[ext]` using a supported language, define a function, and call `arcanist FUNCTION_NAME`. The `arcanist` file should be in the current or child directories. Example `arcanist.py` file:

```python
def example():
    print("Executed with arcanist!")
```
To call `example` function, run `arcanist example`.

## Limitations
### Calling a function with arguments
Currently it is not possible to call a function with arguments.

### Multiple definitions of the same function across `arcanist.[ext]` files
If a function is defined multiple times the function will be called multiple times. This will be solved with the definition of `arcanist.[yml|yaml]` files.

## Installation
Refer for the `Installation` section of the release you want to install. To install the latest version, visit the [latest installation instructions](https://github.com/dloez/arcanist/INSTALL.md).

## Supported platforms
This is the current list of platforms which have pre-built binaries. If your platform is missing, open a new issue and/or visit the [installing from source section from the latest installation instructions](https://github.com/dloez/arcanist/INSTALL.md).

- Linux (x32-x64-aarch64)
- macOS (x64)
- Windows (x32-x64)
