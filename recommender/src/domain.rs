/// Domain
///
/// Just contains base traits for our recommender system
///
pub trait User {
    fn average_ratings(&self) -> f32;
    fn rated<I>(&self, item: &I) -> Option<f32>
    where
        I: Item;
    fn rated_items<I>(&self) -> &[I]
    where
        I: Item;
}

pub trait SimilarityAlgorithm<U: User> {
    fn similarity(u1: &U, u2: &U) -> f32;
}

pub trait Item {}
