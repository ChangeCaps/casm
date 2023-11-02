use std::{collections::HashMap, fs, ops::Index, path::PathBuf};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct SourceId {
    index: usize,
}

impl SourceId {
    pub const NULL: Self = Self::null();

    pub const fn null() -> Self {
        SourceId { index: usize::MAX }
    }

    pub fn is_null(&self) -> bool {
        *self == Self::null()
    }
}

#[derive(Clone, Debug)]
pub struct File {
    pub path: PathBuf,
    pub contents: String,
}

impl File {
    pub fn read(path: impl Into<PathBuf>) -> std::io::Result<Self> {
        let path = path.into();
        let contents = fs::read_to_string(&path)?;

        Ok(Self { path, contents })
    }
}

#[derive(Clone, Debug, Default)]
pub struct SourceMap {
    sources: HashMap<SourceId, File>,
    next_id: usize,
}

impl SourceMap {
    pub fn new() -> Self {
        Self::default()
    }

    fn generate_id(&mut self) -> SourceId {
        if self.next_id + 1 == usize::MAX {
            panic!(
                "Somehow {}, SourceIds have been generated, that's on you",
                usize::MAX
            );
        }

        let id = SourceId {
            index: self.next_id,
        };

        self.next_id += 1;

        id
    }

    pub fn insert(&mut self, file: File) -> SourceId {
        let id = self.generate_id();
        self.sources.insert(id, file);
        id
    }

    pub fn get(&self, source: &SourceId) -> Option<&File> {
        self.sources.get(source)
    }
}

impl Index<SourceId> for SourceMap {
    type Output = File;

    fn index(&self, index: SourceId) -> &Self::Output {
        self.get(&index).expect("Source not present in SourceMap")
    }
}
