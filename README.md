# rprs

:crab: A CLI app for replacing files.

## Install

1. Binaries are placed in the [releases](https://github.com/Karthik-d-k/rprs/releases), you can choose depending on your OS.
2. Download and extract the binary and you are all set to use the tool.

## Usage

```bash
$ rprs [OPTIONS] <SRC_DIR> <DES_DIR>
```

**Arguments:**

```
<SRC_DIR>  Path to source directory
<DES_DIR>  Path to destinatin directory
```

**Options:**

```
-C, --enable-case-sensitive
        Enabling case sensitivity for file names while replacing
-H, --enable-hidden-dirs
        Enabling hidden directories for replacing files
-D, --max-depth <MAX_DEPTH>
        maximum allowed depth to recurse through source directory [default: 255]
-I, --ignore-paths <IGNORE_PATHS>
        list of file paths to ignore
-h, --help
        Print help information
-V, --version
        Print version information
```

> By default, `hidden directories` that starts with `.` are untouched !!!

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
