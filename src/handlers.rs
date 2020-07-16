use std::sync::Arc;

use dashmap::DashMap;

use crate::machine::Edge;

// Temporary filler pod
#[derive(Clone, Debug)]
pub struct Pod {
    pub name: String,
    pub namespace: String,
    pub image: String,
    pub volumes: Vec<String>,
}

pub trait StateHandler<T> {
    fn handle(&self, pod: T) -> Edge;
}

//
// Handlers section
//

pub struct ImagePullHandler {
    pub state: Arc<DashMap<String, Pod>>,
}

impl StateHandler<Pod> for ImagePullHandler {
    fn handle(&self, pod: Pod) -> Edge {
        if let Some(current_state) = self.state.get(&pod.name) {
            if current_state.image == pod.image {
                println!("Image is already up to date");
                return Edge::Success;
            }
            println!(
                "Downloading image {} for pod {} in namespace {}",
                pod.image, pod.name, pod.namespace
            );
            self.state.update(&pod.name, |_, v| {
                let mut new_pod = v.clone();
                new_pod.image = pod.image.clone();
                new_pod
            });
        } else {
            return Edge::Failure(format!(
                "OH NOES: Pod with the name of {} in namespace {} doesn't exist in store",
                pod.name, pod.namespace
            ));
        }

        Edge::Success
    }
}

pub struct ImagePullErrorHandler {
    pub _state: Arc<DashMap<String, Pod>>,
}

impl StateHandler<Pod> for ImagePullErrorHandler {
    fn handle(&self, _pod: Pod) -> Edge {
        Edge::Success
    }
}

pub struct VolumeHandler {
    pub state: Arc<DashMap<String, Pod>>,
}

impl StateHandler<Pod> for VolumeHandler {
    fn handle(&self, pod: Pod) -> Edge {
        if let Some(current_state) = self.state.get(&pod.name) {
            if current_state.volumes == pod.volumes {
                println!("Volumes are already up to date");
                return Edge::Success;
            }
            println!(
                "Updating volumes {:?} for pod {} in namespace {}",
                pod.volumes, pod.name, pod.namespace
            );
            self.state.update(&pod.name, |_, v| {
                let mut new_pod = v.clone();
                new_pod.volumes = pod.volumes.clone();
                new_pod
            });
        } else {
            return Edge::Failure(format!(
                "OH NOES: Pod with the name of {} in namespace {} doesn't exist in store",
                pod.name, pod.namespace
            ));
        }

        Edge::Success
    }
}

pub struct VolumeErrorHandler {
    pub state: Arc<DashMap<String, Pod>>,
}

impl StateHandler<Pod> for VolumeErrorHandler {
    fn handle(&self, _pod: Pod) -> Edge {
        Edge::Success
    }
}

pub struct ContainerStartHandler {
    pub state: Arc<DashMap<String, Pod>>,
}

impl StateHandler<Pod> for ContainerStartHandler {
    fn handle(&self, _pod: Pod) -> Edge {
        Edge::Success
    }
}

pub struct ContainerErrorHandler {
    pub state: Arc<DashMap<String, Pod>>,
}

impl StateHandler<Pod> for ContainerErrorHandler {
    fn handle(&self, _pod: Pod) -> Edge {
        Edge::Success
    }
}

pub struct PodRunningHandler {
    pub state: Arc<DashMap<String, Pod>>,
}

impl StateHandler<Pod> for PodRunningHandler {
    fn handle(&self, _pod: Pod) -> Edge {
        Edge::Success
    }
}
