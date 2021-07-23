// https://link.springer.com/content/pdf/10.1007/s10479-016-2367-1.pdf
#![allow(dead_code)]

mod domain;
mod matrix;

use im::HashSet;
use itertools::Itertools;
use std::hash::Hash;

struct Recommender;

impl Recommender {
    pub fn recommend<'a, U, I, S>(
        user: &'a U,
        items: &'a [I],
        neighborhood: &'a [U],
        users: &'a [U],
        similarity_algorithm: S,
        n: usize,
    ) -> Vec<&'a I>
    where
        U: domain::User,
        I: domain::Item,
        S: domain::SimilarityAlgorithm<U>,
    {
        let user_domain = neighborhood;
        let (items, _): (Vec<_>, Vec<_>) = items
            .iter()
            .map(|i| Recommender::predict::<U, I, S>(user, i, user_domain))
            .sorted_by(|a, b| (a.1).partial_cmp(&b.1).expect("cannot compare"))
            .take(n)
            .unzip();

        items
    }

    /// Predict what _`user`_ would rate _`item`_ based on the other users
    /// who have rated also it.
    ///
    /// It uses the adjusted weighted sum approach.
    ///
    /// <br>
    ///
    /// _`user_domain`_ refers to the neighborhood of `user` and the `User`s who have rated `item`
    pub fn predict<'a, U, I, S>(user: &'a U, item: &'a I, user_domain: &[U]) -> (&'a I, f32)
    where
        U: domain::User,
        I: domain::Item,
        S: domain::SimilarityAlgorithm<U>,
    {
        let average_rating = user.average_ratings();
        let norm: f32 = 1. / {
            user_domain
                .iter()
                .map(|u| S::similarity(user, u))
                .sum::<f32>()
        };

        (
            item,
            average_rating
                + norm * {
                    user_domain
                        .iter()
                        .map(|u| {
                            S::similarity(user, u) * (u.rated(item).unwrap() - u.average_ratings())
                        })
                        .sum::<f32>()
                },
        )
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn it_works() {}
}
