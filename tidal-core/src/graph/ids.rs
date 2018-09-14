use derive_more::{Display, From, Into};
use serde::{Deserialize, Serialize};

#[derive(
    Clone,
    Copy,
    Debug,
    Deserialize,
    Display,
    Eq,
    From,
    Hash,
    Into,
    Ord,
    PartialEq,
    PartialOrd,
    Serialize,
)]
pub struct NodeId(pub(crate) usize);

#[derive(
    Clone,
    Copy,
    Debug,
    Deserialize,
    Display,
    Eq,
    From,
    Hash,
    Into,
    Ord,
    PartialEq,
    PartialOrd,
    Serialize,
)]
pub struct PortId(pub(crate) usize);

impl NodeId {
    #[inline(always)]
    pub fn root() -> Self {
        Self(0)
    }

    #[inline(always)]
    pub fn is_root(&self) -> bool {
        *self == Self::root()
    }
}

#[derive(
    Serialize, Deserialize, From, Hash, Ord, PartialOrd, Debug, Copy, Clone, Eq, PartialEq, Display,
)]
#[display(fmt = "({}, {})", _0, _1)]
pub struct NodePortId(pub NodeId, pub PortId);

impl NodePortId {
    pub fn get_node_id(&self) -> NodeId {
        self.0
    }

    pub fn get_port_id(&self) -> PortId {
        self.1
    }
}
