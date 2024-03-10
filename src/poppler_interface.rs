use directories::ProjectDirs;
use futures_util::StreamExt;
//use rand::Rng;
use anyhow::{Context, Result};
use std::error::Error;
use std::fmt::Debug;
use std::fs;
use std::fs::create_dir_all;
use std::fs::remove_file;
use std::path::Path;
use std::path::PathBuf;
use std::process::Command;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use tokio::runtime::Builder;
use zip_extensions::zip_extract;

const MAGIC_URL: &str =
    "https://github.com/oschwartz10612/poppler-windows/releases/download/v24.02.0-0/Release-24.02.0-0.zip";
#[derive(Default)]
pub enum OutputType {
    PPM,
    JPG,
    JPEGCMKY,
    #[default]
    PNG,
    TIFF,
}
impl OutputType {
    fn get_formatter(&self) -> String {
        match self {
            OutputType::PPM => "ppm",
            OutputType::JPG => "jpg",
            OutputType::JPEGCMKY => "jpg",
            OutputType::TIFF => "tif",
            OutputType::PNG => "png",
        }
        .to_string()
    }
    fn get_arg(&self) -> String {
        match self {
            OutputType::PPM => "-q",
            OutputType::JPG => "-jpeg",
            OutputType::JPEGCMKY => "-jpegcmyk",
            OutputType::TIFF => "-tiff",
            OutputType::PNG => "-png",
        }
        .to_string()
    }
}
/// An interface for using poppler
///
/// # Example
/// ```ignore
/// use super::poppler_interface::PopperTools;
/// use super::poppler_interface::PopplerInterface;
/// let tools = PopperTools::new();
///  if !tools.is_tool_present().unwrap().is_dir() {
///      tools.download_magik().unwrap();
///  }
///  let out =
///      tools.convert_to_image(r"C:\Users\thula\Downloads\sponsorship.pdf", vec!["-png"]).unwrap();
///  println!("{:?}", out);
///  // Out is a vector of png image paths, use as fit
///  tools.delete_files(out).unwrap();
///  let result = 4;
///  assert_eq!(result, 2);
///
///
/// ```
pub struct PopperTools;
impl PopperTools {
    pub fn new() -> PopperTools {
        PopperTools {}
    }
}
impl PopplerInterface for PopperTools {}
impl Default for PopperTools {
    fn default() -> Self {
        Self::new()
    }
}
pub trait PopplerInterface {
    /// Checks if [Popper](https://github.com/oschwartz10612/poppler-windows) pdf handling tool is present. Returns `Ok(PathBuf)` is path popper is present, else returns an `Err`
    fn is_tool_present(&self) -> Result<PathBuf, Box<dyn Error>> {
        let mut path = get_file_path()?;
        std::fs::create_dir_all(path.clone())?;
        path.push("poppler-24.02.0");
        Ok(path)
    }
    /// Downloads and extraction Popper to app data dir. Returns `Ok(())` if successful
    fn download_magik(&self) -> Result<(), Box<dyn Error>> {
        let path = get_file_path()?;
        let runtime = Builder::new_multi_thread()
            .worker_threads(4)
            .thread_name("Popper Download")
            .thread_stack_size(3 * 1024 * 1024)
            .enable_time()
            .enable_io()
            .build()?;
        let mut magic_zip_path = path.clone();
        magic_zip_path.push("popper.zip");
        runtime.block_on(download_files(MAGIC_URL, &magic_zip_path))?;
        zip_extract(&magic_zip_path, &path)?;
        fs::remove_file(magic_zip_path)?;
        Ok(())
    }
    /// Can delete all the files returned by `convert_to_image`. Returns a `Result`
    fn delete_files(&self, files: Vec<impl Into<PathBuf>>) -> Result<(), Box<dyn Error>> {
        for i in files {
            remove_file(i.into())?;
        }
        Ok(())
    }
    /// Uses popper to convert the given PDF to image and returns a `Vec<Pathbuf>` pointing to various files generated.
    /// Note that atleast vec!["-png"] is needed for the generation to work
    /// # Example
    /// ```ignore
    /// let out =  tools.convert_to_image(r"C:\Users\thula\Downloads\sponsorship.pdf", vec!["-png"]).unwrap();
    /// println!("{:?}", out);
    /// tools.delete_files(out).unwrap();
    /// ```
    fn convert_to_image(
        &self,
        pdf_path: impl Into<PathBuf> + std::convert::AsRef<std::ffi::OsStr>,
        out_type: OutputType,
        args: Option<Vec<&str>>,
    ) -> Result<Vec<impl Into<PathBuf> + Debug>, Box<dyn Error>> {
        let mut path = Self::is_tool_present(self)?;
        path.push("Library\\bin\\pdftoppm.exe");
        println!("{}", path.is_file());
        //let mut num = rand::thread_rng().gen_range(0..100000);
        let mut cache_path = get_cache_path()?;
        create_dir_all(cache_path.clone())?;
        cache_path.push(
            Path::new(&pdf_path)
                .file_stem()
                .context("No file name found")?
                .to_str()
                .context("Invalid UTF-8 File name")?,
        );
        // cache_path.push(format!("{}", num));
        // while cache_path.is_dir() {
        //     cache_path.pop();
        //     num = rand::thread_rng().gen_range(0..100000);
        //     cache_path.push(format!("{}", num));
        // }
        let mut cmd = Command::new(path.as_path());
        if let Some(args) = args {
            cmd.args(args);
        }
        let _output = cmd
            .arg(out_type.get_arg())
            .arg(pdf_path)
            .arg(cache_path.clone())
            .output()?;
        //println!("{}",output.status);
        //println!("{}", String::from_utf8(output.stdout)?.trim_end());
        //println!("{}", String::from_utf8(output.stderr)?.trim_end());
        let mut out_vec = Vec::new();
        for entry in glob::glob(&format!(
            "{}*.{}",
            cache_path.display(),
            out_type.get_formatter(),
        ))? {
            match entry {
                Ok(path) => out_vec.push(path),
                Err(e) => Err(e)?,
            }
        }
        let mut final_out = Vec::new();
        for i in out_vec {
            let init_path: &Path = i.as_ref();
            let mut out_path = i.clone();
            let name = init_path
                .file_stem()
                .context("Unreachable")?
                .to_str()
                .context("Bad file name")?;
            let mut iter = name.split('-');
            let num: i32 = iter.next_back().context("Invalid Filename")?.parse()?;
            let from_name: String = iter.collect();
            out_path.set_file_name(format!("{}-{:03}", from_name, num));
            if let Some(ext) = init_path.extension() {
                out_path.set_extension(ext);
            }
            fs::rename(init_path, out_path.as_path())?;
            final_out.push(out_path);
        }
        Ok(final_out)
    }
}

fn get_file_path() -> Result<PathBuf, std::io::Error> {
    if let Some(project_dirs) = ProjectDirs::from("com", "pdf", "magik") {
        return Ok(PathBuf::from(project_dirs.data_local_dir()));
    }
    Err(std::io::Error::new(
        std::io::ErrorKind::NotFound,
        "No project dir found",
    ))
}
fn get_cache_path() -> Result<PathBuf, std::io::Error> {
    if let Some(project_dirs) = ProjectDirs::from("com", "pdf", "magik") {
        return Ok(PathBuf::from(project_dirs.cache_dir()));
    }
    Err(std::io::Error::new(
        std::io::ErrorKind::NotFound,
        "No project dir found",
    ))
}
async fn download_files(url: &str, path: &Path) -> Result<(), Box<dyn Error>> {
    let mut file = File::create(path).await?;
    println!("Downloading {}", url);

    let mut stream = reqwest::get(url).await?.bytes_stream();

    while let Some(chunk_result) = stream.next().await {
        let chunk = chunk_result?;
        file.write_all(&chunk).await?;
    }
    file.flush().await?;
    println!("Downloaded {}", url);
    Ok(())
}
