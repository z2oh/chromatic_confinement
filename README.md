# chromatic_confinement

See the article for this repo here: http://blog.krum.io/k-d-trees/

An implementation of a k-d tree to quantize color images down to a defined set.

After cloning, use `cargo build --release` to download dependencies and compile. From there, you can invoke the binary directly in `./target/release/chromatic_confiner <args>` or you can use `cargo run --release -- <args>`.

There are three required files to be passed into the program.

`-c <FILE>` The color specification file to be used in restricting the image. This should be formatted as a list of `r,g,b` color values, with one color per line. See https://github.com/z2oh/chromatic_confinement/blob/master/color_files/css4.colors for an example.

`-i <FILE>` The input image to quantize

`-o <FILE>` The path of the resultant output image (this should end in .png).

You can pass the `-n` flag to run the slow version of this algorithm. This is the non k-d tree version, which runs much more slowly than the k-d tree version when the number of colors is large. See http://blog.krum.io/k-d-trees/#results for a more detailed comparison.

Example usage (can be run immediately after cloning):

`cargo run --release -- -c color_files/1kcolors.colors -i images/bridge_4096_2304.jpg -o image.png` will produce this image (in `./image.png`, the image you see here is scaled down to reduce load time of this repo. Image provided courtesy of [Julian Herzog](https://julianherzog.com/), see [LICENSE](images/LICENSE) for more information):

![Exmaple output, scaled to 25% size, image of bridge quantized to 1024 random colors](example_out/image.png)

If you would like to try out this code on huge images, I recommend searching for images on Wikimedia Commons. [Here is a 30,000 x 23,756 pixel image of Starry Night](https://commons.wikimedia.org/wiki/File:Van_Gogh_-_Starry_Night_-_Google_Art_Project.jpg), for example.
