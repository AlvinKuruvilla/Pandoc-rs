#[derive(PartialEq)]
pub enum OutputFormat {
    ASCIIDOC,
    BEAMER,
    COMMONMARK,
    CONTEXT,
    DOCBOOK,
    DOCX,
    DOKUWIKI,
    DZSLIDES,
    EPUB,
    EPUB3,
    FB2,
    HADDOCK,
    HTML,
    HTML5,
    ICML,
    JSON,
    LATEX,
    MAN,
    MARKDOWN,
    MARKDOWN_GITHUB,
    MARKDOWN_MMD,
    MARKDOWN_PHPEXTRA,
    MARKDOWN_STRICT,
    MEDIAWIKI,
    NATIVE,
    ODT,
    OPENDOCUMENT,
    OPML,
    ORG,
    PDF,
    PLAIN,
    REVEALJS,
    RST,
    RTF,
    S5,
    SLIDEOUS,
    SLIDY,
    TEXINFO,
    TEXTILE
}
pub struct OFormat{
    format: OutputFormat,
    format_name: String
}

impl Default for OutputFormat {
    fn default() -> Self {OutputFormat::DOCX}
}

impl OFormat {
pub fn new(f: OutputFormat) -> OFormat {
   let r =  OFormat {
        format: f,
        format_name: Default::default()
   };
    return r;
}

pub fn getFormat() -> String{
    let i = OFormat {
        format: OutputFormat::default(),
        format_name: Default::default()
    };
    return i.format_name;
  }

}