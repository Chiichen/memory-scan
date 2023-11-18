use std::{
    borrow::BorrowMut,
    collections::HashMap,
    fmt::Display,
    fs::File,
    io::{self, ErrorKind},
    path::Path,
    sync::{Arc, Mutex},
};

use aya::maps::{HashMap as BpfHashMap, MapData};

#[derive(Debug)]
pub struct MemoryHotMap(Arc<Mutex<HashMap<i64, i64>>>);

impl Display for MemoryHotMap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.0.lock())
    }
}

impl MemoryHotMap {
    pub fn len(&self) -> usize {
        return self.0.lock().unwrap().len();
    }
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
    fn add(&self, k: i64, v: i64) -> Result<(i64, i64), i64> {
        let r = self.0.lock();
        if r.is_err() {
            return Err(k);
        }
        let mut writer = r.unwrap();
        let r = writer.get_mut(&k);
        if r.is_some() {
            let prev = r.unwrap();
            *prev = (*prev).clone() + v;
            return Ok((k, v));
        } else {
            if writer.insert(k, v).is_none() {
                return Ok((k, v));
            } else {
                return Err(k);
            }
        }
    }
    pub fn take_from_bpfmap<T: BorrowMut<MapData>>(
        &self,
        bpf_map: Arc<Mutex<BpfHashMap<T, i64, i64>>>,
    ) {
        let r = bpf_map.lock();
        if r.is_err() {
            return;
        }
        let mut writer = r.unwrap();
        let mut iter_vec = Vec::<(i64, i64)>::new();
        for iter in writer.iter() {
            if iter.is_ok() {
                iter_vec.push(iter.unwrap());
            }
        }
        for kv in iter_vec {
            let r = writer.remove(&kv.0);
            if r.is_err() {
                println!(
                    "Failed to remove {:?} from bpf map with error {}",
                    kv,
                    r.unwrap_err()
                )
            }
            let r = self.add(kv.0, kv.1);
            if r.is_err() {
                println!("Failed to add {:?} into userspace hashmap", kv)
            }
        }
    }

    pub fn save_to_file(&self, file_path: &str) -> io::Result<usize> {
        let reader = self.0.lock();
        if reader.is_err() {
            return Err(ErrorKind::PermissionDenied.into());
        }
        let path = Path::new(file_path);
        let file = File::create(path)?;

        let mut writer = csv::Writer::from_writer(file);

        for (key, value) in reader.unwrap().iter() {
            writer.write_record(&[key.to_string(), value.to_string()])?;
        }

        writer.flush()?;
        Ok(self.len())
    }

    pub fn new() -> Self {
        Self(Arc::new(Mutex::new(HashMap::new())))
    }
}

// #[derive(Debug)]
// pub struct MemoryHotMap<K, V>(Arc<Mutex<HashMap<K, V>>>);

// impl<K, V> Display for MemoryHotMap<K, V> {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         todo!()
//     }
// }

// impl<K: Eq + Hash + PartialEq + Pod, V: for<'a> std::ops::Add<&'a V, Output = V> + Pod + Copy>
//     MemoryHotMap<K, V>
// {
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
//     fn add(&mut self, k: K, v: V) -> Result<(K, V), K> {
//         let r = self.0.lock();
//         if r.is_err() {
//             return Err(k);
//         }
//         let mut writer = r.unwrap();
//         let r = writer.get_mut(&k);
//         if r.is_some() {
//             let prev = r.unwrap();
//             *prev = (*prev).clone() + v;
//             return Err(k);
//         } else {
//             if writer.insert(k, v).is_none() {
//                 return Ok((k, v));
//             } else {
//                 return Err(k);
//             }
//         }
//     }
//     /// 从 bpf_map 中遍历提取元素
//     ///
//     /// ## 参数
//     ///
//     /// - `bpf_map` 传入的 aya::maps::hash_map::hash_map 类型的hashmap
//     pub fn take_from_bpfmap<T: BorrowMut<MapData>>(
//         &mut self,
//         bpf_map: Arc<Mutex<BpfHashMap<T, K, V>>>,
//     ) {
//         let r = bpf_map.lock();
//         if r.is_err() {
//             return;
//         }
//         let mut writer = r.unwrap();
//         return for iter in writer.iter() {
//             if iter.is_ok() {
//                 let kv = iter.unwrap();
//                 writer.remove(&kv.0);
//                 self.add(kv.0, kv.1);
//             }
//         };
//     }

//     pub fn new() -> Self {
//         Self(Arc::new(Mutex::new(HashMap::new())))
//     }
// }
