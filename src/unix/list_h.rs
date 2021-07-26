

#define LWIP_LIST_H

struct elem;

struct list {
  first: &mut elem, *last;
  size: int, elems;
};

struct elem {
  next: &mut elem;
  void *data;
};

list_new: &mut list(size: int);
list_push: int(list: &mut list, void *data);
pub fn  *list_pop(list: &mut list);
pub fn  *list_first(list: &mut list);
list_elems: int(list: &mut list);
pub fn  list_delete(list: &mut list);
list_remove: int(list: &mut list, void *elem);
pub fn  list_map(list: &mut list, void (* func)(arg: &mut Vec<u8>));


