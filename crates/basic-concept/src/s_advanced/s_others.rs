use rand::Rng;
use std::hash::Hash;
use std::{collections::HashMap, env, ffi::OsString};

///
/// 限制可见性语法
/// pub                 无任何限制
/// pub(crate)          限制可见性为当前crate内部
/// pub(super)          限制可见性为父模块
/// pub(self)           限制可见性为当前模块
/// pub(in crate::xxx)  限制可见性为某个模块
pub(in crate::s_advanced) fn study_rand() {
    let rand_score = rand::thread_rng().gen_range(1..101);
    println!("{rand_score}");
}

pub fn study_format() {
    println!(" {{ and }} is \"transformed\"");

    // 若参数出现非unicode字符，那std::env::args()会panic
    let args: Vec<String> = env::args().collect();

    let args2: Vec<OsString> = std::env::args_os().collect();
}

fn get_default<'m, K, V>(map: &'m mut HashMap<K, V>, key: K) -> &'m mut V
where
    K: Clone + Eq + Hash,
    V: Default,
{
    // match map.get_mut(&key) {
    //     Some(value) => value,
    //     None => {
    //         map.insert(key.clone(), V::default());
    //         map.get_mut(&key).unwrap()
    //     }
    // }

    if map.contains_key(&key) {
        return map.get_mut(&key).unwrap();
    }
    map.insert(key.clone(), V::default());
    return map.get_mut(&key).unwrap();
}

fn test() {
    let arr = vec![Box::new(1), Box::new(2)];
    let (first, second) = (&arr[0], &arr[1]);
    let sum = **first + **second;
}

struct Node {
    next: Option<Box<Node>>,
}

impl Node {
    fn back(&mut self) -> &mut Option<Box<Node>> {
        let mut anchor = &mut self.next;
        while let Some(ref mut node) = *anchor {
            anchor = &mut node.next;
        }
        anchor
    }
}
