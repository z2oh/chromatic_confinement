extern crate ego_tree;
extern crate image;

use image::GenericImage;

use ego_tree::{Tree, NodeMut, NodeRef, NodeId};

use std::path::Path;
use std::fs::File;
use std::env;

fn main() {
    let mut naive: bool = false;
    let mut color_vec: Vec<Vec<u8>> = Vec::new();
    let mut color_spec: &str = "";

    for arg in env::args().skip(1) {
        match arg.as_ref() {
            "naive" => {
                naive = true;
            },
            "css1" => {
                color_vec = vec![
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
                color_spec = "css1";
            },
            "css4" => {
                color_vec = vec![
                    vec![0x00,0x00,0x00],
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
                    vec![0xff,0xa5,0x00],
                    vec![0xf0,0xf8,0xff],
                    vec![0xfa,0xeb,0xd7],
                    vec![0x7f,0xff,0xd4],
                    vec![0xf0,0xff,0xff],
                    vec![0xf5,0xf5,0xdc],
                    vec![0xff,0xe4,0xc4],
                    vec![0xff,0xeb,0xcd],
                    vec![0x8a,0x2b,0xe2],
                    vec![0xa5,0x2a,0x2a],
                    vec![0xde,0xb8,0x87],
                    vec![0x5f,0x9e,0xa0],
                    vec![0x7f,0xff,0x00],
                    vec![0xd2,0x69,0x1e],
                    vec![0xff,0x7f,0x50],
                    vec![0x64,0x95,0xed],
                    vec![0xff,0xf8,0xdc],
                    vec![0xdc,0x14,0x3c],
                    vec![0x00,0xff,0xff],
                    vec![0x00,0x00,0x8b],
                    vec![0x00,0x8b,0x8b],
                    vec![0xb8,0x86,0x0b],
                    vec![0xa9,0xa9,0xa9],
                    vec![0x00,0x64,0x00],
                    vec![0xa9,0xa9,0xa9],
                    vec![0xbd,0xb7,0x6b],
                    vec![0x8b,0x00,0x8b],
                    vec![0x55,0x6b,0x2f],
                    vec![0xff,0x8c,0x00],
                    vec![0x99,0x32,0xcc],
                    vec![0x8b,0x00,0x00],
                    vec![0xe9,0x96,0x7a],
                    vec![0x8f,0xbc,0x8f],
                    vec![0x48,0x3d,0x8b],
                    vec![0x2f,0x4f,0x4f],
                    vec![0x2f,0x4f,0x4f],
                    vec![0x00,0xce,0xd1],
                    vec![0x94,0x00,0xd3],
                    vec![0xff,0x14,0x93],
                    vec![0x00,0xbf,0xff],
                    vec![0x69,0x69,0x69],
                    vec![0x69,0x69,0x69],
                    vec![0x1e,0x90,0xff],
                    vec![0xb2,0x22,0x22],
                    vec![0xff,0xfa,0xf0],
                    vec![0x22,0x8b,0x22],
                    vec![0xdc,0xdc,0xdc],
                    vec![0xf8,0xf8,0xff],
                    vec![0xff,0xd7,0x00],
                    vec![0xda,0xa5,0x20],
                    vec![0xad,0xff,0x2f],
                    vec![0x80,0x80,0x80],
                    vec![0xf0,0xff,0xf0],
                    vec![0xff,0x69,0xb4],
                    vec![0xcd,0x5c,0x5c],
                    vec![0x4b,0x00,0x82],
                    vec![0xff,0xff,0xf0],
                    vec![0xf0,0xe6,0x8c],
                    vec![0xe6,0xe6,0xfa],
                    vec![0xff,0xf0,0xf5],
                    vec![0x7c,0xfc,0x00],
                    vec![0xff,0xfa,0xcd],
                    vec![0xad,0xd8,0xe6],
                    vec![0xf0,0x80,0x80],
                    vec![0xe0,0xff,0xff],
                    vec![0xfa,0xfa,0xd2],
                    vec![0xd3,0xd3,0xd3],
                    vec![0x90,0xee,0x90],
                    vec![0xd3,0xd3,0xd3],
                    vec![0xff,0xb6,0xc1],
                    vec![0xff,0xa0,0x7a],
                    vec![0x20,0xb2,0xaa],
                    vec![0x87,0xce,0xfa],
                    vec![0x77,0x88,0x99],
                    vec![0x77,0x88,0x99],
                    vec![0xb0,0xc4,0xde],
                    vec![0xff,0xff,0xe0],
                    vec![0x32,0xcd,0x32],
                    vec![0xfa,0xf0,0xe6],
                    vec![0xff,0x00,0xff],
                    vec![0x66,0xcd,0xaa],
                    vec![0x00,0x00,0xcd],
                    vec![0xba,0x55,0xd3],
                    vec![0x93,0x70,0xdb],
                    vec![0x3c,0xb3,0x71],
                    vec![0x7b,0x68,0xee],
                    vec![0x00,0xfa,0x9a],
                    vec![0x48,0xd1,0xcc],
                    vec![0xc7,0x15,0x85],
                    vec![0x19,0x19,0x70],
                    vec![0xf5,0xff,0xfa],
                    vec![0xff,0xe4,0xe1],
                    vec![0xff,0xe4,0xb5],
                    vec![0xff,0xde,0xad],
                    vec![0xfd,0xf5,0xe6],
                    vec![0x6b,0x8e,0x23],
                    vec![0xff,0x45,0x00],
                    vec![0xda,0x70,0xd6],
                    vec![0xee,0xe8,0xaa],
                    vec![0x98,0xfb,0x98],
                    vec![0xaf,0xee,0xee],
                    vec![0xdb,0x70,0x93],
                    vec![0xff,0xef,0xd5],
                    vec![0xff,0xda,0xb9],
                    vec![0xcd,0x85,0x3f],
                    vec![0xff,0xc0,0xcb],
                    vec![0xdd,0xa0,0xdd],
                    vec![0xb0,0xe0,0xe6],
                    vec![0xbc,0x8f,0x8f],
                    vec![0x41,0x69,0xe1],
                    vec![0x8b,0x45,0x13],
                    vec![0xfa,0x80,0x72],
                    vec![0xf4,0xa4,0x60],
                    vec![0x2e,0x8b,0x57],
                    vec![0xff,0xf5,0xee],
                    vec![0xa0,0x52,0x2d],
                    vec![0x87,0xce,0xeb],
                    vec![0x6a,0x5a,0xcd],
                    vec![0x70,0x80,0x90],
                    vec![0x70,0x80,0x90],
                    vec![0xff,0xfa,0xfa],
                    vec![0x00,0xff,0x7f],
                    vec![0x46,0x82,0xb4],
                    vec![0xd2,0xb4,0x8c],
                    vec![0xd8,0xbf,0xd8],
                    vec![0xff,0x63,0x47],
                    vec![0x40,0xe0,0xd0],
                    vec![0xee,0x82,0xee],
                    vec![0xf5,0xde,0xb3],
                    vec![0xf5,0xf5,0xf5],
                    vec![0x9a,0xcd,0x32],
                    vec![0x66,0x33,0x99],
                ];
                color_spec = "css4";
            },
            _ => {
                naive = false;
            },
        }
    }

    let image_path = Path::new("/home/jaday/img.jpeg");
    let image = image::open(image_path).unwrap();
    let (img_x, img_y) = image.dimensions();
    let mut img_buf = image::ImageBuffer::new(img_x, img_y);

    if !naive {
        let kd_tree = construct_kd_tree(&mut color_vec[..], 3);
        println!("{:?}", kd_tree);
        for (x, y, rgb) in img_buf.enumerate_pixels_mut() {
            let rgba = image.get_pixel(x, y).data;
            let c = vec![rgba[0], rgba[1], rgba[2]];
            let color_tup = query_nearest_neighbor(&c[..], &kd_tree, 3, kd_tree.root().id());
            *rgb = image::Rgba([color_tup.0, color_tup.1, color_tup.2, 255]);
        }
    }
    else if naive {
        for (x, y, rgb) in img_buf.enumerate_pixels_mut() {
            let rgba = image.get_pixel(x, y).data;
            let c = vec![rgba[0], rgba[1], rgba[2]];
            let nn = return_nearest_naive(&color_vec, c);
            *rgb = image::Rgba([nn[0], nn[1], nn[2], 255]);
        }
    }

    let mut save_path = String::new();
    if naive {
        save_path.push_str("naive_");
    }
    else if !naive {
        save_path.push_str("k-d_tree_");
    }

    if color_spec == "css1" {
        save_path.push_str("css1");
    }
    else if color_spec == "css4" {
        save_path.push_str("css4");
    }

    save_path.push_str(".png");

    let ref mut fout = File::create(save_path).unwrap();

    image::ImageRgba8(img_buf).save(fout, image::PNG).unwrap();
    
    unsafe {
        println!("{} calls to distance function.", DIST_CALLS);
    }
}

static mut DIST_CALLS: u32 = 0;

fn dist_sq(a: &[u8], b: &Vec<u8>) -> u32 {
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

fn return_nearest_naive(colors: &Vec<Vec<u8>>, query: Vec<u8>) -> Vec<u8> {
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

fn query_nearest_neighbor<'a>(q: &[u8], kd_tree: &'a Tree<Vec<u8>>, max_dimension: usize, root_node_id: NodeId) -> &'a NodeRef<Vec<u8>> {
   let mut current_node = kd_tree.root(); 
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
       if current_node.id() == root_node_id {
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
           let second_best_guess_node = query_nearest_neighbor(q, kd_tree, max_dimension, current_node.next_sibling().unwrap().id());
           let second_best_guess_dist = dist_sq(q, second_best_guess_node.value());
           if second_best_guess_dist < best_guess_dist {
               best_guess_dist = second_best_guess_dist;
               best_guess_node = second_best_guess_node;
           }
       }

       current_node = parent_node;
   }

   return best_guess_node;
}

