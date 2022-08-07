# rprs
:crab: A CLI app for replacing files from source to desination directory.

## Install
1. Binaries are placed in the [release tags](https://github.com/Karthik-d-k/rprs/releases/tag/v0.1.0-alpha), you can choose depending on your OS.
2. Download and extract the binary and you are all set to use the tool.

## Usage
```bash
rprs <src_dir> <des_dir> <enable_case_sensitive(optional)>
```
- `src_dir` --> Path to source directory.
- `des_dir` --> Path to destinatin directory.
- `enable_case_sensitive` --> Enabling case sensitivity for file names while replacing (optional), defaults to case insensitive.

## Caveat
|OS      | Tested           | Working          |
|--------|:----------------:|:----------------:|
|Linux   |:heavy_check_mark:|:heavy_check_mark:|
|Windows |:x:               |:x:               |
|MacOS   |:x:               |:x:               |

## About tool name
`rprs` is a combination of following words -->
- `rp` -> Stands for **replace**
- `rs` -> Stands for **rust**
