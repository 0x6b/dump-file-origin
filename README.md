# macos-downloaded-url-viewer

A small CLI utility to view the downloaded URL of a file on macOS.

## Install

```console
$ cargo install --git https://github.com/0x6b/macos-downloaded-url-viewer
```

## Uninstall

```console
$ cargo uninstall macos-downloaded-url-viewer
```

## Usage

```
A small CLI utility to view the downloaded URL of a file on macOS

Usage: macos-downloaded-url-viewer [OPTIONS] [PATH]

Arguments:
  [PATH]  Path to the file, or directory to check. Defaults to ~/Downloads

Options:
  -a, --all      Output file name, regardless of xattr kMDItemWhereFroms presence
  -h, --help     Print help
  -V, --version  Print version
```

## Privacy

This CLI never send your data to any server.

## License

This extension is released under the MIT License. See [LICENSE](LICENSE) for details.
