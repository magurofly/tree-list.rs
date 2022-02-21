# tree-list.rs

平衡二分木（AVL木）で実装されたリスト。

## 操作

次の操作が O(1) で可能。

- `self.len() -> usize`: 要素数の取得
- `self.is_empty() -> bool`: 空か判定

次の操作が O(logN) で可能。

- `&self[index] -> &T`: 要素の取得
- `&mut self[index] -> &mut T`: 要素の変更
- `self.push_front(element)`: 先頭への要素の挿入
- `self.push_back(element)`: 末尾への要素の挿入
- `self.pop_front() -> Option<T>`: 先頭からの要素の取り出し
- `self.pop_back() -> Option<T>`: 末尾からの要素の取り出し
- `self.insert(index, element)`: 任意の位置への要素の挿入
- `self.remove(index) -> Option<T>`: 任意の位置の要素の削除
- `self.append(&mut other)`: 結合
- `self.split_off(index) -> TreeList<T>`: 分割

次の操作が O(N) で可能。

- `self.iter() -> impl Iterator`: 要素の列挙

次の操作が O(NlogN) で可能。

- `TreeList::from_iter(into_iter)`: `IntoIterator` からの構築
