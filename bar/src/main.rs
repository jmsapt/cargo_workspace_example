use foo::add;

fn main() {
    println!("Using `add` from our lib crate `foo`:");
    println!("foo::add(10, 23) => {}", add(10, 23));
}
