# dump-file-origin



A small CLI utility for extracting a file's origin metadata on macOS i.e dump [kMDItemWhereFroms](https://developer.apple.com/documentation/coreservices/kmditemwherefroms). Will work on macOS 10.4+ only. 

## Install

```console
$ cargo install --git https://github.com/0x6b/dump-file-origin
```

## Uninstall

```console
$ cargo uninstall dump-file-origin
```

## Usage

```
A small CLI utility for extracting a file's origin metadata on macOS.

Usage: dump-file-origin [OPTIONS] [PATH]

Arguments:
  [PATH]  Path to a file, or a directory to check. Default: ~/Downloads

Options:
  -a, --all      Output file name, regardless of xattr kMDItemWhereFroms presence
  -h, --help     Print help
  -V, --version  Print version
```

## Privacy

This CLI never send your data to any server.

## License

This extension is released under the MIT License. See [LICENSE](LICENSE) for details.
