# Rust resize image

Tool to resize images and optimize them for web.

## Features

* Resize image to width respecting aspect ratio
* Crop image when height requirement is not met
* Optimize for web (*not implemented yet*)

## How it works

* Reads image from **path_to_input_image**
* Resizes input image to **required_width** with respect to aspect ratio
* Crops image to **required_height**, either cropping from **top**, **bottom** or **even** as determined by **crop_method** argument
* Writes new image to **output_image**

### Crop methods

* top *(crops from the top)*
* bottom *(crops from the bottom)*
* even *(crops evenly from top and bottom)*

## Build

```
cargo build --release
```

Binary is output in target/release

## Run binary

```
./rust_resize_image <path_to_input_image> <required_width> <required_height> <crop_method> <output_image>
./rust_resize_image header.png 723 281 even out.png
```

## Run development

```
cargo run header.png 723 281 even out.png
```
