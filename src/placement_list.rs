use auto_enums::auto_enum;
use rayon::prelude::*;

use crate::placement::*;
use crate::pruner::*;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

pub struct PlacementList<I>(pub I);

impl<I: ParallelIterator<Item = Placement>> ParallelIterator for PlacementList<I> {
    type Item = Placement;

    fn drive_unindexed<C>(self, consumer: C) -> C::Result
    where
        C: rayon::iter::plumbing::UnindexedConsumer<Self::Item>,
    {
        self.0.drive_unindexed(consumer)
    }
}

impl<I: IndexedParallelIterator<Item = Placement>> IndexedParallelIterator for PlacementList<I> {
    fn len(&self) -> usize {
        self.0.len()
    }

    fn drive<C: rayon::iter::plumbing::Consumer<Self::Item>>(self, consumer: C) -> C::Result {
        self.0.drive(consumer)
    }

    fn with_producer<CB: rayon::iter::plumbing::ProducerCallback<Self::Item>>(
        self,
        callback: CB,
    ) -> CB::Output {
        self.0.with_producer(callback)
    }
}

// blanket implementation to convert all iterators over Placements into PlacementList
pub trait IntoPlacements {
    type I: ParallelIterator<Item = Placement>;
    fn into_placements(self) -> PlacementList<Self::I>;
    fn into_par_iter(self) -> Self::I;
}

impl<I: ParallelIterator<Item = Placement>> IntoPlacements for I {
    type I = I;
    fn into_placements(self) -> PlacementList<I> {
        PlacementList(self)
    }

    fn into_par_iter(self) -> I {
        self.into_placements().0
    }
}

impl<'a, I: IndexedParallelIterator<Item = Placement> + 'a> PlacementList<I> {
    /// prunes a list of placements and
    /// creates an instance of placement list from
    /// the filtered list

    pub fn new(placements: I, _pruner: &impl Pruner) -> impl IntoPlacements {
        // Self(placements.into_iter().filter(|x| pruner.prune(x)).into_placements::<I>())
        placements
    }

    pub fn empty() -> impl IntoPlacements {
        rayon::iter::empty()
    }

    /// extends each vector field within the placement list
    pub fn extend<J>(self, other: PlacementList<J>) -> impl IntoPlacements
    where
        J: IndexedParallelIterator<Item = Placement>,
    {
        self.0.chain(other.0).into_placements()
    }

    #[auto_enum(rayon::IndexedParallelIterator)]
    fn _add(self, other: Placement, pruner: &impl Pruner) -> impl IntoPlacements {
        if pruner.precondition(&other) {
            self.0.chain(rayon::iter::once(other))
        } else {
            // self
            self.0
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
