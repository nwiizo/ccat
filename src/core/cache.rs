use anyhow::Result;
use lru::LruCache;
use std::path::{Path, PathBuf};
use std::fs;
use std::time::SystemTime;
use std::num::NonZeroUsize;
use sha2::{Sha256, Digest};

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct CacheKey {
    path: PathBuf,
    modified: SystemTime,
    size: u64,
}

impl CacheKey {
    pub fn from_path<P: AsRef<Path>>(path: P) -> Result<Self> {
        let path = path.as_ref();
        let metadata = fs::metadata(path)?;
        
        Ok(Self {
            path: path.to_path_buf(),
            modified: metadata.modified()?,
            size: metadata.len(),
        })
    }
}

pub struct FileCache<T> {
    cache: LruCache<CacheKey, T>,
}

impl<T> FileCache<T> {
    pub fn new(capacity: usize) -> Self {
        Self {
            cache: LruCache::new(NonZeroUsize::new(capacity).unwrap_or(NonZeroUsize::MIN)),
        }
    }

    pub fn get<P: AsRef<Path>>(&mut self, path: P) -> Result<Option<&T>> {
        let key = CacheKey::from_path(path)?;
        Ok(self.cache.get(&key))
    }

    pub fn get_mut<P: AsRef<Path>>(&mut self, path: P) -> Result<Option<&mut T>> {
        let key = CacheKey::from_path(path)?;
        Ok(self.cache.get_mut(&key))
    }

    pub fn insert<P: AsRef<Path>>(&mut self, path: P, value: T) -> Result<()> {
        let key = CacheKey::from_path(path)?;
        self.cache.put(key, value);
        Ok(())
    }

    pub fn remove<P: AsRef<Path>>(&mut self, path: P) -> Result<Option<T>> {
        let key = CacheKey::from_path(path)?;
        Ok(self.cache.pop(&key))
    }

    pub fn clear(&mut self) {
        self.cache.clear();
    }

    pub fn len(&self) -> usize {
        self.cache.len()
    }

    pub fn is_empty(&self) -> bool {
        self.cache.is_empty()
    }
}

pub struct ContentHashCache {
    hash_cache: LruCache<PathBuf, String>,
}

impl ContentHashCache {
    pub fn new(capacity: usize) -> Self {
        Self {
            hash_cache: LruCache::new(NonZeroUsize::new(capacity).unwrap_or(NonZeroUsize::MIN)),
        }
    }

    pub fn get_or_compute<P: AsRef<Path>>(&mut self, path: P) -> Result<String> {
        let path = path.as_ref();
        
        if let Some(hash) = self.hash_cache.get(&path.to_path_buf()) {
            return Ok(hash.clone());
        }

        let hash = self.compute_hash(path)?;
        self.hash_cache.put(path.to_path_buf(), hash.clone());
        Ok(hash)
    }

    fn compute_hash<P: AsRef<Path>>(&self, path: P) -> Result<String> {
        let mut file = fs::File::open(path)?;
        let mut hasher = Sha256::new();
        std::io::copy(&mut file, &mut hasher)?;
        Ok(format!("{:x}", hasher.finalize()))
    }
}