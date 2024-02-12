use TryMorcos::call_with_larch;
use TryMorcos::expand_to_larch;
use TryMorcos::recognize_tree;
fn main() {
    let s = expand_to_larch!();
    println!("{s}");
    recognize_tree!(expand_to_larch!()); // 无法直接使用 `expand_to_larch!` 的展开结果
    call_with_larch!(recognize_tree); // 回调就是给另一个宏传入宏的名称 (`ident`)，而不是宏的结果
}
