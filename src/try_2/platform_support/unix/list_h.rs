//

// struct elem;

pub struct list {
    pub first: elem,
    pub last: elem,
    pub size: i32,
    pub elems: i32,
}

pub struct elem {
    // let mut next: &mut elem;
    pub data: Vec<u8>,
}

// list_new: &mut list(size: i32);
// list_push: i32(list: &mut list, data: &mut Vec<u8>);
// pub fn  *list_pop(list: &mut list);
// pub fn  *list_first(list: &mut list);
// list_elems: i32(list: &mut list);
// pub fn  list_delete(list: &mut list);
// list_remove: i32(list: &mut list, elem: &mut Vec<u8>);
// pub fn  list_map(list: &mut list, void (* func)(arg: &mut Vec<u8>));
