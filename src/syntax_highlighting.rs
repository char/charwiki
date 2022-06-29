use std::collections::HashMap;

use chroma_syntaxis::SyntaxHighlighter;
use comrak::adapters::SyntaxHighlighterAdapter;

pub struct ChromaSyntaxisAdapter<'a, 'b>(pub &'a SyntaxHighlighter<'b>);

impl SyntaxHighlighterAdapter for ChromaSyntaxisAdapter<'_, '_> {
    fn highlight(&self, lang: Option<&str>, code: &str) -> String {
        match lang {
            Some(lang) => self.0.highlight(lang, code),
            None => code.to_string(),
        }
    }
    fn build_pre_tag(&self, _attributes: &HashMap<String, String>) -> String {
        String::from("<pre>")
    }
    fn build_code_tag(&self, attributes: &HashMap<String, String>) -> String {
        let mut s = String::from("<code");

        for (k, v) in attributes {
            s.push(' ');
            s.push_str(k);
            s.push('=');
            s.push('"');
            s.push_str(v);
            s.push('"');
        }

        s.push('>');
        s
    }
}
