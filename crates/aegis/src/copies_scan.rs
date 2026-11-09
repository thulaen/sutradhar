//! The search behind `copies`. First every table is squeezed: a
//! stretch of at least `least` tokens that is one short piece written
//! again and again, names and all (a list of like entries, or the same
//! line said many times), keeps only its first piece. So tables of
//! different lengths line up, and none can hide or join a copy. Tables
//! are found on the tokens as written, so lines of one shape with their
//! own names, which are code, are kept whole. Then every run of `least`
//! tokens is kept in a map, and each place a run is seen again is grown
//! to the longest shared run. A run is reported only where it starts,
//! so each copy is found once.

use std::collections::HashMap;

use super::{Copied, Token};

/// One file: its path and its tokens.
type File<'a> = (&'a str, Vec<Token>);

/// A place in the files: which file, and which token in it.
#[derive(Clone, Copy)]
struct At {
    file: usize,
    token: usize,
}

/// How far the runs at `old` (in `before`) and `new` (in `after`) stay
/// equal. In one file the first run stops where the second begins.
fn shared(before: &[u32], old: At, after: &[u32], new: At) -> usize {
    let same = old.file == new.file;
    let mut len = 0_usize;
    loop {
        let here = old.token.saturating_add(len);
        if same && here >= new.token {
            return len;
        }
        let there = new.token.saturating_add(len);
        match (before.get(here), after.get(there)) {
            (Some(one), Some(two)) if one == two => len = len.saturating_add(1),
            _ => return len,
        }
    }
}

/// Whether the runs at `old` and `new` start there: the tokens just
/// before them differ, or one is at the start of its file.
fn starts(before: &[u32], old: usize, after: &[u32], new: usize) -> bool {
    match (old.checked_sub(1), new.checked_sub(1)) {
        (Some(one), Some(two)) => before.get(one) != after.get(two),
        _ => true,
    }
}

/// The file's tokens with every table squeezed: each stretch of at
/// least `least` tokens in which every token as written equals the one
/// `step` places on, for a `step` under a fifth of `least`, keeps only
/// its first `step` tokens.
fn squeezed(file: &[Token], least: usize) -> Vec<Token> {
    let written: Vec<u32> = file.iter().map(|t| t.1).collect();
    let mut gone = vec![false; file.len()];
    for step in 1..least / 5 {
        let mut from = 0_usize;
        for at in 0..=written.len() {
            let more = at.checked_add(step).and_then(|n| written.get(n));
            if more.is_some() && written.get(at) == more {
                continue;
            }
            // The stretch from `from` repeats with this step up to `end`.
            let end = at.saturating_add(step).min(written.len());
            if end.saturating_sub(from) >= least {
                let first = from.saturating_add(step);
                gone.iter_mut()
                    .take(end)
                    .skip(first)
                    .for_each(|g| *g = true);
            }
            from = at.saturating_add(1);
        }
    }
    file.iter()
        .zip(gone)
        .filter_map(|(t, g)| (!g).then_some(*t))
        .collect()
}

/// Where a run is: its file's path and its first and last lines.
fn place(file: &File<'_>, from: usize, len: usize) -> (String, usize, usize) {
    let line = |i: usize| file.1.get(i).map_or(0, |t| t.2);
    let last = from.saturating_add(len).saturating_sub(1);
    (file.0.to_owned(), line(from), line(last))
}

/// The longest copy starting at `new` among the earlier `places` of the
/// same run: the earlier place and the length.
fn longest(ids: &[Vec<u32>], places: &[At], new: At) -> Option<(At, usize)> {
    let after = ids.get(new.file)?;
    places
        .iter()
        .filter_map(|&old| {
            let before = ids.get(old.file)?;
            starts(before, old.token, after, new.token)
                .then(|| (old, shared(before, old, after, new)))
        })
        .max_by_key(|found| found.1)
}

/// Every copy of at least `least` tokens among `files`, looking at no
/// more than `most` earlier places for any one run. For each place only
/// the longest earlier copy is kept, and a place inside a copy already
/// found starts no new one.
pub fn scan(files: &[File<'_>], least: usize, most: usize) -> Vec<Copied> {
    let files: Vec<File<'_>> =
        files.iter().map(|f| (f.0, squeezed(&f.1, least))).collect();
    let ids: Vec<Vec<u32>> = files
        .iter()
        .map(|f| f.1.iter().map(|t| t.0).collect())
        .collect();
    let mut seen: HashMap<&[u32], Vec<At>> = HashMap::new();
    let mut out = Vec::new();
    for (file, mine) in ids.iter().enumerate() {
        let mut covered_to = 0_usize;
        for token in 0..mine.len() {
            let Some(run) = mine.get(token..token.saturating_add(least)) else {
                break;
            };
            let new = At { file, token };
            let places = seen.entry(run).or_default();
            let found = (token >= covered_to)
                .then(|| longest(&ids, places, new))
                .flatten()
                .filter(|found| found.1 >= least);
            if places.len() < most {
                places.push(new);
            }
            let Some((old, len)) = found else { continue };
            covered_to = token.saturating_add(len);
            if let (Some(first), Some(second)) =
                (files.get(old.file), files.get(file))
            {
                out.push(Copied {
                    first: place(first, old.token, len),
                    second: place(second, token, len),
                    tokens: len,
                });
            }
        }
    }
    out
}
