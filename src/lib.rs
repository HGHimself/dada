use rand::thread_rng;
use rand::Rng;
use regex::Regex;

extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;

extern crate wee_alloc;
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn dada(text: &str) -> String {
    let not_words_re = Regex::new(r"[^0-9A-Za-z_']").unwrap();
    let words_re = Regex::new(r"[0-9A-Za-z_']+").unwrap();
    let template_re = Regex::new("~").unwrap();

    let words = not_words_re.split(text.clone()).collect::<Vec<_>>();
    let mut useful_words = words.into_iter().filter(|&i| i != "").collect::<Vec<_>>();

    useful_words = shuffle(useful_words);

    let mut template = words_re.replace_all(text.clone(), "~").into_owned();

    for replacement_word in useful_words {
        template = template_re
            .replace(&template, replacement_word)
            .into_owned();
    }

    template
}

fn shuffle(mut array: Vec<&str>) -> Vec<&str> {
    let mut rng = thread_rng();
    let mut i = array.len() - 1;

    while i != 0 {
        let j = rng.gen::<usize>() % (i + 1);
        let hold = array[j];
        array[j] = array[i];
        array[i] = hold;
        i -= 1;
    }
    array
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dada() {
        assert_eq!(
            dada("He don't know what you're talking about, man"),
            String::from("")
        );
    }
}
