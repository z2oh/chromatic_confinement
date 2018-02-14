extern crate ego_tree;
extern crate image;
extern crate clap;

use image::GenericImage;

use ego_tree::{Tree, NodeMut, NodeRef};

use std::io::{BufReader, BufRead};
use std::path::Path;
use std::fs::File;
use std::error::Error;

use clap::{Arg, App};

fn main() {
    let matches = App::new("Chromatic Confinement")
                           .version("0.1")
                           .author("Jeremy Day <jadaytime@gmail.com>")
                           .arg(Arg::with_name("input")
                                .short("i")
                                .long("input")
                                .value_name("FILE")
                                .help("The input image")
                                .takes_value(true)
                                .required(true))
                           .arg(Arg::with_name("output")
                                .short("o")
                                .long("output")
                                .value_name("FILE")
                                .help("The output image path")
                                .takes_value(true)
                                .required(true))
                           .arg(Arg::with_name("colors")
                                .short("c")
                                .long("colors")
                                .value_name("FILE")
                                .help("The colors to be used in restricting the image")
                                .takes_value(true)
                                .required(true))
                           .arg(Arg::with_name("naive")
                                .short("n")
                                .long("naive")
                                .help("Specifiy that the naive method should be used"))
                           .get_matches();

    let colors_path = Path::new(matches.value_of("colors").unwrap());

    let colors_file = match File::open(colors_path) {
        Err(v) => panic!("Could not open color specification file: {}", v.description()),
        Ok(r) => BufReader::new(r),
    };

    let mut color_vec: Vec<Vec<u8>> = Vec::new();

    for line in colors_file.lines() {
        let color: Vec<u8> = line.unwrap().split(",").map(|v| v.parse::<u8>().unwrap()).collect();
        color_vec.push(color);
    }

    let naive: bool = match matches.occurrences_of("naive") {
        0 => false,
        _ => true,
    };

    let image_path = Path::new(matches.value_of("input").unwrap());
    let image = image::open(image_path).unwrap();
    let (img_x, img_y) = image.dimensions();
    let mut img_buf = image::ImageBuffer::new(img_x, img_y);

    if !naive {
        let kd_tree = construct_kd_tree(&mut color_vec[..], 3);
        for (x, y, rgb) in img_buf.enumerate_pixels_mut() {
            let rgba = image.get_pixel(x, y).data;
            let color_tup = query_nearest_neighbor(&rgba[..3], &kd_tree, 3, kd_tree.root()).value();
            *rgb = image::Rgba([color_tup[0], color_tup[1], color_tup[2], 255]);
        }
    }
    else if naive {
        for (x, y, rgb) in img_buf.enumerate_pixels_mut() {
            let rgba = image.get_pixel(x, y).data;
            let nn = return_nearest_naive(&color_vec, &rgba[..3]);
            *rgb = image::Rgba([nn[0], nn[1], nn[2], 255]);
        }
    }

    let save_path = Path::new(matches.value_of("output").unwrap());
    let ref mut fout = File::create(save_path).unwrap();

    image::ImageRgba8(img_buf).save(fout, image::PNG).unwrap();
    
    unsafe {
        println!("{} calls to distance function.", DIST_CALLS);
    }
}

static mut DIST_CALLS: u64 = 0;

fn dist_sq(a: &[u8], b: &[u8]) -> u32 {
    unsafe {
        DIST_CALLS += 1;
    }
    let mut sum: u32 = 0;
    for (aa, bb) in a.iter().zip(b.iter()) {
        let val: i32 = (*aa as i32) - (*bb as i32);
        sum += (val * val) as u32;
    }
    return sum;
}

fn return_nearest_naive(colors: &Vec<Vec<u8>>, query: &[u8]) -> Vec<u8> {
    let mut min_dist = 255*255 + 255*255 + 255*255;
    let mut idx = 0;
    for (i, color) in colors.iter().enumerate() {
        let dist = dist_sq(color, query);
        if dist < min_dist {
            min_dist = dist;
            idx = i;
        }
    }
    return colors[idx].clone();
}

