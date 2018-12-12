# lf

lf - Rust製find代替コマンドであるfdのラッパーコマンド

## Install

```
$ wget https://github.com/sharkdp/fd/releases/download/v7.2.0/fd_7.2.0_amd64.deb && sudo dpkg -i ./fd_7.2.0_amd64.deb
$ cargo install ripgrep
$ cargo install --git https://github.com/miyagaw61/lf
```

fdのバージョンは任意

## Usage

```
lf [regex...] [-e <glob...>] [-a] [-0] [-d <depth>] [-t <type>] [-f <file-extension>]
```

カレントディレクトリにあるtmpという文字列を含むファイルを検索したかったら

```
lf tmp
```

再帰的に検索したかったら

```
lf -a tmp
```

depth2で検索したかったら

```
lf -d2 tmp
```

ディレクトリのみ検索したかったら

```
lf -t d tmp
```

ファイルのみ検索したかったら

```
lf -t f tmp
```

カレントディレクトリにあるtmpという文字列を含むファイルとtestという文字列を含むファイルを検索したかったら

```
lf tmp test
```

そこからbakという文字列が含まれているものを除外するには

```
lf tmp test -e '*bak*'
```

そこから更にorgという文字列が含まれているものを除外するには

```
lf tmp test -e '*bak*' '*org*'
```

ちなみに*の代わりに@を使うことでシングルクオートで囲わなくてもよくなる

```
lf pdf$ docx$ -e @bak@ @org@
```

カレントディレクトリにある拡張子がpdfのファイルを検索するには

```
lf -f pdf
```

-0オプションはfdと同様。

## TODO

- 拡張子指定オプション

現状で厳密に拡張子チェックをするには次のようにしなければならないので。

```
lf '\.py$'
```

- ディレクトリだけ/ファイルだけ抽出するオプション

- depth指定オプション

- fdの-Iオプション
