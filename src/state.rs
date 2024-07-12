use serde::Deserialize;
use std::{collections::HashMap, hash::Hash};

pub trait AbstractState {
    fn matches(&self, other: &Self) -> bool;
}

/// Ignored Fileds
#[derive(Debug)]
pub struct Unmatched<T>(pub T);

impl<T> AbstractState for Unmatched<T> {
    fn matches(&self, _other: &Self) -> bool {
        true
    }
}

/// Common Value
#[derive(Deserialize, Debug)]
pub struct Value<T>(pub T)
where
    T: PartialEq;

impl<T> AbstractState for Value<T>
where
    T: PartialEq,
{
    /// Values match if they are equal
    fn matches(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

/// Ordered List of Values
#[derive(Deserialize, Debug)]
pub struct ValueList<T>(pub Vec<Value<T>>)
where
    T: PartialEq;

impl<T> AbstractState for ValueList<T>
where
    T: PartialEq,
{
    fn matches(&self, other: &Self) -> bool {
        if self.0.len() != other.0.len() {
            return false;
        }
        self.0.iter().zip(other.0.iter()).all(|(a, b)| a.matches(b))
    }
}

/// Unordered Set of Values
#[derive(Deserialize, Debug)]
pub struct ValueSet<T>(pub Vec<Value<T>>)
where
    T: PartialEq;

impl<T> AbstractState for ValueSet<T>
where
    T: PartialEq,
{
    fn matches(&self, other: &Self) -> bool {
        if self.0.len() != other.0.len() {
            return false;
        }
        self.0.iter().any(|a| other.0.iter().any(|b| a.matches(b)))
    }
}

/// Common Identifier
#[derive(Debug, Deserialize, Clone)]
pub struct Ident<T>(pub T);

impl<T> AbstractState for Ident<T> {
    /// Single Identifier always matches
    fn matches(&self, _other: &Self) -> bool {
        return true;
    }
}

/// Ordered List of Identifiers
#[derive(Debug, Deserialize)]
pub struct IdentList<T>(pub Vec<Ident<T>>)
where
    T: Hash + Eq;

impl<T> AbstractState for IdentList<T>
where
    T: Hash + Eq,
{
    fn matches(&self, other: &Self) -> bool {
        if self.0.len() != other.0.len() {
            return false;
        }
        map_ident(&self.0) == map_ident(&other.0)
    }
}

/// Unordered Set of Identifiers
#[derive(Debug, Deserialize)]
pub struct IdentSet<T>(pub Vec<Ident<T>>)
where
    T: Hash + Eq;

impl<T> AbstractState for IdentSet<T>
where
    T: Hash + Eq,
{
    fn matches(&self, other: &Self) -> bool {
        if self.0.len() != other.0.len() {
            return false;
        }
        let mut self_mapped = map_ident(&self.0);
        let mut other_mapped = map_ident(&other.0);
        self_mapped.sort();
        other_mapped.sort();
        self_mapped == other_mapped
    }
}

fn map_ident<T>(list: &Vec<Ident<T>>) -> Vec<usize>
where
    T: Hash + Eq,
{
    let mut map = HashMap::new();
    list.iter().for_each(|e| {
        if !map.contains_key(&e.0) {
            map.insert(&e.0, map.len());
        }
    });
    let mut mapped = Vec::new();
    for i in 0..list.len() {
        mapped.push(*map.get(&list[i].0).unwrap());
    }
    mapped
}
