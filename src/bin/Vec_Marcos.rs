use TryMorcos::vec_strs;
fn main() {
    let s = vec_strs![1, "a", true, 3.14159f32];
    assert_eq!(s, &["1", "a", "true", "3.14159"]);
    println!("{:#?}", s)
}
