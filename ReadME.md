# MonarchToGriddyPlug

Convert a monarch language to a GriddyCode Lua plugin.

## Usage

```shell
# convert the module to JSON
deno run getJSON.ts path/to/monarch/language/file.ts
# run the converter
cargo run -- path/to/lang.json path/to/conf.json
```

## Idea

Given a JSON version of the language and config,
convert as much as possible to a GriddyCode Lua plugin.
