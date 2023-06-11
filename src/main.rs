use docx::document::Paragraph;
use docx::document::Break;
use docx::DocxFile;

fn main() {
    println!("Hello, world!");
    let docx_file: DocxFile = DocxFile::from_file("template.docx").unwrap();
    let docx_file: docx::Docx = docxFile.parse().unwrap();

}
