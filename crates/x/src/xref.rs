use std::{
    collections::{HashMap, HashSet},
    error::Error,
    fmt, fs, io,
    path::PathBuf,
    process,
    rc::Rc,
};

// ref: xref
/// Cross-reference checker, see `doc/dev/xref.md`
#[derive(Debug, clap::Parser)]
#[command(author, version, about, long_about = None)]
pub(crate) struct Config {
    /// Paths to check for definitions and references
    #[arg()]
    pub(crate) paths: Vec<PathBuf>,
}

#[derive(Debug, Eq, Hash, PartialEq)]
struct Pos {
    file: Rc<PathBuf>,
    line: usize,
}

impl fmt::Display for Pos {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", self.file.display(), self.line)
    }
}

fn find_pat(s: &str, p: &str) -> Option<usize> {
    s.find(p).map(|i| i + p.len())
}

// ref: xref-regex
fn find_def_or_ref(s: &str, def_or_ref: &str) -> Option<String> {
    if let Some(i) = find_pat(s, def_or_ref) {
        let s = &s[i..];
        if let Some(j) = s.find(' ') {
            return Some(s[..j].to_string());
        }
        return Some(s.to_string());
    }
    None
}

fn find_def(s: &str) -> Option<String> {
    find_def_or_ref(s, " def: ")
}

fn find_ref(s: &str) -> Option<String> {
    find_def_or_ref(s, " ref: ")
}

fn collect(
    path: Rc<PathBuf>,
    content: &str,
    defs: &mut HashMap<String, HashSet<Pos>>,
    refs: &mut HashMap<String, HashSet<Pos>>,
) {
    for (n, line) in content.lines().enumerate() {
        let p = Pos {
            file: path.clone(),
            line: n + 1, // line numbers start at 1
        };
        if let Some(def) = find_def(line) {
            defs.entry(def).or_default().insert(p);
        } else if let Some(ref_) = find_ref(line) {
            refs.entry(ref_).or_default().insert(p);
        }
    }
}

fn check(defs: HashMap<String, HashSet<Pos>>, refs: HashMap<String, HashSet<Pos>>) -> bool {
    let mut ok = true;
    // ref: xref-err-ref-has-def
    for (r, ps) in &refs {
        if !defs.contains_key(r) {
            ok = false;
            for p in ps {
                eprintln!("{p}: Missing def for ref `{r}`");
            }
        }
    }
    // ref: xref-err-def-has-ref
    for (d, ps) in &defs {
        if !refs.contains_key(d) {
            ok = false;
            for p in ps {
                eprintln!("{p}: Missing ref for def `{d}`");
            }
        }
    }
    // ref: xref-err-dup-defs
    for (d, ps) in defs {
        let l = ps.len();
        if l != 1 {
            ok = false;
            for (i, p) in ps.into_iter().enumerate() {
                eprintln!("{p}: def {i} of {l} for `{d}`");
            }
        }
    }
    ok
}

pub(super) fn go(mut conf: Config) -> Result<(), Box<dyn Error>> {
    conf.paths.dedup();
    let mut defs = HashMap::with_capacity(64);
    let mut refs = HashMap::with_capacity(64);

    let mut paths = conf.paths;
    while let Some(path) = paths.pop() {
        if path.is_dir() {
            for entry in fs::read_dir(&path)? {
                let entry = entry?;
                paths.push(entry.path());
            }
        } else {
            match fs::read_to_string(path.as_path()) {
                Ok(s) => {
                    let path = Rc::new(path);
                    collect(path, s.as_str(), &mut defs, &mut refs);
                }
                Err(e) if e.kind() == io::ErrorKind::InvalidData => {
                    // invalid UTF-8, just skip
                }
                e @ Err(_) => {
                    e?;
                }
            }
        }
    }
    if check(defs, refs) {
        Ok(())
    } else {
        process::exit(1)
    }
}
