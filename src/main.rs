extern crate mecab;

use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader, Write},
};

use mecab::Tagger;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut tagger = Tagger::new("");
    let mut dictionary = HashMap::new();
    let mut count = 0;
    println!("Count: {}", count);
    println!("Found: {}", dictionary.len());
    println!("Input: ");
    for result in BufReader::new(File::open("./data/corpus/AA/wiki_00")?).lines() {
        let input = result?;
        count += 1;
        if count % 100 == 0 {
            print!("\x1b[3A");
            println!("Count: {}", count);
            println!("Found: {}", dictionary.len());
            let width = 80;
            println!(
                "Input: {}",
                if input.char_indices().count() <= width {
                    &input
                } else {
                    let begin = input.char_indices().nth(0).unwrap().0;
                    let end = input.char_indices().nth(width).unwrap().0;
                    &input[begin..end]
                }
            );
        }
        let mut compounds = Vec::new();
        let mut exist_head_word = false;
        let mut skip = false;
        for node in tagger.parse_to_node(input).iter_next() {
            let features = node
                .feature
                .split(",")
                .map(String::from)
                .collect::<Vec<_>>();
            if features.len() <= 7 {
                if compounds.len() == 0 {
                    continue;
                }
                if compounds.len() > 1 && exist_head_word {
                    let compound = compounds.join("");
                    let counter = dictionary.entry(compound).or_insert(0);
                    *counter += 1;
                }
                compounds.clear();
            } else if features[0] != "名詞" && features[1] != "接頭詞" {
                skip = false;
                if compounds.len() == 0 {
                    continue;
                }
                if compounds.len() > 1 && exist_head_word {
                    let compound = compounds.join("");
                    let counter = dictionary.entry(compound).or_insert(0);
                    *counter += 1;
                }
                compounds.clear();
            } else if skip {
                continue;
            } else if features[0] == "名詞" && features[1] == "接尾" {
                let yomi = features[7].clone();
                compounds.push(yomi);
                if compounds.len() > 1 {
                    let compound = compounds.join("");
                    let counter = dictionary.entry(compound).or_insert(0);
                    *counter += 1;
                }
                compounds.clear();
            } else if features[0] == "名詞" && (features[1] == "固有名詞" || features[1] == "数")
            {
                skip = true;
                compounds.clear();
            } else if features[0] == "名詞" || features[0] == "接頭詞" {
                let yomi = features[7].clone();
                compounds.push(yomi);
                exist_head_word = features[0] == "接頭詞"
            } else {
                panic!();
            }
        }
    }
    println!("Result: {}", dictionary.len());
    let mut file = File::create("./data/compounds-20220204-2.tsv")?;
    for compound in &dictionary {
        writeln!(file, "{}\t{}", compound.0, compound.1)?;
    }
    file.flush()?;
    Ok(())
}
