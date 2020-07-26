use std::fs::File;

use csv;
use hangeul::{get_jongseong, is_syllable};
use lazy_static::lazy_static;
use serde::Deserialize;

mod utils;
use utils::StringUtils;

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
        }
    }

    Ok(())
}

#[derive(Debug, Deserialize)]
struct Record {
    #[serde(alias = "번역")]
    translation: String,
}

// OUTPUT: csv with cols: unique word, # of times spotted, original sentence(s), definition (if found), hanja (if found)
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
    use super::*;

    #[test]
    fn it_parses_verb_stem() {
        assert_eq!(parse("받으세요").unwrap(), vec!["받다"]);
        assert_eq!(parse("그렇습니다").unwrap(), vec!["그렇다"]);
        assert_eq!(parse("지내고").unwrap(), vec!["지내다"]);
        assert_eq!(parse("도난당했다는 것을").unwrap(), vec!["도난하다"]);
        assert_eq!(parse("이루어진다").unwrap(), vec!["이루어지다"]);
        assert_eq!(parse("무서운 것").unwrap(), vec!["무섭다"]);
    }

    #[test]
    fn it_parses_sentences() {
        assert_eq!(parse("영혼은 명계에서 살 것이고").unwrap(), vec!["영혼", "명계", "사다"]);
    }

    #[test]
    fn it_filters_particles() {
        assert_eq!(parse("거리에").unwrap(), vec!["거리"]);
        assert_eq!(parse("사람이라").unwrap(), vec!["사람"]);
    }
}
