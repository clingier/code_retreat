# Python Scaffold for Code Retreat

## Option 1: use docker wrapper scripts

See `scripts/` folder.

## Install Python tools

The easiest option is to install the [Anaconda](https://www.continuum.io/downloads) distribution, which includes Python plus many popular libraries. Prefer using Python 3.x.

## Run the tests

From `src/`, run:

```Bash
py.test -v
```

## Run the app

```Bash
python my_app.py
```

## Hierarchy of directories

* `src`: the source code, including a module `my_module.py` and a script `my_app.py`.
* `src/test`: the tests
