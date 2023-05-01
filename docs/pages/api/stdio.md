<!-- @generated with lune-cli -->

# Stdio

Built-in standard input / output & utility functions

### Example usage

```lua
local stdio = require("@lune/stdio")

-- Prompting the user for basic input
local text: string = stdio.prompt("text", "Please write some text")
local confirmed: boolean = stdio.prompt("confirm", "Please confirm this action")

-- Writing directly to stdout or stderr, without the auto-formatting of print/warn/error
stdio.write("Hello, ")
stdio.write("World! ")
stdio.write("All on the same line")
stdio.ewrite("\nAnd some error text, too")
```

---

## Functions

### color

```lua
function stdio.color(color: StdioColor)
```

Return an ANSI string that can be used to modify the persistent output color.

Pass `"reset"` to get a string that can reset the persistent output color.

**Example usage:**

```lua
stdio.write(stdio.color("red"))
print("This text will be red")
stdio.write(stdio.color("reset"))
print("This text will be normal")
```

### ewrite

```lua
function stdio.ewrite(s: string)
```

Writes a string directly to stderr, without any newline.

### format

```lua
function stdio.format(_: ...any)
```

Formats arguments into a human-readable string with syntax highlighting for tables.

### prompt

```lua
function stdio.prompt(_: nil | nil | "text" | nil | "confirm" | nil | "select" | "multiselect", _: nil | nil | string? | nil | string | nil | string? | string?, _: nil | nil | string? | nil | boolean? | nil | { string } | { string })
```

Prompts for user input using the wanted kind of prompt:

* `"text"` - Prompts for a plain text string from the user
* `"confirm"` - Prompts the user to confirm with y / n (yes / no)
* `"select"` - Prompts the user to select *one* value from a list
* `"multiselect"` - Prompts the user to select *one or more* values from a list
* `nil` - Equivalent to `"text"` with no extra arguments

### style

```lua
function stdio.style(style: StdioStyle)
```

Return an ANSI string that can be used to modify the persistent output style.

Pass `"reset"` to get a string that can reset the persistent output style.

**Example usage:**

```lua
stdio.write(stdio.style("bold"))
print("This text will be bold")
stdio.write(stdio.style("reset"))
print("This text will be normal")
```

### write

```lua
function stdio.write(s: string)
```

Writes a string directly to stdout, without any newline.

---

## Types

### StdioColor

```lua
type StdioColor = "reset" | "black" | "red" | "green" | "yellow" | "blue" | "purple" | "cyan" | "white"
```

### StdioStyle

```lua
type StdioStyle = "reset" | "bold" | "dim"
```