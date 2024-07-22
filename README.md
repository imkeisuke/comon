# comon(common compression)
![Static Badge](https://img.shields.io/badge/License-MIT-blue)
![rs report](https://rust-reportcard.xuri.me/badge/github.com/imkeisuke/comon)
[![Coverage Status](https://coveralls.io/repos/github/imkeisuke/comon/badge.svg?branch=main)](https://coveralls.io/github/imkeisuke/comon?branch=main)
[![build](https://github.com/imkeisuke/comon/actions/workflows/build.yaml/badge.svg)](https://github.com/imkeisuke/comon/actions/workflows/build.yaml)

各種圧縮フォーマットを統一的なインタフェースで扱う

## Description
圧縮フォーマットにはいくつかの種類があるが、それらの圧縮ツールに用いられるインタフェースは異なる点がある。そこで、Comonを用いることで統一的なインタフェースを提供する。これにより、各圧縮フォーマットを容易に扱うことができる

## Usage
```sh
Comon [OPTIONS] <ARGUMENTS...>
ファイルやディレクトリを様々な形式で圧縮・解凍するツールです。

使い方: totebag [オプション] [引数]...

引数:
  [引数]...      処理するファイルやディレクトリのリスト

オプション:
  -m, --mode <MODE>          操作モードを指定します。 [デフォルト: auto] [可能な値: auto, archive, extract, list]
  -o, --output <DEST>        圧縮モードでは出力ファイル、解凍モードでは出力ディレクトリを指定します
  -v, --verbose              詳細な出力を表示します
  -h, --help                 ヘルプを表示します

  
```
## About

### Authors
keisuke nakajima

