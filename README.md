# disney_wallpaper_changer
disney が毎月配布している壁紙をPCの壁紙に設定する。mac限定

## 手動実行する場合
このリポジトリをクローンして実行
```bash
git clone XXX
cd disney_wallpaper_changer
./disney_wallpaper_changer/target/release/disney_wallpaper_changer
```

## 定期実行する場合
1. このリポジトリをクローン
```bash
git clone XXX
cd disney_wallpaper_changer
```

2. cron の有効化
システム設定 > プライバシーとセキュリティ > フルディスクアクセス > +
cmd + shift + G で検索を開き、 /usr/sbin/cron を開く
一覧に cron があればok

3. 定期実行の設定
cmd から crontab を開く
```bash
crontab -e
```

定期実行の処理を入れる。毎時間実行なら
```vim
0 * * * * /path/to/repository/disney_wallpaper_changer/disney_wallpaper_changer/target/release/disney_wallpaper_changer
```
`/path/to/...` の部分は適宜変える 
:wq で vim を閉じると色々許可を求められるので許可する。

毎時間実行するのは、PCスリープ時にはcronの処理が走らないため。（壁紙取得済みなら処理は全てskipされる）