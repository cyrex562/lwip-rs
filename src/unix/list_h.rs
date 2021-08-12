

// #define LWIP_LIST_H

struct elem;

struct list {
  first: &mut elem, *last;
  size: i32, elems;
};

struct elem {
  next: &mut elem;
  data: &mut ();
};

list_new: &mut list(size: i32);
list_push: i32(list: &mut list, data: &mut ());
pub fn  *list_pop(list: &mut list);
pub fn  *list_first(list: &mut list);
list_elems: i32(list: &mut list);
pub fn  list_delete(list: &mut list);
list_remove: i32(list: &mut list, elem: &mut ());
pub fn  list_map(list: &mut list, void (* func)(arg: &mut Vec<u8>));


