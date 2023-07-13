/// Get word form for given value
pub fn decline_for_num(value: i32, word_forms: &(&str, &str, &str)) -> String {
    let n = value.abs() % 100;

    let n1 = n % 10;

    if n > 10 && n < 20 {
        return word_forms.2.to_string()

    } else if n1 > 1 && n1 < 5 {
        return word_forms.1.to_string()

    } else if n1 == 1 {
        return word_forms.0.to_string()
    }

    return word_forms.2.to_string()
}

#[cfg(test)]
mod tests {
    use crate::decline_for_num;

    #[test]
    fn first_form_tests() {
        let word_forms = get_word_forms();

        vec![19251, 329981, 5918271, 70000021].iter().for_each(|v| {
            let form = decline_for_num(*v, &word_forms);

            assert_eq!(word_forms.0, form);
        })
    }

    #[test]
    fn second_form_tests() {
        let word_forms = get_word_forms();

        vec![29373, 8234, 982562].iter().for_each(|v| {
            let form = decline_for_num(*v, &word_forms);

            assert_eq!(word_forms.1, form);
        })
    }

    #[test]
    fn third_form_tests() {
        let word_forms = get_word_forms();

        vec![17, 19, 567827].iter().for_each(|v| {
            let form = decline_for_num(*v, &word_forms);

            assert_eq!(word_forms.2, form);
        })
    }

    fn get_word_forms<'a>() -> (&'a str, &'a str, &'a str) {
        ("минута", "минуты", "минут")
    }
}
