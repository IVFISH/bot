use crate::placement::*;
use crate::pruner::*;
use fumen;
use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;
use std::iter::zip;
use std::path::Path;

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
        self.placements.extend(other.placements.into_iter());
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
        self.placements.extend(placements.into_iter())
    }

    /// debugging tool to write all the fumens to a json file
    pub fn write_fumens(&self, filename: &str) {
        let fumens = self
            .placements
            .iter()
            .map(|p| p.get_fumen())
            .collect::<Vec<_>>()
            .join("\n");
        let path = Path::new(filename);
        let mut file = File::create(&path).unwrap();
        let _ = file.write_all(fumens.as_bytes());
    }
}