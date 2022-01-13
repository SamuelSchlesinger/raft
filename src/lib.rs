/// Raft Consensus Protocol
use sled::Db;

struct Config {
    db: String,
    participants: usize,
    index: usize,
}

struct Node {
    db: Db,
    commit_index: usize,
    last_applied: usize,
    next_index: Vec<usize>,
    match_index: Vec<usize>,
    index: usize,
    participants: usize,
    is_leader: bool,
}

impl Node {
    fn create(config: Config) -> Option<Node> {
        if let Some(db) = sled::open(&config.db).ok() {
            Some(Node {
                db,
                commit_index: 0,
                last_applied: 0,
                next_index: vec![0; config.participants],
                match_index: vec![0; config.participants],
                index: config.index,
                participants: config.participants,
                is_leader: false,
            })
        } else {
            None
        }
    }

    fn current_term(&self) -> Option<usize> {
        self.db.get(b"term").ok().flatten().map(|x| (*x).try_into().ok()).flatten().map(usize::from_be_bytes)
    }

    fn set_current_term(&self, term: usize) -> Option<()> {
        self.db.insert(b"term", &term.to_be_bytes()).ok().flatten().map(|_| ())
    }

    fn voted_for(&self) -> Option<usize> {
        self.db.get(b"voted_for").ok().flatten().map(|x| (*x).try_into().ok()).flatten().map(usize::from_be_bytes)
    }

    fn set_voted_for(&self, voted_for: usize) -> Option<()> {
        self.db.insert(b"voted_for", &voted_for.to_be_bytes()).ok().flatten().map(|_| ())
    }
}
