use crate::objects::*;
use crate::hit::{Hit, Hittable};
use crate::vector::Vector;
use crate::ray::Ray;

// type ChildNode = Option<Box<BTNode>>;

pub struct Node {
    bbox: bounding::Aabb,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
    object: Option<Box<dyn Hittable>>
}

impl Node {
    pub fn add_split(bbox: bounding::Aabb, l: Node, r: Node) -> Self {
        Node {bbox: bbox, left: Some(Box::new(l)), right: Some(Box::new(r)), object: None}  
    }
    pub fn add_leaf(bbox: bounding::Aabb, object: Box<dyn Hittable>) -> Self {
        Node {bbox: bbox, left: None, right: None, object: Some(object)}
    }

    pub fn send_ray_through(&self, ray: Ray) -> Option<Hit> {
        match &self.object {
            // Check if leaf node
            Some(obj) => return obj.intersect(ray),
            None => {
                // Check if ray hit big bounding box
                match self.bbox.intersect(ray) {
                    None => return None,
                    Some(_) => {
                        // If yes, reapeat for leaves
                        match &self.left {
                            Some(node) => return node.send_ray_through(ray),
                            None => {
                                match &self.right {
                                    None => return None,
                                    Some(node) => return node.send_ray_through(ray)
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub struct SceneTree {
    pub root: Node,
    pub light: light::Light
}

pub fn generate_tree(mut scene: scene::Scene) -> SceneTree {

    // Get min and max of all bounding boxes
    let n = scene.hittable_objects.len() - 1;
    scene.hittable_objects.sort_by(|a, b|
        a.get_bounds().min.x.partial_cmp(&b.get_bounds().min.x)
        .unwrap_or(std::cmp::Ordering::Equal)
    );
    let mut root_min = Vector::new(scene.hittable_objects[0].get_bounds().min.x, 0.0, 0.0);
    let mut root_max = Vector::new(scene.hittable_objects[n].get_bounds().max.x, 0.0, 0.0);
    scene.hittable_objects.sort_by(|a, b|
        a.get_bounds().min.y.partial_cmp(&b.get_bounds().min.y)
        .unwrap_or(std::cmp::Ordering::Equal)
    );
    root_min.y = scene.hittable_objects[0].get_bounds().min.y;
    root_max.y = scene.hittable_objects[n].get_bounds().max.y;
    scene.hittable_objects.sort_by(|a, b|
        a.get_bounds().min.z.partial_cmp(&b.get_bounds().min.z)
        .unwrap_or(std::cmp::Ordering::Equal)
    );
    root_min.z = scene.hittable_objects[0].get_bounds().min.z;
    root_max.z = scene.hittable_objects[n].get_bounds().max.z;
    let _root_box = bounding::Aabb::new(root_min, root_max);
    // println!("Root Box: {root_box:#?}");

    
    scene.hittable_objects.sort_by(|a, b|
        a.get_center().z.partial_cmp(&b.get_center().z)
        .unwrap_or(std::cmp::Ordering::Equal)
    );
    
    
    let mut root_node = Node{bbox: root_box, left: None, right: None, object: None};

    // let parent_node_ptr: *mut Node = &mut root_node;
    let mut idx = 0;
    // while &scene.hittable_objects.len() > &0 {
    //     root_node.left = Some(Box::new(
    //         Node::add_split(
    //             scene.hittable_objects[idx].bounding_box(),
    //              l: Node, 
    //              r: Node
    //         )
    //     ))
    // }
    // for obj in &scene.hittable_objects {

    // }

    // let test_node = Node::add_leaf(scene.hittable_objects[0].bounding_box(), Box::new(scene.hittable_objects[5]));
    SceneTree{root: root_node, light: scene.light}
    // scene::Scene{hittable_objects: scene.hittable_objects, light: scene.light}
}
