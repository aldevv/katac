# Tutorial: write your own kata

The embedded templates (`katac init`) are a starting point — most of the value of `katac` is using it with your own katas. This page walks through scaffolding one from scratch, watching the test fail, implementing it, and watching it pass.

![custom kata tutorial](tutorial.gif)

## What you'll build

A trivial `Add` kata in Python: a function `add(a, b)` plus a one-line assertion test. We start with a stub that returns `0`, see the test fail, replace `return 0` with `return a + b`, and watch the test pass.

> The choice of Python is incidental. `katac run` shells out to `make run` — what's inside the kata folder is up to you.

## Walkthrough

### 1. Scaffold an empty kata

```bash
katac new Add
```

This creates `katas/Add/` with a placeholder `Makefile` (`run: echo TODO`). Nothing else.

### 2. Fill in the kata

Drop three files into `katas/Add/`:

```python
# katas/Add/add.py
def add(a, b):
    return 0  # TODO
```

```python
# katas/Add/test_add.py
from add import add
assert add(2, 3) == 5
print('PASS')
```

```make
# katas/Add/Makefile
run:
	python3 test_add.py
```

The `Makefile` is the only piece `katac` cares about. Its `run` target can be `pytest`, `go test`, `cargo test`, `bun test`, a shell script — whatever runs your kata.

### 3. Start today's session

```bash
katac Add
```

`katac` copies `katas/Add/` into the next `days/dayN/` folder (`days/day1/Add/` on the first run). From now on you work in `days/day1/Add/`, not in `katas/`. The template stays clean for tomorrow.

### 4. Run it — and watch it fail

```bash
katac run
```

```
> Running Add [1/1]
----------------------
Traceback (most recent call last):
  File ".../days/day1/Add/test_add.py", line 2, in <module>
    assert add(2, 3) == 5
AssertionError
make: *** [Makefile:2: run] Error 1
```

That's the kata: the test tells you what's expected, the failure tells you you're not there yet.

### 5. Implement it

Open `days/day1/Add/add.py` and replace the stub:

```python
def add(a, b):
    return a + b
```

### 6. Run it again — passing

```bash
katac run
```

```
> Running Add [1/1]
----------------------
PASS
```

Day 1 done. Tomorrow, `katac Add` again creates `days/day2/Add/` from the original template — you start over, fresh.

## Where to go next

- [usage.md](usage.md) — the full reference: `katac.toml` configuration, restricting the random pool, per-kata Makefile recipes.
- [contributing.md](contributing.md) — add a new embedded language template so `katac init` knows about it.
