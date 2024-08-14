use crate::placement::*;
use crate::pruner::*;

#[derive(Default)]
pub struct PlacementList {
    pub placements: Vec<Placement>,
}

impl PlacementList {
    /// prunes a list of placements and
    /// creates an instance of placement list from
    /// the filtered list
    pub fn new(placements: Vec<Placement>, pruner: &impl Pruner) -> Self {
        Self {
            placements: pruner.prune(placements),
        }
    }

    /// extends each vector field within the placement list
    pub fn extend(&mut self, other: PlacementList) {
        self.placements.extend(other.placements);
    }

    /// adds a placement onto this placement list if it should not be pruned
    pub fn add(&mut self, other: Placement, pruner: &impl Pruner) {
        if pruner.precondition(&other) {
            self.placements.push(other);
        }
    }

    /// adds all elements of an iterable that should not be pruned
    pub fn add_many<T>(&mut self, placements: T)
    where
        T: IntoIterator<Item = Placement>,
    {
        self.placements.extend(placements)
    }
}
