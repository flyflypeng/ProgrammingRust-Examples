fn main() {
    let mut v = Vec::new();
    for i in 101..106 {
        v.push(i.to_string());
    }

    // move the string from vector to a new variable is forbidden
    // recommend: use borrow references instead
    // let a = v[0];
    // let b = v[1];

    let a = &v[0];
    let b = &v[1];

    println!("a: {}, b: {}", a, b);

    let v = vec![
        "liberté".to_string(),
        "égalité".to_string(),
        "fraternité".to_string(),
    ];

    for mut s in v {
        s.push('!');
        println!("{}", s);
    }

    // v nolonger has the ownership of the vector
    // println!("v: {:?}", v);
}
