# 関数
型関数:fnc
fnc tup
最後の文,値を評価する
名称:fnc
val||fnc val
val||fnc:
        val
## 特殊関数
### bool関数
名称:bfnc
val|| var1 bfnc var2

### 算用関数
名称:sfnc
val|| var1 sfnc var2
### 制御関数
名称:cfnc
#### 条件処理
#### 反復処理
### 型関数
名称:dfnc
val||dfnc val


# 置換
var=val
# コレクション型
名称:col
## タプル
型関数:tup
名称:tup
(val,val,val...)
(
  val
  val
  ...
)
## リスト
型関数:list
名称:list
[val,val,val...]
[
  val
  val
  ...
]
## 辞書
型関数:dic
名称:dic
{val:val,val:val,val:val...}
{
  val:val
  val:val
  ...
}
## 集合
型関数:set
名称:set
{val,val,val...}
{
  val
  val
  ...
}
# 型
型関数:type
type 
名称:type
必ずメソットを持つ
val||var.mthd val
は以下と同じ意味を持つ
val||fnc var val
# コメント
#コメント#
##改行までコメント
###範囲コメント###

