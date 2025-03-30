pub mod oxc {
    use std::path::Path;

    use oxc::{allocator::Allocator, parser::Parser, span::SourceType};

    pub fn parse(path: &Path, source: &str) -> Allocator {
        let allocator = Allocator::default();
        let source_type = SourceType::from_path(path).unwrap();
        _ = Parser::new(&allocator, source, source_type).parse();
        allocator
    }
}

pub mod swc {
    use std::path::Path;

    use swc_ecma_ast::Module;
    use swc_ecma_parser::{EsSyntax, Parser, StringInput, Syntax, TsSyntax};

    pub fn parse(path: &Path, source: &str) -> Module {
        let syntax = match path.extension().unwrap().to_str().unwrap() {
            "js" => Syntax::Es(EsSyntax::default()),
            "tsx" => Syntax::Typescript(TsSyntax { tsx: true, ..TsSyntax::default() }),
            _ => panic!("need to define syntax  for swc"),
        };
        let input = StringInput::new(source, Default::default(), Default::default());
        Parser::new(syntax, input, None).parse_module().unwrap()
    }
}

pub mod brimstone {
    use std::{path::Path, rc::Rc};

    use brimstone::js::{
        common::{options::OptionsBuilder, wtf_8::Wtf8String},
        parser::{parse_script, source::Source, ParseContext},
    };

    pub fn parse(_: &Path, source: &str) -> ParseContext {
        let options = Rc::new(OptionsBuilder::new().annex_b(true).build());
        let source =
            Rc::new(Source::new_for_eval(String::new(), Wtf8String::from_str(source)).unwrap());
        let pcx = ParseContext::new(source);

        let _ = parse_script(&pcx, options).unwrap();

        pcx
    }
}
