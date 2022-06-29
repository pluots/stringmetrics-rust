//! A dictionary contains methods and a list of Entries
//! Load hunspell dicts
//! http://pwet.fr/man/linux/fichiers_speciaux/hunspell/

use crate::spellcheck::affix::Affix;
use core::hash::Hash;
use std::collections::HashSet;

/// This is the main object used for spellchecking
///
/// A dictionary contains
pub struct Dictionary {
    /// This contains the dictionary's configuration
    pub affix: Affix,

    // General word list
    wordlist: HashSet<String>,
    // Words to accept but never suggest
    wordlist_nosuggest: HashSet<String>,
    // Words forbidden by the personal dictionary, i.e. do not accept as correct
    wordlist_forbidden: HashSet<String>,

    // These hold the files as loaded
    // Will be emptied upon compile
    raw_wordlist: Vec<String>,
    raw_wordlist_personal: Vec<String>,
    // Indicator of whether or not this has been compiled
    compiled: bool,
}

impl Dictionary {
    pub fn new() -> Dictionary {
        Dictionary {
            affix: Affix::new(),
            wordlist: HashSet::new(),
            wordlist_nosuggest: HashSet::new(),
            wordlist_forbidden: HashSet::new(),
            raw_wordlist: Vec::new(),
            raw_wordlist_personal: Vec::new(),
            compiled: false,
        }
    }

    /// Can also be done with strings
    pub fn load_affix_from_str(&mut self, s: &str) -> Result<(), String> {
        self.compiled = false;
        self.affix.load_from_str(s)
    }

    pub fn load_dictionar_from_str(&mut self, s: &str) {
        self.compiled = false;

        let mut lines = s.lines();
        // First line is just a count of the number of items
        let _first = lines.next();
        self.raw_wordlist = lines.map(|l| l.to_string()).collect()
    }
    pub fn load_personal_dict_from_str(&mut self, s: &str) {
        self.compiled = false;

        self.raw_wordlist_personal = s.lines().map(|l| l.to_string()).collect()
    }

    /// Match affixes, personal dict, etc
    pub fn compile(&mut self) -> Result<(), String> {
        // Work through the personal word list
        for word in self.raw_wordlist_personal.iter() {
            // Words will be in the format "*word/otherword" where "word" is the
            // main word to add, but it will get all rules of "otherword"
            let split: Vec<&str> = word.split('/').collect();
            let forbidden = word.starts_with('*');

            match split.get(1) {
                Some(rootword) => {
                    // Find "otherword/" in main wordlist
                    let mut tmp = rootword.to_string();
                    tmp.push('/');
                    let filtval = tmp.trim_start_matches("*");

                    match self
                        .raw_wordlist
                        .iter()
                        .filter(|s| s.starts_with(&filtval))
                        .next()
                    {
                        Some(w) => (),
                        None => return Err("Root word not found".to_string()),
                    }
                }
                None => (),
            }
        }

        for word in self.raw_wordlist.iter() {
            let split: Vec<&str> = word.split('/').collect();
            let rootword = split[0];
            match split.get(1) {
                Some(rule_keys) => {
                    let wordlist = self.affix.create_affixed_words(rootword, rule_keys);
                    match rule_keys.contains(&self.affix.nosuggest_flag) {
                        true => iter_to_hashset(wordlist, &mut self.wordlist_nosuggest),
                        false => iter_to_hashset(wordlist, &mut self.wordlist),
                    }
                }
                None => {
                    self.wordlist.insert(rootword.to_string());
                }
            }
        }

        self.compiled = true;

        Ok(())
    }

    /// Check that a word is spelled correctly. Returns true if so
    ///
    /// This is the main spellchecking feature. It checks a single word for
    /// validity according to the loaded dictionary. This accepts any
    /// string-like type including `&str`, `String` and `&String`
    ///
    /// # Panics
    ///
    /// This will panic if the dictionary has not yet been compiled.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::fs;
    /// use stringmetrics::spellcheck::Dictionary;
    ///
    /// let mut dic = Dictionary::new();
    ///
    /// let aff_content = fs::read_to_string("tests/files/short.aff").unwrap();
    /// let dic_content = fs::read_to_string("tests/files/short.dic").unwrap();
    ///
    /// dic.affix.load_from_str(aff_content.as_str()).unwrap();
    /// dic.load_dictionar_from_str(dic_content.as_str());
    /// dic.compile().unwrap();
    ///
    /// assert_eq!(dic.check("yyication"), true);
    /// ```
    pub fn check<T: AsRef<str>>(&self, s: T) -> bool {
        // We actually just need to check
        self.break_if_not_compiled();

        self.check_no_break(s)
    }

    // Private function that checks a single word. Same as check() but doesn't
    // validate this dictionary is compiled
    fn check_no_break<T: AsRef<str>>(&self, s: T) -> bool {
        // Convert to a usable string reference
        let sref = s.as_ref();

        // Must not be in a forbidden word list
        // Note that in the future this implementation might change
        (!self.wordlist_forbidden.contains(sref))
            // And one of the "exists" wordlists contains the word
            && (self.wordlist.contains(sref) || self.wordlist_nosuggest.contains(sref))
    }

    /// Create a sorted vector of all items in the word list
    ///
    /// Note that this is relatively slow. Prefer [`check`] for validating a word
    /// exists.
    pub fn wordlist_items(&self) -> Vec<&str> {
        self.break_if_not_compiled();

        let mut items = self
            .wordlist
            .iter()
            .map(|s| s.as_str())
            .collect::<Vec<&str>>();
        items.sort();
        items
    }

    /// Helper function to error if we haven't compiled when we needed to
    fn break_if_not_compiled(&self) {
        assert!(
            self.compiled == true,
            "This method requires compiling the dictionary with `dic.compile()` first."
        )
    }
}

/// Apply affix rules to a given root word, based on what tokens it provides
fn generate_wordlist_from_afx(rootword: &str, tokens: &str, affix: &Affix) -> Vec<String> {
    for rule in &affix.affix_rules {
        if tokens.contains(&rule.ident) {}
    }
    Vec::new()
}

fn iter_to_hashset<I, T>(items: I, hs: &mut HashSet<T>)
where
    I: IntoIterator<Item = T>,
    T: Eq + Hash,
{
    for item in items {
        hs.insert(item);
    }
}
