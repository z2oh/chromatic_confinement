extern crate petgraph;
extern crate image;

use image::{GenericImage, ImageBuffer};

use petgraph::Graph;
use petgraph::graph::NodeIndex;
use petgraph::dot::Dot;

use std::path::Path;
use std::fs::File;

fn main() {
    let colors = vec![
        vec![128u8, 128u8, 128u8],
        vec![127u8, 0u8, 10u8],
        vec![150u8, 250u8, 250u8],
        vec![150u8, 150u8, 150u8],
        vec![16u8, 32u8, 48u8],
        vec![64u8, 33u8, 0u8],
        vec![200u8, 200u8, 200u8],
        vec![64u8, 31u8, 5u8],
        vec![0u8, 0u8, 0u8],
    ];

    let css1 = vec![
        vec![0x0,0x0,0x0],
        vec![0xc0,0xc0,0xc0],
        vec![0x80,0x80,0x80],
        vec![0xff,0xff,0xff],
        vec![0x80,0x00,0x00],
        vec![0xff,0x00,0x00],
        vec![0x80,0x00,0x80],
        vec![0xff,0x00,0xff],
        vec![0x00,0x80,0x00],
        vec![0x00,0xff,0x00],
        vec![0x80,0x80,0x00],
        vec![0xff,0xff,0x00],
        vec![0x00,0x00,0x80],
        vec![0x00,0x00,0xff],
        vec![0x00,0x80,0x80],
        vec![0x00,0xff,0xff],
    ];

    let image_path = Path::new("/home/jaday/img.jpeg");
    let image = image::open(image_path).unwrap();

    let (img_x, img_y) = image.dimensions();

    let mut img_buf = image::ImageBuffer::new(img_x, img_y);

    for (x, y, rgb) in img_buf.enumerate_pixels_mut() {
        let rgba = image.get_pixel(x, y).data;
        let c = vec![rgba[0], rgba[1], rgba[2]];
        let nn = return_nearest_naive(&css1, c);
        *rgb = image::Rgba([nn[0], nn[1], nn[2], 255]);
    }

    let ref mut fout = File::create("img.png").unwrap();

    image::ImageRgba8(img_buf).save(fout, image::PNG).unwrap();

    let (root_idx, kd_tree) = construct_kd_tree(colors, 3);
    println!("{:?}", Dot::with_config(&kd_tree, &[]));
}

fn return_nearest_naive(colors: &Vec<Vec<u8>>, query: Vec<u8>) -> Vec<u8> {
    fn dist_sq(a: &Vec<u8>, b: &Vec<u8>) -> u32 {
        let mut sum: u32 = 0;
        for (aa, bb) in a.iter().zip(b.iter()) {
            let val: i32 = (*aa as i32) - (*bb as i32);
            sum += (val * val) as u32;
        }
        return sum;
    }
    let mut min_dist = 255*255 + 255*255 + 255*255;
    let mut idx = 0;
    for (i, color) in colors.iter().enumerate() {
        let dist = dist_sq(color, &query);
        if dist < min_dist {
            min_dist = dist;
            idx = i;
        }
    }
    return colors[idx].clone();
}

fn construct_kd_tree(mut v: Vec<Vec<u8>>, dimension: u8) -> (NodeIndex, Graph<Vec<u8>, u8>) {
    let mut graph = Graph::<Vec<u8>, u8>::new();

    let root = construct_kd_tree_recursive(&mut v[..], &mut graph, 0, dimension);

    return (root, graph);
}

fn construct_kd_tree_recursive(v: &mut [Vec<u8>], mut graph: &mut Graph<Vec<u8>, u8>, current_dim: u8, max_dim: u8) -> NodeIndex {
    if v.len() == 1 {
        return graph.add_node(v[0].clone());
    }
    else if v.len() == 2 {
        v.sort_unstable_by_key(|k| k[current_dim as usize]);
        let parent_idx = graph.add_node(v[1].clone());
        let child_idx = graph.add_node(v[0].clone());
        graph.add_edge(parent_idx, child_idx, 0);
        return parent_idx;
    }

    v.sort_unstable_by_key(|k| k[current_dim as usize]);
    let middle = v.len() / 2;
    let middle_idx = graph.add_node(v[middle].clone());

    let next_dim = (current_dim + 1) % max_dim;

    let left_idx = construct_kd_tree_recursive(&mut v[..middle], graph, next_dim, max_dim);
    let right_idx = construct_kd_tree_recursive(&mut v[middle+1..], graph, next_dim, max_dim);

    graph.add_edge(middle_idx, left_idx, 0);
    graph.add_edge(middle_idx, right_idx, 1);

    return middle_idx;
}
