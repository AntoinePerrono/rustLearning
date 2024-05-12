fn interproduct(a: i32, b: i32, c: i32) -> i32 {
    return a*b+b*c+c*a;
}

fn takes_u32(x: u32) {
    println!("u32: {x}");
}

fn takes_i8(y: i8) {
    println!("i8: {y}");
}

fn main() {
    println!("Hello, world! it's me, Skafee");
    let a: i32 = 58;
    println!("a: {a}");
    let string: &str = "coucou";
    println!("{string}");
    println!("result: {}", interproduct(120, 45, 36));
    let x = 10;
    let y = 20;

    takes_u32(x);
    takes_i8(y);
}
