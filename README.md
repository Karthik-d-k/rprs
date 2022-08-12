# rprs
:crab: A CLI app for replacing files from source to desination directory.

## Install
1. Binaries are placed in the [releases](https://github.com/Karthik-d-k/rprs/releases), you can choose depending on your OS.
2. Download and extract the binary and you are all set to use the tool.

## Usage
```bash
rprs <src_dir> <des_dir> <max_depth(optional)> <enable_case_sensitive(optional)>
```
- `src_dir` --> Path to source directory.
- `des_dir` --> Path to destinatin directory.
- `max_depth` --> maximum allowed depth to recurse through given directory (optional)
  - Should be set to positive integer `> 0`, default value is `255`
- `enable_case_sensitive` --> Enabling case sensitivity for file names while replacing (optional)
  - Should be set to either to `true` or `false`, default is `false`

## Caveats
|OS      | Tested           | Working          |
|--------|:----------------:|:----------------:|
|Linux   |:heavy_check_mark:|:heavy_check_mark:|
|Windows |:heavy_check_mark:|:heavy_check_mark:|
|MacOS   |:heavy_check_mark:|:question:        |
> :question: --> I haven't tested it myself, but CI tests are passing.

## About tool name
`rprs` is a combination of following words -->
- `rp` -> Stands for **replace**
- `rs` -> Stands for **rust**
