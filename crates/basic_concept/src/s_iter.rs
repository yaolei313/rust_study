/// [IntoIterator]
pub fn study_iter() {
    let mut v1 = vec![1, 2, 3];
    // 不可变引用
    let mut i1 = v1.iter();
    assert_eq!(i1.next(), Some(&1));

    // 结果是5
    let sum: i32 = i1.sum();
    println!("sum : {}", sum);

    // 可变引用
    let mut i2 = v1.iter_mut();
    assert_eq!(i2.next(), Some(&mut 1));
    for item in i2.take(2) {
        // 修改内容
        *item += 1;
    }
    println!("after iter_mut modify: {:?}", v1);

    // 所有权转移
    let mut i3 = v1.into_iter();
    assert_eq!(i3.next(), Some(1));

    let v2 = vec!["1", "2", "3"];
    let v3: Vec<String> = v2
        .iter()
        .map(|x| -> String {
            let mut r = String::from(*x);
            r.push_str("suffix");
            r
        })
        .collect();

    // map
    let v4: Vec<String> = v2.into_iter().map(|x| x.to_owned()).collect();

    // filter
    let v5 = vec![1, 2, 3, 4];
    let v6: Vec<&i32> = v5.iter().filter(|x| *x % 2 == 0).collect();
    println!("{:?}", v6);

    // fold
    let sum = v5.iter().fold(0, |acc, x| acc + x);
    println!("sum is {}", sum);

    let a6 = [10; 5];
    let sum1: i32 = a6.iter().sum();
    Iterator::sum::<i32>(a6.iter());
}
