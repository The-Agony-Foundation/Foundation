# Development

Run the following command in the root of the project to start the Dioxus dev server:

```bash
dx serve --hot-reload --platform desktop
```

## Pre-Commit

In order to contribute, `pre-commit` must be installed to catch certain issues, which requires >= Python 3.8.

Install `pre-commit` with the `pip` command: `pip install pre-commit`.

Install the relevant hooks:

```console
pre-commit install
pre-commit install --hook-type commit-msg
```
