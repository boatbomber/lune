<!-- @generated with lune-cli -->

# FS

Built-in library for filesystem access

### Example usage

```lua
local fs = require("@lune/fs")

-- Reading a file
local myTextFile: string = fs.readFile("myFileName.txt")

-- Reading entries (files & dirs) in a directory
for _, entryName in fs.readDir("myDirName") do
    if fs.isFile("myDirName/" .. entryName) then
        print("Found file " .. entryName)
    elseif if fs.isDir("myDirName/" .. entryName) then
        print("Found subdirectory " .. entryName)
    end
end
```

---

## Functions

### isDir

```lua
function fs.isDir(path: string)
```

Checks if a given path is a directory.

An error will be thrown in the following situations:

* The current process lacks permissions to read at `path`.
* Some other I/O error occurred.

### isFile

```lua
function fs.isFile(path: string)
```

Checks if a given path is a file.

An error will be thrown in the following situations:

* The current process lacks permissions to read at `path`.
* Some other I/O error occurred.

### move

```lua
function fs.move(from: string, to: string, overwriteOrOptions: (boolean | FsWriteOptions)?)
```

Moves a file or directory to a new path.

Throws an error if a file or directory already exists at the target path.
This can be bypassed by passing `true` as the third argument, or a dictionary of options.
Refer to the documentation for `FsWriteOptions` for specific option keys and their values.

An error will be thrown in the following situations:

* The current process lacks permissions to read at `from` or write at `to`.
* The new path exists on a different mount point.
* Some other I/O error occurred.

### readDir

```lua
function fs.readDir(path: string)
```

Reads entries in a directory at `path`.

An error will be thrown in the following situations:

* `path` does not point to an existing directory.
* The current process lacks permissions to read the contents of the directory.
* Some other I/O error occurred.

### readFile

```lua
function fs.readFile(path: string)
```

Reads a file at `path`.

An error will be thrown in the following situations:

* `path` does not point to an existing file.
* The current process lacks permissions to read the file.
* The contents of the file cannot be read as a UTF-8 string.
* Some other I/O error occurred.

### removeDir

```lua
function fs.removeDir(path: string)
```

Removes a directory and all of its contents.

An error will be thrown in the following situations:

* `path` is not an existing and empty directory.
* The current process lacks permissions to remove the directory.
* Some other I/O error occurred.

### removeFile

```lua
function fs.removeFile(path: string)
```

Removes a file.

An error will be thrown in the following situations:

* `path` does not point to an existing file.
* The current process lacks permissions to remove the file.
* Some other I/O error occurred.

### writeDir

```lua
function fs.writeDir(path: string)
```

Creates a directory and its parent directories if they are missing.

An error will be thrown in the following situations:

* `path` already points to an existing file or directory.
* The current process lacks permissions to create the directory or its missing parents.
* Some other I/O error occurred.

### writeFile

```lua
function fs.writeFile(path: string, contents: string)
```

Writes to a file at `path`.

An error will be thrown in the following situations:

* The file's parent directory does not exist.
* The current process lacks permissions to write to the file.
* Some other I/O error occurred.

---

## Types

### FsWriteOptions

```lua
type FsWriteOptions = {
	overwrite: boolean?,
}
```

Options for filesystem APIs what write to files and/or directories.

This is a dictionary that may contain one or more of the following values:

* `overwrite` - If the target path should be overwritten or not, in the case that it already exists
