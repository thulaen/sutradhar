use super::read;
use crate::facts::Facts;

fn names(source: &str) -> Vec<(String, u64)> {
    let mut f = Facts::default();
    read("a.yaml", source, &mut f);
    f.of("env_declared")
        .iter()
        .filter_map(|v| {
            Some((
                v.get("name")?.as_str()?.to_owned(),
                v.get("line")?.as_u64()?,
            ))
        })
        .collect()
}

#[test]
fn map_and_list_blocks_and_outside_reads_are_recorded() {
    let source = "services:\n  a:\n    volumes:\n      - ${DIR:?x}:/d\n    \
                  environment:\n      ONE: \"1\"\n      # a note\n      \
                  TWO: \"a,\\\n        b\"\n    image: x\njobs:\n  j:\n    \
                  env:\n      - THREE=3\n    steps: []\n";
    assert_eq!(
        names(source),
        [
            ("DIR".to_owned(), 4),
            ("ONE".to_owned(), 6),
            ("TWO".to_owned(), 8),
            ("THREE".to_owned(), 14),
        ]
    );
}

#[test]
fn keys_outside_a_settings_block_are_not_settings() {
    assert_eq!(names("image: x\nports:\n  - 3000:3000\n"), []);
}
