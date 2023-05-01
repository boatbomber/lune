<!-- @generated with lune-cli -->

# Uncategorized

All globals that are not available under a specific scope.

These are to be used directly without indexing a global table first.

---

## Functions

### error

```lua
function error(message: T, level: number?)
```

Throws an error and prints a formatted version of it with a leading `[ERROR]` tag.

### print

```lua
function print(T...)
```

Prints given value(s) to stdout.

This will format and prettify values such as tables, numbers, booleans, and more.

### printinfo

```lua
function printinfo(T...)
```

Prints given value(s) to stdout with a leading `[INFO]` tag.

This will format and prettify values such as tables, numbers, booleans, and more.

### warn

```lua
function warn(T...)
```

Prints given value(s) to stdout with a leading `[WARN]` tag.

This will format and prettify values such as tables, numbers, booleans, and more.
