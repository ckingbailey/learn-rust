/*
Translate phrase to pig latin.
Pig Latin rules:
    If a word starts with a consonants
        • remove the consonant from the front of the word
        • append to the end of the word and append "ay"
    If a word starts with a vowel
        • append "hay" to the end of the word

Iterate over each letter, `c`, of `phrase`:
    If `word_start` is true, compose `pig_end`:
        If `c` is vowel, c = 'h'
        Append "ay"
        Store as `pig_end`
    If `c` is whitespace:
        Set `word_start` to true
        Append `pig_end` to `translated`
        Reset `pig_end` to empty string
    Else append `char` to `translated`
Append pig_end
*/
fn pig_latinify(phrase: &str) -> String {
    let mut translated = String::from("");

    let vowels = "aeiou";
    let mut word_start = true;
    let mut cap = false;
    let mut pig_end = String::from("");

    for mut c in phrase.chars() {
        if word_start {
            if vowels.contains(c) {
                if pig_end == "" {
                    cap = if c.is_ascii_uppercase() { true } else { false };
                    pig_end = 'h'.to_string();
                    word_start = false;
                } else {
                    c = if cap { c.to_ascii_uppercase() } else { c.to_ascii_lowercase() };
                    translated.push(c);
                    word_start = false;
                }
            } else {
                if pig_end == "" {
                    cap = if c.is_ascii_uppercase() { true } else { false };
                }
                pig_end.push(c.to_ascii_lowercase());
            }
        } else if c == ' ' {
            translated = append_pig_end(&pig_end, translated);
            translated.push(c);
            pig_end = String::from("");
            word_start = true;
        } else {
            translated.push(c.to_ascii_lowercase());
        }
    }

    translated = append_pig_end(&pig_end, translated);
    translated
}

fn append_pig_end(pig_end: &String, mut output: String) -> String {
    output.push_str(pig_end);
    output.push_str("ay");
    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn append_b_to_a() {
        let appended = append_pig_end(&String::from("b"), String::from("a"));
        let expect_appended = "abay";
        assert!(
            appended == expect_appended,
            "Expected \"{}\", got \"{}\"",
            expect_appended, appended
        )
    }

    #[test]
    fn a_simple_case() {
        let phrase = "beans for dinner tonight";

        let expect_pig = "eansbay orfay innerday onighttay";
        let pigged = pig_latinify(phrase);

        assert!(
            pigged == expect_pig,
            "Expected phrase \"{}\" to be translated to \"{}\", got \"{}\"",
            phrase, expect_pig, pigged
        )
    }

    #[test]
    fn starts_with_consonants() {
        let phrase = "The way to San Jose";

        let expect_pig = "Ethay ayway otay Ansay Osejay";
        let pigged = pig_latinify(phrase);

        assert!(
            pigged == expect_pig,
            "Expected phrase \"{}\" to be translated to \"{}\", got \"{}\"",
            phrase, expect_pig, pigged
        );
    }
}