fn construct_kd_tree(v: &mut [Vec<u8>], dimension: usize) -> Tree<Vec<u8>> {
    v.sort_unstable_by_key(|k| k[0]);
    let middle: usize = v.len() / 2;
    let mut tree = Tree::with_capacity(v[middle].clone(), v.len());
    {
        let mut root = tree.root_mut();
        construct_kd_tree_recursive(&mut v[..middle], &mut root, 1, dimension);
        construct_kd_tree_recursive(&mut v[middle+1..], &mut root, 1, dimension);
    }
    tree
}

fn construct_kd_tree_recursive(v: &mut [Vec<u8>], node: &mut NodeMut<Vec<u8>>, current_dim: usize, max_dimension: usize) {
    if v.len() == 2 {
        v.sort_unstable_by_key(|k| k[current_dim]);
        let mut child = node.append(v[1].clone());
        child.append(v[0].clone());
        return;
    }
    else if v.len() == 1 {
        node.append(v[0].clone());
        return;
    }
    v.sort_unstable_by_key(|k| k[current_dim]);
    let middle: usize = v.len() / 2;
    let mut child = node.append(v[middle].clone());
    let mut next_dim = current_dim + 1;
    if next_dim == max_dimension {
        next_dim = 0;
    }
    construct_kd_tree_recursive(&mut v[..middle], &mut child, next_dim, max_dimension);
    construct_kd_tree_recursive(&mut v[middle+1..], &mut child, next_dim, max_dimension);
}

fn query_nearest_neighbor<'a>(q: &[u8], kd_tree: &'a Tree<Vec<u8>>, max_dimension: usize, root_node_ref: NodeRef<'a, Vec<u8>>) -> NodeRef<'a, Vec<u8>> {
    let mut current_node = root_node_ref; 
    let mut current_dim = 0;
    loop {
       // We have reached a leaf node.
        if !current_node.has_children() {
            break;
        }
        let left_child = current_node.first_child().unwrap();
        if !left_child.has_siblings() {
            current_node = left_child;
            current_dim += 1;
            if current_dim == max_dimension {
                current_dim = 0;
            }
        }
        else {
            if q[current_dim] < current_node.value()[current_dim] {
                current_node = left_child;
            }
            else {
                current_node = current_node.last_child().unwrap();
            }
            current_dim += 1;
            if current_dim == max_dimension {
                current_dim = 0;
            }
        }
    }

    let mut best_guess_node = current_node;
    let mut best_guess_dist = dist_sq(q, current_node.value());

    loop {
        // We have reached the root.
        if current_node == root_node_ref {
            break;
        }
        let parent_node = current_node.parent().unwrap();
        let parent_val = parent_node.value();
        let parent_dist = dist_sq(q, parent_val);
        if parent_dist < best_guess_dist{
            best_guess_dist = parent_dist;
            best_guess_node = parent_node;
        }

        let plane_dist: u32 = (parent_val[current_dim] as i32 - best_guess_node.value()[current_dim] as i32).abs() as u32;
        if plane_dist * plane_dist < best_guess_dist {
            let mut node_id_option = current_node.next_sibling();
            if node_id_option.is_none() {
                node_id_option = current_node.prev_sibling();
            }
            if !node_id_option.is_none() {
                let second_best_guess_node = query_nearest_neighbor(q, kd_tree, max_dimension, node_id_option.unwrap());
                let second_best_guess_dist = dist_sq(q, second_best_guess_node.value());
                if second_best_guess_dist < best_guess_dist {
                    best_guess_dist = second_best_guess_dist;
                    best_guess_node = second_best_guess_node;
                }
            }
        }

        if current_dim == 0 {
            current_dim = max_dimension - 1;
        }
        else {
            current_dim -= 1;
        }

        current_node = parent_node;
    }

    return best_guess_node;
}
