use getrandom;
use regex::Regex;

extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;

extern crate wee_alloc;
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn dada(text: &str) -> String {
    if text == "" {
        return "".to_string();
    }

    let not_words_re = Regex::new(r"[^0-9A-Za-z_']").unwrap();
    let words_re = Regex::new(r"[0-9A-Za-z_']+").unwrap();
    let template_re = Regex::new("~").unwrap();

    let words = not_words_re.split(text.clone()).collect::<Vec<_>>();
    let mut useful_words = words.into_iter().filter(|&i| i != "").collect::<Vec<_>>();

    shuffle(&mut useful_words);

    let mut template = words_re.replace_all(text.clone(), "~").into_owned();

    for replacement_word in useful_words {
        template = template_re
            .replace(&template, replacement_word)
            .into_owned();
    }

    template
}

fn shuffle(array: &mut Vec<&str>) -> () {
    for i in (1..array.len()).rev() {
        let r = random().unwrap();
        let j = r % (i + 1);
        array.swap(i, j);
    }
}

fn random() -> Result<usize, getrandom::Error> {
    let mut buf = [0u8; 1];
    getrandom::getrandom(&mut buf)?;
    Ok(buf[0] as usize)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shuffle() {
        let mut v = vec!["1", "2", "3", "4", "5", "6", "7"];
        shuffle(&mut v);
        assert_ne!(v, vec!["1", "2", "3", "4", "5", "6", "7"]);

        let mut e = vec!["1", "2", "3", "4", "5", "6", "7"];
        let mut c = vec!["1", "2", "3", "4", "5", "6", "7"];
        shuffle(&mut e);
        shuffle(&mut c);
        assert_ne!(e, c);
    }

    #[test]
    fn test_dada() {
        assert_ne!(
            dada("He don't know what you're talking about, man"),
            String::from("He don't know what you're talking about, man")
        );

        assert_ne!(
            dada("He don't know what you're talking about, man"),
            dada("He don't know what you're talking about, man")
        );

        assert_ne!(
            dada("He don't know what you're talking about, man"),
            String::from("")
        );

        assert_eq!(dada(""), String::from(""));
    }
}
