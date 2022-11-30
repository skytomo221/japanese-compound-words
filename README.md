# Japanese Compound Words

日本語のよく使われる複合語リストを生成します。

## 環境構築

```console
$ docker-compose build
$ docker-compose run corpus
# cp /jawiki.xml.bz2 /workspace
# exit
$ docker-compose run wikiextractor
$ docker-compose run generator
```

1. ターミナルによる実行で上のコマンドを実行する。
2. Visual Studio Code を起動する。
3. 拡張機能で[Remote - Containers](https://marketplace.visualstudio.com/items?itemName=ms-vscode-remote.remote-containers)をインストールする。
4. ステータスバーの左端にある![image](https://user-images.githubusercontent.com/18415838/137567497-f16c9ef4-ed2c-4f8e-bde4-d3d5f452787e.png)
   をクリックする。
5. 「Reopen in Container」をクリックする。
6. ターミナル上で`cargo run`を実行する。
