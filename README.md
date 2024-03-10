An interface for using poppler

## Example
```rust
use super::poppler_interface::OutputType;
use super::poppler_interface::PopperTools;
use super::poppler_interface::PopplerInterface;
let tools = PopperTools::new();
if !tools.is_tool_present().unwrap().is_dir() {
      tools.download_magik().unwrap();
}
let out =
      tools.convert_to_image(r"C:\Users\thula\Downloads\sponsorship.pdf",OutputType::PNG,Some(vec!["-r","300"])).unwrap();
println!("{:?}", out);
// Out is a vector of png image paths, use as fit
tools.delete_files(out).unwrap();
```
