use std::collections::HashSet;
use std::hash::Hash;

/*
 * This function need refactor.
 */
pub fn multi_intersections(lists: Vec<Vec<usize>>) -> Vec<usize> {
    let a = lists[0].clone().into_iter().collect::<HashSet<_>>();
    let b = lists[1].clone().into_iter().collect::<HashSet<_>>();
    let c = lists[2].clone().into_iter().collect::<HashSet<_>>();
    let g = intersections(&[&a, &b, &c]);
    let apa: Vec<usize> = g.into_iter().collect();
    apa
}

/*
 * Return an inverse.
 */
pub fn inverse_vec(input: &[usize]) -> Vec<usize> {
    vec![1, 2, 3, 4, 5, 6, 7, 8, 9]
        .into_iter()
        .filter(|item| !input.contains(item))
        .collect()
}

// From https://users.rust-lang.org/t/intersection-of-multiple-hashsets/85318/2
pub fn intersections<T>(sets: &[&HashSet<T>]) -> HashSet<T>
where
    T: Clone + Eq + Hash,
{
    match sets.len() {
        0 => HashSet::new(),
        _ => sets[1..].iter().fold(sets[0].clone(), |mut acc, set| {
            acc.retain(|item| set.contains(item));
            acc
        }),
    }
}

/*
 * Remove element from Vector
 */
pub fn remove_element(element: usize, vector: &mut Vec<usize>) -> bool {
    match vector.iter().position(|x| *x == element) {
        Some(index) => {
            vector.remove(index);
            true
        }
        None => false,
    }
}

/*
 * Custom function to sort vector
 */
pub fn compare(a: &(&usize, &Vec<usize>), b: &(&usize, &Vec<usize>)) -> std::cmp::Ordering {
    let (_, av) = a;
    let (_, bv) = b;

    if av.len() < bv.len() {
        return std::cmp::Ordering::Greater;
    }

    if av.len() == bv.len() {
        return std::cmp::Ordering::Equal;
    }
    std::cmp::Ordering::Less
}
