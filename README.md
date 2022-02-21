# tree-list.rs

平衡二分木（AVL木）で実装されたリスト。

## 操作

次の操作が O(logN) で可能。

- `push_front(element)`: 先頭への要素の挿入
- `push_back(element)`: 末尾への要素の挿入
- `pop_front()`: 先頭からの要素の取り出し
- `pop_back()`: 末尾からの要素の取り出し
- `insert(index, element)`: 任意の位置への要素の挿入
- `remove(index)`: 任意の位置の要素の削除
- `append(&mut other)`: 結合
- `split_off(index)`: 分割

次の操作が O(N) で可能。

- `iter()`: 要素の列挙
- `from_iter(into_iter)`: `IntoIterator` からの構築
