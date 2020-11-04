#[derive(PartialEq)]
pub enum InputFormat {
    COMMONMARK,
    DOCBOOK,
    DOCX,
    EPUB,
    HADDOCK,
    HTML,
    JSON,
    LATEX,
    MARKDOWN,
    MARKDOWN_GITHUB,
    MARKDOWN_MMD,
    MARKDOWN_PHPEXTRA,
    MARKDOWN_STRICT,
    MEDIAWIKI,
    NATIVE,
    ODT,
    OPML,
    ORG,
    RST,
    T2T,
    TEXTILE,
    TWIKI,
}
pub struct IFormat{
    format: InputFormat,
    format_name: String
}

impl Default for InputFormat {
    fn default() -> Self {InputFormat::COMMONMARK}
}

impl IFormat {
pub fn new(f: InputFormat) -> IFormat {
   let r =  IFormat {
        format: f,
        format_name: Default::default()
   };
    return r;
}
pub fn getFormat() -> String{
    let i = IFormat {
        format: InputFormat::default(),
        format_name: Default::default()

    };
    return i.format_name;
  }

}