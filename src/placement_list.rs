use rayon::iter::IntoParallelIterator;

use crate::placement::*;
use crate::pruner::*;
use std::fs::File;
use std::io::prelude::*;
use std::iter;
use std::path::Path;

pub struct PlacementList<I: Iterator<Item = Placement>>(pub I);

impl<I: Iterator<Item = Placement>> Iterator for PlacementList<I> {
    type Item = Placement;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }
}

impl<I: IntoParallelIterator<Item = Placement> + Iterator<Item = Placement>> IntoParallelIterator
    for PlacementList<I>
{
    type Iter = I::Iter;
    type Item = Placement;

    fn into_par_iter(self) -> Self::Iter {
        self.0.into_par_iter()
    }
}

// blanket implementation to convert all iterators over Placements into PlacementList
pub trait IntoPlacements<I: Iterator<Item = Placement>> {
    fn into_placements(self) -> PlacementList<I>;
}

impl<I: Iterator<Item = Placement>> IntoPlacements<I> for I {
    fn into_placements(self) -> PlacementList<I> {
        PlacementList(self)
    }
}

impl<'a, I: Iterator<Item = Placement> + 'a> PlacementList<I> {
    /// prunes a list of placements and
    /// creates an instance of placement list from
    /// the filtered list
    pub fn new(
        placements: I,
        _pruner: &impl Pruner,
    ) -> PlacementList<impl Iterator<Item = Placement>> {
        // Self(placements.into_iter().filter(|x| pruner.prune(x)).into_placements::<I>())
        placements.into_placements()
    }

    /// extends each vector field within the placement list
    pub fn extend(self, other: PlacementList<I>) -> PlacementList<impl Iterator<Item = Placement>> {
        self.0.chain(other.0).into_placements()
    }

    /// adds a placement onto this placement list if it should not be pruned
    pub fn add(
        self,
        other: Placement,
        pruner: &impl Pruner,
    ) -> PlacementList<Box<dyn Iterator<Item = Placement> + 'a>> {
        if pruner.precondition(&other) {
            PlacementList(Box::new(self.0.chain(iter::once(other))))
        } else {
            PlacementList(Box::new(self.0))
        }
    }

    /// consuming
    /// debugging tool to write all the fumens to a json file
    pub fn write_fumens(self, filename: &str) {
        let fumens = self.0.map(|p| p.get_fumen()).collect::<Vec<_>>().join("\n");
        let path = Path::new(filename);
        let mut file = File::create(path).unwrap();
        let _ = file.write_all(fumens.as_bytes());
    }
}
