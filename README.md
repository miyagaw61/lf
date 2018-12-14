# lf

lf - Rust製find代替コマンドであるfdのラッパーコマンド

## Install

```
$ cargo install ripgrep
$ cargo install --git https://github.com/sharkdp/fd
$ cargo install --git https://github.com/miyagaw61/lf
```

## Usage

```
lf [regex...] [-e <glob...>] [-a] [-0] [-d <depth>] [-t <type>] [-f <file-extension>] [-F <file-extension>] [-I]
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

カレントディレクトリにある拡張子がpdf以外のファイルを検索するには

```
lf -F pdf
```

-0,-Iオプションはfdと同様。
