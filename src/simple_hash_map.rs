use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::hash::{BuildHasher, Hash};
use std::marker::PhantomData;
use std::ptr::{hash, NonNull, null};

#[cfg(not(feature = "ahash"))]
pub enum DefaultHashBuilder {}

// Separate Chaining
pub struct SimpleHashMap<K, V, S = DefaultHashBuilder> {
    pub(crate) hash_builder: S,
    pub table: Box<[Node<K, V>; 6]>,
    size: usize,
    mod_count: u32,
    threshold: u32,
}

struct Node<K, V> {
    key: K,
    value: V,
    hash: usize,
    next: Option<NonNull<Node<K, V>>>,
    marker: PhantomData<K>,
}

impl<K, V> Node<K, V> {
    fn new(key: K, value: V, hash: usize) -> Self {
        Self {
            key,
            value,
            hash,
            next: None,
            marker: PhantomData,
        }
    }
}

impl<K, V> SimpleHashMap<K, V> {
    fn index_for(&self, hash: &usize) -> usize {
        hash % self.size
    }

    fn putVal(&mut self, mut node: Box<Node<K, V>>) -> Option<V>{
        let mut tab = *self.table;
        let n: usize = self.size;
        let i: usize = self.index_for(&node.hash);
        let p: Option<Node<K, V>> = Some(tab[i]);
        if tab.is_empty() && n == 0 {
            // TODO: resize(): HashMap의 사이즈를 늘리는 함수.
        }
        match p {
            None => tab[i] = *node,
            Some(mut p) => {
                let e: Option<Node<K, V>>;
                let k:K = p.key;

                if p.hash == node.hash && k == node.key || node.key == k {
                    let mut e = e.unwrap();
                    let old_value = e.value;
                    e.value = node.value;
                    old_value
                }
                else {
                    let mut bin_count:u32 = 0;
                    loop {
                        if p.next.is_none() {
                            p.next = Some(Box::leak(node.into()).into());
                            break;
                        }
                        let e:Node<K, V> = Option::from(p).unwrap();
                        if p.hash == node.hash && k == node.key || k == node.key {
                            break;
                        }
                        p = e;
                        bin_count += 1;
                    }
                }
            }
        }
        self.mod_count += 1;
        self.size += 1;
        if self.size > threshold {
            // TODO: resize(): HashMap의 사이즈를 늘리는 함수.
        }
        None
    }
}
// make_insert_hash::<K, S>(&self.hash_builder, &k);