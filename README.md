# Frederick

Frederick is a simple and lightweight cli that helps you with metadata on your audio files.

It uses the MusicBrainz API to find metadata for audio files.

> [!WARNING]  
> This project is in version 0.0.1, its usable but some **unknown** bugs might take place. Its not (yet) intended for automated use.


```ts
$ frederick --help

Usage: frederick [OPTIONS]

Options:
  -f, --file <FILE>                  
  -g, --generate-config-file         
  -r, --release-name <RELEASE_NAME>  
  -h, --help                         Print help
  -V, --version                      Print version
```


### Examples
![Help VHS gif](./vhs/help.gif)
![Usage VHS gif](./vhs/usage.gif)

## Build
To build the cli, simply clone the repository and run `cargo build` or `cargo build --release`. You can use the nix flake to ensure that you have all the dependencies. To use it just run `nix develop`

To build the VHS gifs you need to have a flac file with the filename `FLYMETOTHEMOON.flac` on the root of the project. Then run `vhs ./vhs/usage.tape && vhs ./vhs/help.tape`