/*
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

    let mut next_dim = current_dim + 1;
    if next_dim == max_dim {
        next_dim = 0;
    }

    let left_idx = construct_kd_tree_recursive(&mut v[..middle], graph, next_dim, max_dim);
    let right_idx = construct_kd_tree_recursive(&mut v[middle+1..], graph, next_dim, max_dim);

    graph.add_edge(middle_idx, left_idx, 0);
    graph.add_edge(middle_idx, right_idx, 1);

    return middle_idx;
}

fn query_nearest_neighbor(q: &Vec<u8>, kd_tree: &Graph<Vec<u8>, u8>, root_idx: NodeIndex, max_dimension: u8) -> NodeIndex {
    let mut current_idx = root_idx;
    let mut current_dim = 0;
    loop {
        let mut neighbors = kd_tree.neighbors(current_idx);
        let first = neighbors.next();

        // If we have reached a leaf.
        if first.is_none() {
            break;
        }

        // If we have only one child, we continue down that path until we reach
        // a leaf node.
        else {
            let second = neighbors.next();
            if second.is_none() {
                current_idx = first.unwrap();
            }
            // Otherwise, we traverse down the kd-tree following our rule that the
            // smaller value at the current dimension is on the left and the greater
            // value at the current dimension is on the right.
            else {
                //let mut edges: Vec<EdgeReference<u8>> = kd_tree.edges(current_idx);
                let node1_idx = first.unwrap();
                let node2_idx = second.unwrap();
                let node1 = &kd_tree[node1_idx];
                let node2 = &kd_tree[node2_idx];
                //edges.sort_unstable_by_key(|e| e.target());
                let current_dim_val = (&kd_tree[current_idx])[current_dim as usize];
                if q[current_dim as usize] < current_dim_val {
                    if node1[current_dim as usize] < current_dim_val {
                        current_idx = node1_idx;
                    }
                    else {
                        current_idx = node2_idx;
                    }
                }
                else {
                    if node1[current_dim as usize] < current_dim_val {
                        current_idx = node2_idx;
                    }
                    else {
                        current_idx = node1_idx;
                    }
                }
            }
        }
        current_dim += 1;
        if current_dim == max_dimension {
            current_dim = 0;
        }
    }

    let mut best_guess_idx = current_idx;
    let mut best_guess_distance = dist_sq(&q, &kd_tree[best_guess_idx]);

    loop {
        let parent_idx_vec: Vec<NodeIndex> = kd_tree.neighbors_directed(current_idx, petgraph::Direction::Incoming).collect();
        if parent_idx_vec.len() == 0 {
            break;
        }
        let parent_idx = parent_idx_vec[0];
        if parent_idx == root_idx {
            break;
        }
        let parent_val = &kd_tree[parent_idx];
        let new_dist = dist_sq(&q, &kd_tree[parent_idx]);
        if new_dist < best_guess_distance {
            best_guess_distance = new_dist;
            best_guess_idx = parent_idx;
        }

        let plane_dist: u32 = (parent_val[current_dim as usize] as i32 - (&kd_tree[best_guess_idx])[current_dim as usize] as i32).abs() as u32;
        if plane_dist * plane_dist < best_guess_distance {
            let second_best_guess_idx = query_nearest_neighbor(&q, kd_tree, parent_idx, max_dimension);
            let second_best_guess_dist = dist_sq(&q, &kd_tree[second_best_guess_idx]);
            if second_best_guess_dist < best_guess_distance {
                best_guess_distance = second_best_guess_dist;
                best_guess_idx = second_best_guess_idx;
            }
        }

        current_idx = parent_idx;
        
        if current_dim == 0 {
            current_dim = max_dimension - 1;
        }
        else {
            current_dim -= 1;
        }
    }

    return best_guess_idx;
}
*/
