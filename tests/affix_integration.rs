use std::fs;
use stringmetrics::spellcheck::Affix;

#[test]
fn affix_create_words() {
    let mut afx = Affix::new();

    let content = fs::read_to_string("tests/files/short.aff").unwrap();

    afx.load_from_str(content.as_str()).unwrap();

    assert_eq!(
        afx.create_affixed_words("xxx", "A"),
        vec!["xxx".to_string(), "rexxx".to_string()]
    );
    assert_eq!(
        afx.create_affixed_words("xxx", "N"),
        vec!["xxx".to_string(), "xxxen".to_string()]
    );
    assert_eq!(
        afx.create_affixed_words("xxx", "AN"),
        vec![
            "xxx".to_string(),
            "rexxx".to_string(),
            "xxxen".to_string(),
            "rexxxen".to_string()
        ]
    );
}