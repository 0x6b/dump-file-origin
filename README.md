# macos-downloaded-url-viewer

A small CLI utility to display where each file was obtained from i.e dump [kMDItemWhereFroms](https://developer.apple.com/documentation/coreservices/kmditemwherefroms). Will work on macOS 10.4+ only. 

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
