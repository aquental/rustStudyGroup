#![allow(unused)]

fn main() {
    let mut v: Vec<i32> = Vec::new();
    v.push(1);
    v.push(2);
    v.push(3);

    println!("{:?}", v);

    let mut v: Vec<i32> = vec![0i32; 100];
    v[0] = 1;
    v[99] = 100;
    //v[100] = 101;
    println!("{:?}", v);
    mostra(0, v);

    let resp = Some(1);
    println!("{:?}", resp);
    let um = resp.unwrap();
    println!("{:?}", um);
}

fn mostra(pos: usize, v: Vec<i32>) {
    //println!("position {}:{:?}", pos, v.get(pos));
    //rustacean
    match v.get(pos) {
        Some(x) => println!("position {}: {:?}", pos, x),
        None => println!("position {} does not exist!", pos),
    }
}
