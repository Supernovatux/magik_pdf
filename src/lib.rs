pub mod poppler_interface;

#[cfg(test)]
mod tests {
    /*
    use tokio::runtime::Builder;

    use super::poppler_interface::OutputType;
    use super::poppler_interface::PopperTools;
    use super::poppler_interface::PopplerInterface;
    async fn it_works() {
        assert!(true);
        let tools = PopperTools::new();
        if !tools.is_tool_present().unwrap().is_dir() {
            tools.download_magik().unwrap();
        }
        let pdfs = vec![
            r"C:\Users\thula\Downloads\pdf24_images_merged.pdf",
            r"C:\Users\thula\Downloads\bank-cash-flow-statement-template.pdf",
            r"C:\Users\thula\Downloads\customer-bank-statement-template.pdf",
            r"C:\Users\thula\Downloads\Industry-Standard-Financial-Statement-Template.pdf",
            r"C:\Users\thula\Downloads\monthly-bank-statement-template.pdf",
            r"C:\Users\thula\Downloads\NATIONAL PARTNERSHIP FOR QUALITY AFTERSCHOOL LEARNING.pdf",
            r"C:\Users\thula\Downloads\Personal-Bank-Statement-Template.pdf",
            r"C:\Users\thula\Downloads\personal-financial-statement-form.pdf",
            r"C:\Users\thula\Downloads\data table 1.pdf",
            r"C:\Users\thula\Downloads\data table.pdf",
            r"C:\Users\thula\Downloads\DOC270.pdf",
            r"C:\Users\thula\Downloads\DOC327.pdf",
            r"C:\Users\thula\Downloads\images_merged.pdf",
            r"C:\Users\thula\Downloads\Scanned PDF tables-cropped.pdf",
            r"C:\Users\thula\Downloads\safepdfkit.pdf",
            r"C:\Users\thula\Downloads\table.pdf",
        ];
        for i in pdfs {
            let out = tools.convert_to_image_single_page(i, OutputType::JPEGCMKY,2, Some(vec!["-r","300"])).unwrap();
            println!("{:?}", out);
            //tools.delete_files(out).unwrap();
        }
    }
    #[test]
    fn async2() {
    let runtime = Builder::new_multi_thread()
        .worker_threads(4)
        .thread_name("Popper Download")
        .thread_stack_size(3 * 1024 * 1024)
        .enable_time()
        .enable_io()
        .build().unwrap();
    let _ =runtime.block_on(it_works());
    panic!()
    }
    */
}
