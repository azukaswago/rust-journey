fn vecone(v: &mut Vec<i32>, val:i32) {
    for i in v.iter() {
        println!("{}", i);
    }
       v.push(val);
}

fn main() {
   let mut v = vec![1, 2, 3, 4];
   let val = 5;

    vecone(&mut v, val);
    println!("{:?}", v);

    let mut v2 = vec![10, 20, 30];
    v2[0] = 100;
    println!("{:?}", v2);

    for i in v2.iter_mut() {
        *i *= 2;
        println!("{}", i)
    }
}
