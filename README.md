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

カレントディレクトリにあるtmpから始まるファイルを検索するには

```
lf ^tmp
```

再帰的に検索するには

```
lf -a ^tmp
```

depth2で検索するには

```
lf -d2 ^tmp
```

ディレクトリのみ検索するには

```
lf -td ^tmp
```

ディレクトリ以外を検索するには

```
lf -tf ^tmp
```

カレントディレクトリにあるtmpもしくはtestから始まるファイルを検索するには

```
lf ^tmp ^test
```

そこから`*bak*`というglobにマッチするファイルを除外するには

```
lf ^tmp ^test -e '*bak*'
```

そこから更に`*org*`というglobにマッチするファイルを除外するには

```
lf ^tmp ^test -e '*bak*' '*org*'
```

ちなみにシングルクオートで囲わなくても大丈夫だがファイル数が増えれば増えるほど遅くなる可能性がある

```
lf ^tmp ^test -e *bak* *org*
```

`*`の代わりに`@`を使うことでシングルクオートで囲わなくても速度を維持できるようになる

```
lf ^tmp ^test -e @bak@ @org@
```

カレントディレクトリにある拡張子がpdfのファイルを検索するには

```
lf -f pdf
```

カレントディレクトリにある拡張子がpdfかpptxのファイルを検索するには

```
lf -f pdf pptx
```

カレントディレクトリにある拡張子がpng以外のファイルを検索するには

```
lf -F png
```

カレントディレクトリにある拡張子がpngとjpg以外のファイルを検索するには

```
lf -F pdf jpg
```

-0,-Iオプションはfdと同様。
