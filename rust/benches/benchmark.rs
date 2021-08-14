use criterion::{black_box, criterion_group, criterion_main, Criterion};

use keycrypt::{encrypt_char, encrypt_string, decrypt_ascii_string, load_wordlist};

static BIBLE_TEXT: &str = include_str!("../../kjv-bible.txt");

pub fn encrypt_bible(c: &mut Criterion) {
    c.bench_function("encrypt bible", |b| b.iter(|| encrypt_string(black_box(BIBLE_TEXT))));
}

pub fn decrypt_bible(c: &mut Criterion) {
    let encrypted = encrypt_string(black_box(BIBLE_TEXT));
    let word_list = load_wordlist("../words.txt").unwrap();

    c.bench_function("decrypt bible", |b| b.iter(|| decrypt_ascii_string(&encrypted, &word_list)));
}

criterion_group!(benches, encrypt_bible, decrypt_bible);
criterion_main!(benches);
