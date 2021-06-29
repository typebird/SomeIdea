use std::num::NonZeroUsize;

use rand::{self, distributions::Uniform, Rng};

pub fn get_random_number(min: usize, max: usize) -> usize {
    rand::thread_rng().gen_range(min..max)
}

pub fn get_random_item<T: Copy>(list: &Vec<T>) -> T {
    get_random_items::<T>(list, unsafe { NonZeroUsize::new_unchecked(1) })[0]
}

pub fn get_random_items<T: Copy>(list: &Vec<T>, number: NonZeroUsize) -> Vec<T> {
    let mut rng = rand::thread_rng();
    let range = Uniform::from(0..list.len());

    (0..number.get()).map(|_| list[rng.sample(range)]).collect()
}
