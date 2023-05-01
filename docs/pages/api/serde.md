<!-- @generated with lune-cli -->

# Serde

Built-in serialization/deserialization & encoding/decoding functions

### Example usage

```lua
local fs = require("@lune/fs")
local serde = require("@lune/serde")

-- Parse different file formats into lua tables
local someJson = serde.decode("json", fs.readFile("myFile.json"))
local someToml = serde.decode("toml", fs.readFile("myFile.toml"))
local someYaml = serde.decode("yaml", fs.readFile("myFile.yaml"))

-- Write lua tables to files in different formats
fs.writeFile("myFile.json", serde.encode("json", someJson))
fs.writeFile("myFile.toml", serde.encode("toml", someToml))
fs.writeFile("myFile.yaml", serde.encode("yaml", someYaml))
```

---

## Functions

### decode

```lua
function serde.decode(format: SerdeEncodeDecodeFormat, encoded: string)
```

Decodes the given string using the given format into a lua value.

### encode

```lua
function serde.encode(format: SerdeEncodeDecodeFormat, value: any, pretty: boolean?)
```

Encodes the given value using the given format.

---

## Types

### SerdeEncodeDecodeFormat

```lua
type SerdeEncodeDecodeFormat = "json" | "yaml" | "toml"
```