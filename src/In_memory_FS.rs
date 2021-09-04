use std::collections::BTreeMap;
#[derive(Debug)]
enum Entry {
    D(Dir),
    F(File),
}
#[derive(Debug)]
struct File {
    name: String,
    content: String,
}
impl File {
    fn new(name: String, content: String) -> Self {
        Self {
            name,
            content
        }
    }
    fn name(&self) -> String {
        self.name.to_string()
    }
    fn append(&mut self, content: String) {
        self.content += &content;
    }
    fn content(&self) -> String {
        self.content.to_string()
    }
}
#[derive(Debug)]
struct Dir {
    name: String,
    entries: BTreeMap<String, Entry>,
}
impl Dir {
    fn new(name: String) -> Self {
        let entries = BTreeMap::new();
        Dir { name, entries }
    }
    fn list(&self) -> Vec<String> {
        self.entries.keys().map(|s| s.to_string()).collect()
    }
}
#[derive(Debug)]
struct FileSystem {
    root: Entry,
}
impl FileSystem {
    fn new() -> Self {
        let root = Entry::D(Dir::new("".to_string()));
        FileSystem { root }
    }
    fn ls(&self, path: String) -> Vec<String> {
        let mut e: &Entry = &self.root;
        for name in path.split('/').filter(|s| !s.is_empty()) {
            if let Entry::D(dir) = e {
                e = &dir.entries[name];
            
        } else {
            panic!();
        }
        }
        match e {
            Entry::D(d) => d.list(),
            Entry::F(f) => vec![f.name()],
        }
    
    }
    fn mkdir(&mut self, path: String) {
        let mut e: &mut Entry = &mut self.root;
        for name in path.split('/').filter(|s| !s.is_empty()) {
            if let Entry::D(dir) = e {
                e = dir.entries
                    .entry(name.to_string())
                    .or_insert_with(|| Entry::D(Dir::new(name.to_string())))
            } else {
                panic!();
            }
        }
    }
    fn add_content_to_file(&mut self, path: String, content: String) {
        let mut e: &mut Entry = &mut self.root;
        for name in path.split('/').filter(|s| !s.is_empty()) {
            if let Entry::D(dir) = e {
                e = dir
                    .entries
                    .entry(name.to_string())
                    .or_insert_with(|| Entry::D(Dir::new(name.to_string())))
            } else {
                panic!();
            }
        }
        if let Entry::F(file) = e {
            file.append(content);
        } else {
            panic!();
        }
    }
    fn read_content_from_file(&mut self, path: String) -> String {
        let mut e: &mut Entry = &mut self.root;
        for name in path.split('/').filter(|s| !s.is_empty()) {
            if let Entry::D(dir) = e {
                e = dir.entries.get_mut(name).unwrap();
                
            } else {
                panic!();
            }
        }
        if let Entry::F(file) = e {
            file.content()
        } else {
            panic!()
        }
    }
    
}
