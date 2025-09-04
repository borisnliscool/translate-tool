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
  -c, --config <CONFIG>                      
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
  "translations_directory": "test/translations",
}
```

You can also skip using the file, and optionally provide the `--translations-dir` (or `-t`) flag to specify the
directory. The default is `<cwd>/translations`. The default locale will be set to `en` and there will be no post write
commands.

## Running via docker

You can run translate-tool via the docker image. The image is available
on [ghcr.io/borisnliscool/translate-tool](https://github.com/borisnliscool/translate-tool/pkgs/container/translate-tool).
Remember to change `./translations` to the path of your translations.
If you want to use a configuration file, you can mount it as a volume as well, but this is not required.

```shell
docker run \
  -v ./tt.config.json:/tt.config.json \
  -v ./translations:/translations \
  ghcr.io/borisnliscool/translate-tool:latest \
  translate-tool validate
```