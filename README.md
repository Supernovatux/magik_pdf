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
// For single page
// Use tools.convert_to_image_single_page(i, OutputType::JPEGCMKY, Some(vec!["-r","300"])).unwrap(); 
println!("{:?}", out);
// Out is a vector of png image paths, use as fit
tools.delete_files(out).unwrap();
```


All available additional args

```
  -f <int>                                 : first page to print
  -l <int>                                 : last page to print
  -o                                       : print only odd pages
  -e                                       : print only even pages
  -scale-dimension-before-rotation         : for rotated pdf, resize dimensions before the rotation
  -r <fp>                                  : resolution, in DPI (default is 150)
  -rx <fp>                                 : X resolution, in DPI (default is 150)
  -ry <fp>                                 : Y resolution, in DPI (default is 150)
  -scale-to <int>                          : scales each page to fit within scale-to*scale-to pixel box
  -scale-to-x <int>                        : scales each page horizontally to fit in scale-to-x pixels
  -scale-to-y <int>                        : scales each page vertically to fit in scale-to-y pixels
  -x <int>                                 : x-coordinate of the crop area top left corner
  -y <int>                                 : y-coordinate of the crop area top left corner
  -W <int>                                 : width of crop area in pixels (default is 0)
  -H <int>                                 : height of crop area in pixels (default is 0)
  -sz <int>                                : size of crop square in pixels (sets W and H)
  -cropbox                                 : use the crop box rather than media box
  -hide-annotations                        : do not show annotations
  -mono                                    : generate a monochrome PBM file
  -gray                                    : generate a grayscale PGM file
  -displayprofile <string>                 : ICC color profile to use as the display profile
  -defaultgrayprofile <string>             : ICC color profile to use as the DefaultGray color space
  -defaultrgbprofile <string>              : ICC color profile to use as the DefaultRGB color space
  -defaultcmykprofile <string>             : ICC color profile to use as the DefaultCMYK color space
  -sep <string>                            : single character separator between name and page number, default - 
  -forcenum                                : force page number even if there is only one page
  -jpegopt <string>                        : jpeg options, with format <opt1>=<val1>[,<optN>=<valN>]*
  -overprint                               : enable overprint
  -tiffcompression <string>                : set TIFF compression: none, packbits, jpeg, lzw, deflate
  -freetype <string>                       : enable FreeType font rasterizer: yes, no
  -thinlinemode <string>                   : set thin line mode: none, solid, shape. Default: none
  -aa <string>                             : enable font anti-aliasing: yes, no
  -aaVector <string>                       : enable vector anti-aliasing: yes, no
  -opw <string>                            : owner password (for encrypted files)
  -upw <string>                            : user password (for encrypted files)
```
