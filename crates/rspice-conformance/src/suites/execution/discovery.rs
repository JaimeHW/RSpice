//! Deck discovery over a vendored corpus tree.

use super::*;

impl ExecutionRunner {
    /// Every deck in the corpus, as corpus-relative keys, sorted.
    ///
    /// Keys rather than absolute paths because the same string is the
    /// manifest key, the report label, and the stable identifier a failure
    /// gets filed under. Deriving all three from one value is what keeps a
    /// manifest row and a failure message referring to the same thing.
    pub fn discover(&self) -> Vec<String> {
        let mut decks = Vec::new();
        collect(&self.root, &self.root, self.corpus.deck_extensions(), &mut decks);
        decks.sort();
        decks
    }

    /// Absolute path for a corpus-relative deck key.
    pub fn deck_path(&self, key: &str) -> PathBuf {
        self.root.join(key)
    }
}

fn collect(root: &Path, dir: &Path, extensions: &[&str], out: &mut Vec<String>) {
    let Ok(entries) = std::fs::read_dir(dir) else {
        return;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            collect(root, &path, extensions, out);
            continue;
        }
        let is_deck = path
            .extension()
            .and_then(|ext| ext.to_str())
            .is_some_and(|ext| extensions.iter().any(|want| ext.eq_ignore_ascii_case(want)));
        if !is_deck {
            continue;
        }
        if let Ok(relative) = path.strip_prefix(root) {
            out.push(relative.to_string_lossy().replace('\\', "/"));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn temp_dir(tag: &str) -> PathBuf {
        let unique = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("system time after the Unix epoch")
            .as_nanos();
        std::env::temp_dir().join(format!("rspice_execution_{tag}_{unique}"))
    }

    /// Model cards and stimulus files sit beside decks and must not be run.
    #[test]
    fn discovery_selects_deck_extensions_and_recurses() {
        let tests_dir = temp_dir("discovery");
        let root = tests_dir.join("paranoia");
        std::fs::create_dir_all(root.join("nested")).expect("create corpus fixture");
        for (name, body) in [
            ("top.cir", "top\n.end\n"),
            ("nested/inner.sp", "inner\n.end\n"),
            ("nested/deck.deck", "deck\n.end\n"),
            ("models.lib", ".subckt m a\n.ends\n"),
            ("stimulus.stim", "0 0\n"),
        ] {
            std::fs::write(root.join(name), body).expect("write corpus fixture");
        }

        let runner = ExecutionRunner::new(
            ExecutionCorpus::Paranoia,
            &tests_dir,
            ExecutionConfig::default(),
        );

        assert_eq!(
            runner.discover(),
            vec![
                "nested/deck.deck".to_string(),
                "nested/inner.sp".to_string(),
                "top.cir".to_string(),
            ]
        );

        std::fs::remove_dir_all(tests_dir).expect("remove corpus fixture");
    }

    #[test]
    fn manifest_entries_without_decks_are_reported_as_orphans() {
        let tests_dir = temp_dir("orphans");
        let root = tests_dir.join("iscas85");
        std::fs::create_dir_all(&root).expect("create corpus fixture");
        std::fs::write(root.join("live.net"), "live\n.end\n").expect("write deck");
        std::fs::write(
            root.join("execution-manifest.tsv"),
            "live.net\texpected_refusal\tadjudicated\ngone.net\texpected_unsupported\tstale\n",
        )
        .expect("write manifest");

        let runner = ExecutionRunner::new(
            ExecutionCorpus::Iscas85,
            &tests_dir,
            ExecutionConfig::default(),
        );

        assert_eq!(runner.orphaned_manifest_entries(), vec!["gone.net"]);

        std::fs::remove_dir_all(tests_dir).expect("remove corpus fixture");
    }
}
