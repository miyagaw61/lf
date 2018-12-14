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
lf [regex...] [--exclude|-e <glob...>] [--all|-a] [-0] [--depth|-d <depth>] [--type|-t <type>] [--file|-f <file-extension>] [--file-exclude|-F <file-extension>] [--no-ignore|-I]
```

カレントディレクトリにあるtmpから始まるファイルを検索するには

```
lf ^tmp
```

再帰的に検索するには

```
lf --all ^tmp
lf -a ^tmp
```

引数に渡す正規表現はファイルパスのベースネームだけにマッチするようになっている  
そのため、ベースネーム以外の絞り込みは普通にgrepする必要がある

```
lf -a ^tmp | rg ^src
```

depth2で検索するには

```
lf --depth 2 ^tmp
lf -d2 ^tmp
```

ディレクトリのみ検索するには

```
lf --type d ^tmp
lf -td ^tmp
```

ディレクトリ以外を検索するには

```
lf --type f ^tmp
lf -tf ^tmp
```

カレントディレクトリにあるtmpかtestから始まるファイルを検索するには

```
lf '^tmp|^test'
```

カレントディレクトリにあるtmpから始まりtestを含むファイルを検索するには

```
lf ^tmp test
```

そこから`*bak*`というglobにマッチするファイルを除外するには

```
lf ^tmp test --exclude '*bak*'
lf ^tmp test -e '*bak*'
```

そこから更に`*org*`というglobにマッチするファイルを除外するには

```
lf ^tmp test --exclude '*bak*' '*org*'
lf ^tmp test -e '*bak*' '*org*'
```

`*`の代わりに`@`を使うことでシングルクオートが必須ではなくなる

```
lf ^tmp test --exclude @bak@ @org@
lf ^tmp test -e @bak@ @org@
```

カレントディレクトリにある拡張子がpdfのファイルを検索するには

```
lf --file pdf
lf -f pdf
```

カレントディレクトリにある拡張子がpdfかpptxのファイルを検索するには

```
lf --file pdf pptx
lf -f pdf pptx
```

カレントディレクトリにある拡張子がpng以外のファイルを検索するには

```
lf --file-exclude png
lf -F png
```

カレントディレクトリにある拡張子がpngとjpg以外のファイルを検索するには

```
lf --file-exclude pdf jpg
lf -F pdf jpg
```

-0,-Iオプションはfdと同様。
