// Copyright Judica, Inc 2021
//
// This Source Code Form is subject to the terms of the Mozilla Public
//  License, v. 2.0. If a copy of the MPL was not distributed with this
//  file, You can obtain one at https://mozilla.org/MPL/2.0/.

//! Caches for guards
use super::Context;
use super::InternalCompilerTag;
use crate::contract::actions::Guard;
use sapio_base::effects::PathFragment;
use sapio_base::Clause;
use std::collections::HashMap;

pub(crate) enum CacheEntry<T> {
    Cached(Clause),
    Fresh(fn(&T, Context) -> Clause),
}

/// GuardCache assists with caching the computation of guard functions
/// during compilation.
pub(crate) struct GuardCache<T> {
    cache: HashMap<usize, Option<CacheEntry<T>>>,
}
impl<T> GuardCache<T> {
    pub fn new() -> Self {
        GuardCache {
            cache: HashMap::new(),
        }
    }
    pub(crate) fn create_entry(g: Option<Guard<T>>, t: &T, ctx: Context) -> Option<CacheEntry<T>> {
        Some(match g? {
            Guard::Cache(f) => CacheEntry::Cached(f(t, ctx)),
            Guard::Fresh(f) => CacheEntry::Fresh(f),
        })
    }
    pub(crate) fn get(
        &mut self,
        t: &T,
        f: fn() -> Option<Guard<T>>,
        ctx: Context,
    ) -> Option<Clause> {
        Some(
            match self
                .cache
                .entry(f as usize)
                .or_insert_with(|| {
                    Self::create_entry(
                        f(),
                        t,
                        ctx.internal_clone(InternalCompilerTag { _secret: () }),
                    )
                })
                .as_ref()?
            {
                CacheEntry::Cached(s) => s.clone(),
                CacheEntry::Fresh(f) => f(t, ctx),
            },
        )
    }
}

pub(crate) fn create_guards<T>(
    self_ref: &T,
    mut ctx: Context,
    guards: &[fn() -> Option<Guard<T>>],
    gc: &mut GuardCache<T>,
) -> Clause {
    guards
        .iter()
        .zip((0..).flat_map(|i| ctx.derive(PathFragment::Branch(i)).ok()))
        .filter_map(|(x, c)| gc.get(self_ref, *x, c))
        .filter(|x| *x != Clause::Trivial) // no point in using any Trivials
        .fold(Clause::Trivial, |acc, item| match acc {
            Clause::Trivial => item,
            _ => Clause::And(vec![acc, item]),
        })
}
