use super::kd_tree::KdTree;
use super::scene_object::SceneObject;
use super::light::Light;

pub struct Scene {
    pub objects: KdTree<SceneObject>,
    pub lights: Vec<Light>,
}
