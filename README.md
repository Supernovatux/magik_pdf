An interface for using poppler

## Example
```rust
use super::poppler_interface::PopperTools;
use super::poppler_interface::PopplerInterface;
let tools = PopperTools::new();
if !tools.is_tool_present().unwrap().is_dir() {
      tools.download_magik().unwrap();
}
let out =
      tools.convert_to_image(r"C:\Users\thula\Downloads\sponsorship.pdf", vec!["-png"]).unwrap();
println!("{:?}", out);
// Out is a vector of png image paths, use as fit
tools.delete_files(out).unwrap();
```
