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
lf [regex...] [-e <glob...>] [-a] [-0]
```

カレントディレクトリにあるpdfファイルを検索したかったら

```
lf pdf$
```

再帰的に検索したかったら

```
lf -a pdf$
```

カレントディレクトリにあるpdfファイルとdocxファイルを検索したかったら

```
lf pdf$ docx$
```

そこからbakという文字列が含まれているものを除外するには

```
lf pdf$ docx$ -e '*bak*'
```

そこから更にorgという文字列が含まれているものを除外するには

```
lf pdf$ docx$ -e '*bak*' '*org*'
```

ちなみに*の代わりに@を使うことでシングルクオートで囲わなくてもよくなる

```
lf pdf$ docx$ -e @bak@ @org@
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
