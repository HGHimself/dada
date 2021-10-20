use rand::seq::SliceRandom;
use rand::thread_rng;
use regex::Regex;

pub fn dada(text: &str) -> String {
    let not_words_re = Regex::new(r"\W").unwrap();
    let words_re = Regex::new(r"\w+").unwrap();
    let template_re = Regex::new("~").unwrap();

    let words = not_words_re.split(text.clone()).collect::<Vec<_>>();
    let mut useful_words = words.into_iter().filter(|&i| i != "").collect::<Vec<_>>();

    let mut rng = thread_rng();
    useful_words.shuffle(&mut rng);

    let mut template = words_re.replace_all(text.clone(), "~").into_owned();

    for replacement_word in useful_words {
        template = template_re.replace(&template, replacement_word).into_owned();
    }

    template
}
