use crate::machine::Edge;

// Temporary filler pod
pub struct Pod {
    pub name: String,
    pub namespace: String,
}

pub trait StateHandler {
    fn handle(&self, pod: Pod) -> Edge;
}

//
// Handlers section
//

pub struct ImagePullHandler;

impl StateHandler for ImagePullHandler {
    fn handle(&self, _pod: Pod) -> Edge {
        Edge::Success
    }
}

pub struct ImagePullErrorHandler;

impl StateHandler for ImagePullErrorHandler {
    fn handle(&self, _pod: Pod) -> Edge {
        Edge::Success
    }
}

pub struct VolumeHandler;

impl StateHandler for VolumeHandler {
    fn handle(&self, _pod: Pod) -> Edge {
        Edge::Success
    }
}

pub struct VolumeErrorHandler;

impl StateHandler for VolumeErrorHandler {
    fn handle(&self, _pod: Pod) -> Edge {
        Edge::Success
    }
}

pub struct ContainerStartHandler;

impl StateHandler for ContainerStartHandler {
    fn handle(&self, _pod: Pod) -> Edge {
        Edge::Success
    }
}

pub struct ContainerErrorHandler;

impl StateHandler for ContainerErrorHandler {
    fn handle(&self, _pod: Pod) -> Edge {
        Edge::Success
    }
}

pub struct PodRunningHandler;

impl StateHandler for PodRunningHandler {
    fn handle(&self, _pod: Pod) -> Edge {
        Edge::Success
    }
}
