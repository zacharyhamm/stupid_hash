const NHASH: usize = 4096;
const MULTIPLIER: usize = 31;

pub struct NameVal<T> {
    name: String,
    value: T,
}

pub struct Hash<T>(Vec<Vec<NameVal<T>>>);

impl<T> Hash<T> {
    fn new() -> Self {
        let mut hash_vec = Vec::with_capacity(NHASH);

        for i in 0..NHASH {
            hash_vec.push(Vec::<NameVal<T>>::new());
        }

        Hash(hash_vec)
    }

    fn hash(name: &str) -> usize {
        let mut h: usize = 0;

        for p in name.bytes() {
            let p = p as usize;
            // Instead of silently wrapping (like most C implementations do, even if that is
            // strictly undefined), rust panics if we overflow an integer value. So we need to use
            // this magic instead.
            h = h.wrapping_mul(MULTIPLIER).wrapping_add(p);
        }
        h % NHASH
    }

    fn lookup(&self, name: &str) -> Option<&T> {
        let h = Hash::<T>::hash(name);
        let entries = &self.0[h];

        match entries.len() {
            0 => None,
            _ => {
                for entry in entries.iter() {
                    if entry.name == name {
                        return Some(&entry.value);
                    }
                }
                None
            }
        }
    }

    fn upsert(&mut self, name: &str, value: T) {
        let h = Hash::<T>::hash(name);
        let mut entries = &mut self.0[h];

        for entry in entries.iter_mut() {
            if entry.name == name {
                entry.value = value;
                return;
            }
        }

        entries.push(NameVal {
            name: name.to_string(),
            value: value,
        });
    }
}

#[cfg(test)]
mod test {
    use super::Hash;
    use super::NameVal;

    #[test]
    fn basics() {
        let mut hashtab = Hash::new();
        hashtab.upsert("abc", 64);
        hashtab.upsert("abcdefghijklmnopq", 128);
        assert_eq!(hashtab.lookup("abc"), Some(&64));
        assert_eq!(hashtab.lookup("abc"), Some(&64));
        assert_eq!(hashtab.lookup("abcdefghijklmnopq"), Some(&128));
        hashtab.upsert("abc", 256);
        assert_eq!(hashtab.lookup("abc"), Some(&256));
        assert_eq!(hashtab.lookup("abcd"), None);
    }
}