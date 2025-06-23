use case::CaseExt;
use edit_distance::edit_distance;

pub fn expected_variable(compare: &str, expected: &str) -> Option<String> {
    let compare_aux = compare.to_lowercase();
    let expected_aux = expected.to_lowercase();

    if (compare_aux.to_ascii_lowercase() == compare_aux
        || compare_aux.to_camel_lowercase() == compare_aux)
        && !compare_aux.contains("-")
        && !compare_aux.contains(" ")
    {
        let distance = edit_distance(&compare_aux, &expected_aux) as i64;

        if distance == 0 {
            return Some("100%".to_string());
        }

        let percentage = 100 - (distance * 100 / expected.len() as i64);
        if percentage >= 50 {
            let mut res = percentage.to_string();
            res.push_str("%");
            return Some(res);
        }
    }

    None
}