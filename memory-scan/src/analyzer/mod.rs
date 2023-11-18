use std::{
    borrow::{Borrow, BorrowMut},
    collections::HashMap,
    hash::Hash,
};

use aya::{
    maps::{HashMap as BpfHashMap, MapData},
    Pod,
};

// #[derive(Debug)]
// pub struct MemoryHotMap(pub HashMap<i64, i64>);

// impl MemoryHotMap {
//     /// 向 Hashmap中添加元素，如果存在就值相加，不存在就插入
//     ///
//     /// ## 参数
//     ///
//     /// - `k` 键
//     /// - `v` 值
//     ///
//     /// ## 返回值
//     ///
//     /// - `Ok((i64,i64))` 如果成功，返回(k,v)
//     /// - `Err(i64)` 如果失败(指的是可能出现的并发错误，但是没有做任何处理)，返回Err(k)
//     fn add(&self, k: i64, v: i64) -> Result<(i64, i64), i64> {
//         let reader = self.0.get(&k);
//         if reader.is_some() {
//             if self.0.insert(k, v + reader.unwrap()).is_some() {
//                 return Ok((k, v + reader.unwrap()));
//             } else {
//                 return Err(k);
//             }
//         } else {
//             if self.0.insert(k, v).is_none() {
//                 return Ok((k, v));
//             } else {
//                 return Err(k);
//             }
//         }
//     }
//     pub fn take_from_bpfmap<T, K, V>(&self, bpf_map: BpfHashMap<T, K, V>) {}
// }

#[derive(Debug)]
pub struct MemoryHotMap<
    K: Eq + Hash + PartialEq + Pod,
    V: for<'a> std::ops::Add<&'a V, Output = V> + Pod,
>(HashMap<K, V>);

impl<K: Eq + Hash + PartialEq + Pod, V: for<'a> std::ops::Add<&'a V, Output = V> + Pod>
    MemoryHotMap<K, V>
{
    /// 向 Hashmap中添加元素，如果存在就值相加，不存在就插入
    ///
    /// ## 参数
    ///
    /// - `k` 键
    /// - `v` 值
    ///
    /// ## 返回值
    ///
    /// - `Ok((i64,i64))` 如果成功，返回(k,v)
    /// - `Err(i64)` 如果失败(指的是可能出现的并发错误，但是没有做任何处理)，返回Err(k)
    fn add(&self, k: K, v: V) -> Result<(K, V), K> {
        let reader = self.0.get(&k);
        if reader.is_some() {
            if self.0.insert(k, v + reader.unwrap()).is_some() {
                return Ok((k, v + reader.unwrap()));
            } else {
                return Err(k);
            }
        } else {
            if self.0.insert(k, v).is_none() {
                return Ok((k, v));
            } else {
                return Err(k);
            }
        }
    }
    /// 从 bpf_map 中遍历提取元素
    ///
    /// ## 参数
    ///
    /// - `bpf_map` 传入的 aya::maps::hash_map::hash_map 类型的hashmap
    pub fn take_from_bpfmap<T: BorrowMut<MapData>>(&self, mut bpf_map: BpfHashMap<T, K, V>) {
        for iter in bpf_map.iter() {
            if iter.is_ok() {
                let kv = iter.unwrap();
                bpf_map.remove(&kv.0);
                self.add(kv.0, kv.1);
            }
        }
    }

    pub fn new() -> Self {
        Self(HashMap::new())
    }
}
