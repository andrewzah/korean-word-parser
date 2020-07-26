use std::fs::File;

use csv;
use hangeul::{get_jongseong, is_syllable};
use lazy_static::lazy_static;
use serde::Deserialize;

mod utils;
use utils::StringUtils;

lazy_static! {
    static ref PARTICLES1: Vec<&'static str> = vec![ "지", "군", "네", "는", "은", "이", "가", "에", "랑", "고", "자"];
    static ref PARTICLES2: Vec<&'static str> = vec![ "고는", "니다", "구나", "군요", "께서", "한테", "에게", "에서", "이랑", "하고", "는데"];
    static ref PARTICLES3: Vec<&'static str> = vec![ "으세요", "자마자", "습니다", ];
}


fn main() -> std::io::Result<()> {
    let file = File::open("./data/역재6 오픈 검수 - 에피소드1.csv")?;
    let mut rdr = csv::Reader::from_reader(file);


    for res in rdr.deserialize() {
        let record: Record = res?;

        let trimmed = record.translation
            .trim_matches(|c| !is_syllable(c as u32));

        let words = trimmed.split(" ").collect::<Vec<&str>>();

        for word in words {
            if word.len() == 0 {
                continue
            }

            let last = word.chars().last()
                .expect("expected a non-empty string");

            // check whole particles
            check_particle_ending(word);

            // -ㄴ
            // if let Ok(c) = get_jongseong(&last) {
            //     if c == 'ㄴ' {
            //         println!("matched ㄴ in {}!", &record.translation);
            //     }
            // }
        }
    }

    Ok(())
}

#[derive(Debug, Deserialize)]
struct Record {
    #[serde(alias = "번역")]
    translation: String,
}

fn check_particle_ending(word: &str) -> bool {
    let len = word.chars().count();
    if len == 0 {
        return false;
    }

    if len >= 3 {
        let substring: String = word.chars().skip(len-3).take(3).collect();
        println!("s3 {}: '{}'", word, substring);

        if PARTICLES3.contains(&substring.as_str()) {
            println!("{} matched p3!", substring);
            return true
        }
    }

    if len >= 2 {
        let substring: String = word.chars().skip(len-2).take(2).collect();
        println!("s2 {}: '{}'", word, substring);

        if PARTICLES2.contains(&substring.as_str()) {
            println!("{} matched p2!", substring);
            return true
        }

        let substring: String = word.chars().skip(len-1).take(1).collect();
        println!("s1 {}: '{}'", word, substring);

        if PARTICLES1.contains(&substring.as_str()) {
            println!("{} matched p1!", substring);
            return true
        }
    }

    false
}

// OUTPUT: csv with 3 cols: unique word, # of times spotted, original sentence(s)
//
// 1. filter input- remove punctuation, etc,
// 2. map into vec of words, split by space
// 3. run parse logic to strip grammatical particles
// 4. build results in memory
// 5. write to csv

fn parse(s: &str) -> Result<Vec<String>, ()> {
    Ok(vec![])
}

#[cfg(test)]
mod tests {
    use hangeul::is_syllable;
    use super::*;

    #[test]
    fn it_parses_verb_stem() {
        assert_eq!(parse("그렇습니다").unwrap(), vec!["그렇다"]);
        assert_eq!(parse("지내고").unwrap(), vec!["지내다"]);
        assert_eq!(parse("도난당했다는 것을").unwrap(), vec!["도난하다", "것"]);
        assert_eq!(parse("이루어진다").unwrap(), vec!["이루어지다"]);
        assert_eq!(parse("나가 했습니다").unwrap(), vec!["나가다", "하다"]);
        assert_eq!(parse("무서운 것").unwrap(), vec!["무섭다", "것"]);
    }

    #[test]
    fn it_parses_sentences() {
        assert_eq!(parse("영혼은 명계에서 살 것이고").unwrap(), vec!["영혼", "명계", "사다", "것"]);
    }

    #[test]
    fn it_checks_particles() {
        assert_eq!(check_particle_ending("그렇습니다"), true);
        assert_eq!(check_particle_ending("지내고"), true);
        assert_eq!(check_particle_ending("사람에"), true);

        assert_eq!(check_particle_ending("거리"), false);
        assert_eq!(check_particle_ending("사람"), false);
    }

    #[test]
    fn it_filters_particles() {
        assert_eq!(parse("거리에").unwrap(), vec!["거리"]);
        assert_eq!(parse("사람이라").unwrap(), vec!["사람"]);
    }
}
