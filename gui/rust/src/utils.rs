use std::fmt::{Debug, Display};
use godot::classes::Node;
use godot::prelude::*;

#[derive(Debug)]
pub(crate) struct GdCastError<T: Debug>(T);

impl<T: Debug> std::error::Error for GdCastError<T> {}

impl<T: Debug> Display for GdCastError<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.0)
    }
}

pub(crate) fn gderr<T: Debug + 'static>(inner: T) -> Box<dyn std::error::Error> {
    Box::new(GdCastError(inner))
}

// Helper trait to safely get nodes by NodePath and cast them to a specific type.
// Prefer stable NodePaths or unique names over index-based child access.
pub(crate) trait NodeExt {
    fn get_as<TTarget>(&self, path: &str) -> Result<Gd<TTarget>, String>
    where
        TTarget: GodotClass + Inherits<Node>;

    fn get_unique_as<TTarget>(&self, unique_name: &str) -> Result<Gd<TTarget>, String>
    where
        TTarget: GodotClass + Inherits<Node>;
}

impl<TSelf> NodeExt for Gd<TSelf>
where
    TSelf: GodotClass + Inherits<Node>,
{
    fn get_as<TTarget>(&self, path: &str) -> Result<Gd<TTarget>, String>
    where
        TTarget: GodotClass + Inherits<Node>,
    {
        let mut current: Gd<Node> = self.clone().upcast();
        for segment in path.split('/') {
            if segment.is_empty() {
                continue;
            }
            let maybe_child = current
                .get_children()
                .iter_shared()
                .find(|c| c.get_name().to_string() == segment);
            match maybe_child {
                Some(child) => {
                    current = child;
                }
                None => return Err(format!("NodePath not found: {} (missing segment '{}')", path, segment)),
            }
        }
        current
            .try_cast::<TTarget>()
            .map_err(|_e| format!("Cast failed at {}", path))
    }

    fn get_unique_as<TTarget>(&self, unique_name: &str) -> Result<Gd<TTarget>, String>
    where
        TTarget: GodotClass + Inherits<Node>,
    {
        // Simple BFS search for a node with matching name. Assumes uniqueness within owner.
        let mut queue: Vec<Gd<Node>> = self.clone().upcast::<Node>().get_children().iter_shared().collect();
        while let Some(node) = queue.pop() {
            if node.get_name().to_string() == unique_name {
                return node
                    .try_cast::<TTarget>()
                    .map_err(|_e| format!("Cast failed for unique name {}", unique_name));
            }
            for child in node.get_children().iter_shared() {
                queue.push(child);
            }
        }
        Err(format!("Unique node not found: {}", unique_name))
    }
}
