#  ヒープソート
## 課題
ヒープソートのアルゴリズムを Rust で実装。

## 仕様
0 以上 100 以下のランダムな整数を15回発生させ、その整数を昇順に並び替える。

## ヒープソートとは
    ツリー状の2分木に分けてソートを行う。  
    ツリーの頂点が最大(または最小)となるように構成する。  
    頂点が決まると、その値はツリーから除外し、末端の値を一度頂点に移動し、再度木構造を構成する。  
    木構造の代わりに配列を使用し、木構造とみなしてソートを行う。
    最大log(2)X回でソートが完了するため、バブルソートより早い。