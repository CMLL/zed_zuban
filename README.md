# Zuban Extension for Zed

Adds the option to use [Zuban](https://zubanls.com/) as a Language Server in Zed. Zuban is a static type checker, and server made by David Halter.

## Setup

### Install Zuban

`Zuban` must be installed and discoverable in `$PATH` in order for the extension to find it. There are several ways to acomplish this.

```sh
uv tool install zuban
```

```sh
pipx install zuban
```

### Install the extension

Search for the extension `Zuban` in `Zed` catalog and click install.


### Enable

Modify your local editor configuration to load `zuban` as the language server for `Python`. Is not needed but is recommended to disabled other language servers like `pyright` or `pylsp` to avoid duplicated entries.

```json
{
  "languages": {
    "Python": {
      "language_servers": ["zuban", "!pyright", "!pylsp"]
    }
  }
}
```

### Configuration

`Zuban` offers a configuration via `pyproject.toml` or `mypy.ini`. To use with virtual environments, set the `python_executable` option in the configuration.

```toml
[tool.zuban]
python_executable=".venv/bin/python"
strict = true
disallow_untyped_defs = true
warn_unreachable = true
```

Learn more about at:
- [Zuban](https://docs.zubanls.com/en/latest/usage.html#configuration)
- [MyPy](https://mypy.readthedocs.io/en/stable/command_line.html#config-file)
