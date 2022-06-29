use once_cell::sync::Lazy;
use regex::Regex;
use sanitize_html::{
    errors::SanitizeError,
    rules::{pattern::Pattern, Element, Rules},
    sanitize_str,
};

static RULES: Lazy<Rules> = Lazy::new(generate_rules);

pub fn sanitize_content(content: &str) -> Result<String, SanitizeError> {
    sanitize_str(&RULES, content)
}

fn generate_rules() -> Rules {
    // Long function ahead :)

    fn re(regex: &str) -> Pattern {
        Pattern::regex(Regex::new(regex).unwrap())
    }

    fn href() -> Pattern {
        re("^(ftp:|http:|https:|mailto:)") | !re("^[^/]+[[:space:]]*:")
    }

    fn src() -> Pattern {
        re("^(http:|https:)") | !re("^[^/]+[[:space:]]*:")
    }

    fn relaxed_element(name: &str) -> Element {
        Element::new(name)
            .attribute("dir", Pattern::any())
            .attribute("lang", Pattern::any())
            .attribute("title", Pattern::any())
            .attribute("class", Pattern::any())
    }

    Rules::new()
        .allow_comments(true)
        .element(relaxed_element("a").attribute("href", href()))
        .element(relaxed_element("abbr"))
        .element(relaxed_element("address"))
        .element(relaxed_element("article"))
        .element(relaxed_element("aside"))
        .element(relaxed_element("b"))
        .element(relaxed_element("bdo"))
        .element(relaxed_element("blockquote").attribute("cite", src()))
        .element(relaxed_element("br"))
        .element(relaxed_element("caption"))
        .element(relaxed_element("cite"))
        .element(relaxed_element("code"))
        .element(
            relaxed_element("col")
                .attribute("span", Pattern::any())
                .attribute("width", Pattern::any()),
        )
        .element(
            relaxed_element("colgroup")
                .attribute("span", Pattern::any())
                .attribute("width", Pattern::any()),
        )
        .element(relaxed_element("dd"))
        .element(
            relaxed_element("del")
                .attribute("cite", src())
                .attribute("datetime", Pattern::any()),
        )
        .element(relaxed_element("dfn"))
        .element(relaxed_element("dl"))
        .element(relaxed_element("dt"))
        .element(relaxed_element("em"))
        .element(relaxed_element("figcaption"))
        .element(relaxed_element("figure"))
        .element(relaxed_element("footer"))
        .element(relaxed_element("h1"))
        .element(relaxed_element("h2"))
        .element(relaxed_element("h3"))
        .element(relaxed_element("h4"))
        .element(relaxed_element("h5"))
        .element(relaxed_element("h6"))
        .element(relaxed_element("header"))
        .element(relaxed_element("hgroup"))
        .element(relaxed_element("hr"))
        .element(relaxed_element("i"))
        .element(
            relaxed_element("img")
                .attribute("src", src())
                .attribute("align", Pattern::any())
                .attribute("alt", Pattern::any())
                .attribute("width", Pattern::any())
                .attribute("height", Pattern::any()),
        )
        .element(
            relaxed_element("ins")
                .attribute("cite", src())
                .attribute("datetime", Pattern::any()),
        )
        .element(relaxed_element("kbd"))
        .element(relaxed_element("li"))
        .element(relaxed_element("mark"))
        .element(relaxed_element("nav"))
        .element(
            relaxed_element("ol")
                .attribute("start", Pattern::any())
                .attribute("reversed", Pattern::any())
                .attribute("type", Pattern::any()),
        )
        .element(relaxed_element("p"))
        .element(relaxed_element("pre"))
        .element(relaxed_element("q").attribute("cite", src()))
        .element(relaxed_element("rp"))
        .element(relaxed_element("rt"))
        .element(relaxed_element("ruby"))
        .element(relaxed_element("s"))
        .element(relaxed_element("samp"))
        .element(relaxed_element("section"))
        .element(relaxed_element("small"))
        .element(relaxed_element("span"))
        .element(relaxed_element("strike"))
        .element(relaxed_element("strong"))
        .element(relaxed_element("sub"))
        .element(relaxed_element("sup"))
        .element(
            relaxed_element("table")
                .attribute("summary", Pattern::any())
                .attribute("width", Pattern::any()),
        )
        .element(relaxed_element("tbody"))
        .element(
            relaxed_element("td")
                .attribute("abbr", Pattern::any())
                .attribute("axis", Pattern::any())
                .attribute("colspan", Pattern::any())
                .attribute("rowspan", Pattern::any())
                .attribute("width", Pattern::any()),
        )
        .element(relaxed_element("tfoot"))
        .element(
            relaxed_element("th")
                .attribute("abbr", Pattern::any())
                .attribute("axis", Pattern::any())
                .attribute("colspan", Pattern::any())
                .attribute("rowspan", Pattern::any())
                .attribute("scope", Pattern::any())
                .attribute("width", Pattern::any()),
        )
        .element(relaxed_element("thead"))
        .element(
            relaxed_element("time")
                .attribute("datetime", Pattern::any())
                .attribute("pubdate", Pattern::any()),
        )
        .element(relaxed_element("tr"))
        .element(relaxed_element("u"))
        .element(relaxed_element("ul").attribute("type", Pattern::any()))
        .element(relaxed_element("var"))
        .element(relaxed_element("wbr"))
}
