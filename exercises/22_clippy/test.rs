fn foo<'a>(x: &'a mut i32,  y: &'a mut i32) {
    let tmp = x;
    x = y;
    y = tmp;
    println!("{} {}", x, y);
}

fn main() {
    let x:i32 = 1;
    let y:i32 = 2;
    foo(&x, &y);
	println!("{} {}", x, y);
}
