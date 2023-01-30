# rprs

:crab: A CLI application for replacing file(s).

## Install

1. Binaries are placed in the [releases](https://github.com/Karthik-d-k/rprs/releases), you can choose depending on your OS.
2. Download and extract the binary and you are all set to use the tool.

## Usage

```bash
$ rprs <SRC_PATH> <DES_PATH>
```

**Arguments:**

```
<SRC_PATH>  Source file or a directory
<DES_PATH>  Destination file or a directory
```

**Options:**

```
-h, --help
        Print help information
-V, --version
        Print version information
```


## Caveats

| OS      |       Tested       |      Working       |
| ------- | :----------------: | :----------------: |
| Linux   | :heavy_check_mark: | :heavy_check_mark: |
| Windows | :heavy_check_mark: | :heavy_check_mark: |
| MacOS   | :heavy_check_mark: |     :question:     |

> :question: --> I haven't tested it myself, but CI tests are passing.

## About tool name

`rprs` is a combination of following words -->

- `rp` -> Stands for **replace**
- `rs` -> Stands for **rust**
  > `replacer` written in `rust` !!
