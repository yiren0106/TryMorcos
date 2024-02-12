use TryMorcos::ruc;

fn main() {
    let fib = ruc![a[n]: u64 = 0, 1; ...; a[n-1] + a[n-2]];
    for e in fib.take(11) {
        println!("{}", e)
    }
}
