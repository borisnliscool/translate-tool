# Translate Tool

Simple tool for managing json-based translation files.

```shell
Usage: translate-tool [OPTIONS] <COMMAND>

Commands:
  add       Add a translation to all locale files
  update    Update a translation in all locale files
  validate  Validate all keys are present
  help      Print this message or the help of the given subcommand(s)

Options:
  -c, --config <CONFIG>                      [default: tt.config.json]
  -t, --translations-dir <TRANSLATIONS_DIR>  
  -h, --help                                 Print help
  -V, --version                              Print version
```

### Configuration

Configuration is done via a `tt.config.json` file. Example:

```json5
{
  // commands to execute after writing to the disk; defaults to an empty list
  "post_write_commands": [
    "bun format"
  ],
  // defaults to 'en'
  "default_locale": "en",
  // defaults to 'translations'
  "translations_directory": "test",
}
```

You can also skip using the file, and only provide the `--translations-dir` (or `-t`) flag to specify the translations'
directory. The default locale will be set to `en` and there will be no post write commands.