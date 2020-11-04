pub mod Settings;
pub mod InputFormat;
pub mod OutputFormat;

// ///Acts as a placeholder "idea" for a file to deal with the pass parameters and constructir data
// //NOTE: Probably Temporary
// pub struct AbstractFile {}
#[derive(Default)]
pub struct DocumentConverter {
    settings: Settings::Settings,
    extra_options: String,
    fromFormat: String,
    toFormat: String,
    // fromFile: AbstractFile,
    // toFile: AbstractFile,
    // workingDirectory: AbstractFile, 
}
impl DocumentConverter {
    //todo return new DocumentConverter Object on each of the constructors
    pub fn newDocumentConverter(mut self) {
        let s = self.settings;
        s.set_pando_exec("pandoc");
        self.extra_options = String::new();
    }
    pub fn newWithCustomSettings(mut self, r: Settings::Settings) {
        self.settings= r.clone();
        r.set_pando_exec("pandoc");
        self.extra_options = String::new();        
    }
    pub fn from_file(format: String)-> DocumentConverter {
        let dc = DocumentConverter {
            settings: Default::default(),
            extra_options: Default::default(),
            toFormat: Default::default(),
            fromFormat: format,
        };
        return dc;
    }
pub fn to_file(format: String) -> DocumentConverter {
    let dc = DocumentConverter {
        settings: Default::default(),
        extra_options: Default::default(),
        toFormat: format,
        fromFormat: Default::default(),
    };
    return dc;   
}
pub fn add_option(opt: String) -> DocumentConverter {
    let dc = DocumentConverter {
        settings: Default::default(),
        extra_options: "".to_owned() + &opt,
        toFormat: Default::default(),
        fromFormat: Default::default(),
    };
    return dc; 
}
 pub fn convert() {
     unimplemented!()
 }   
}