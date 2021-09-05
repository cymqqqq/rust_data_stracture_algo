use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::collections::VecDeque;

type Pair = (Reverse<usize>, String);
#[derive(Default, Debug)]
struct Trie {
    child: HashMap<char, Trie>,
    count: usize,
}
impl Trie {
    fn new() -> Self {
        Trie::default()
    }
    fn insert(&mut self, s: String, time: usize) {
        let mut link = self;
        for c in s.chars() {
            link = link.child.entry(c).or_default();
        }
        link.count += time;
    }
    fn search(&self, s: &mut Vec<char>, top3: &mut BinaryHeap<Pair>) {
        if self.count > 0 {
            top3.push((Reverse(self.count), s.iter().copied().collect()));
        }
        if top3.len() > 3 {
            top3.pop();
        }
        for (&k, v) in &self.child {
            s.push(k);
            v.search(s,top3);
            s.pop();
        }
    }
}
struct AutocompleteSystem {
    buf: Vec<char>,
    trie: Trie,
}
impl AutocompleteSystem {
    fn new(sentence: Vec<String>, times: Vec<i32>) -> Self {
        let mut trie = Trie::default();
        for (i, s) in sentence.into_iter().enumerate() {
            trie.insert(s, times[i] as usize);
        }
        let buf: Vec<char> = vec![];
        AutocompleteSystem { buf, trie }
    }
    fn input(&mut self, c: char) -> Vec<String> {
        if c == '#' {
            let s: String = self.buf.drain(..).collect();
            self.trie.insert(s, 1);
            vec![]
        } else {
            let mut top3: BinaryHeap<Pair> = BinaryHeap::new();
            self.buf.push(c);
            let mut link = &mut self.trie;
            for &c in self.buf.iter() {
                link = link.child.entry(c).or_default();
                
            }
            link.search(&mut self.buf, &mut top3);
            let mut res: VecDeque<String> = VecDeque::new();
            while let Some((_, s)) = top3.pop() {
                res.push_front(s);
            }
            res.into_iter().collect()
        }
    }
}
