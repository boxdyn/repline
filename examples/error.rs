fn main() {
    let mut rl = repline::Repline::with_input([255, b'\r', b'\n'].as_slice(), "", "", "");
    eprintln!("{:?}", rl.read())
}
