/// Get word form for given value
pub fn decline_for_num(value: i64, word_forms: &(&str, &str, &str)) -> String {
    let n = value.abs() % 100;

    let n1 = n % 10;

    if n > 10 && n < 20 {
        return word_forms.2.to_string()

    } else if n1 > 1 && n1 < 5 {
        return word_forms.1.to_string()

    } else if n1 == 1 {
        return word_forms.0.to_string()
    }

    word_forms.2.to_string()
}

#[cfg(test)]
mod tests {
    use crate::decline_for_num;

    #[test]
    fn first_form_tests() {
        check_form("минута", vec![19251, 329981, 5918271, 70000021])
    }

    #[test]
    fn second_form_tests() {
        check_form("минуты", vec![29373, 8234, 982562, 40283, 60784])
    }

    #[test]
    fn third_form_tests() {
        check_form("минут", vec![17, 19, 567827, 28, 738, 20000])
    }

    fn check_form(expected_form: &str, numbers: Vec<i64>) {
        let word_forms = get_word_forms();
        numbers.iter().for_each(|v| {
            let form = decline_for_num(*v, &word_forms);
            assert_eq!(expected_form, form);
        })
    }

    fn get_word_forms<'a>() -> (&'a str, &'a str, &'a str) {
        ("минута", "минуты", "минут")
    }
}
