use std::{
    cell::{Ref, RefCell},
};

#[derive(Debug)]
pub struct LookupTable<'a> {
    src: &'a str,
    line_br_indices: RefCell<Option<Vec<usize>>>,
}

impl<'a> LookupTable<'a> {
    pub fn new(src: &'a str) -> Self {
        Self {
            src,
            line_br_indices: RefCell::new(None),
        }
    }

    fn get_or_init(&self) -> Ref<'_, Vec<usize>> {
        self.line_br_indices.borrow_mut().get_or_insert_with(|| {
            self.src
                .bytes()
                .enumerate()
                .filter_map(|(i, by)| Some(i).filter(|_|by == b'\n'))
                .collect()
        });


    }
}
