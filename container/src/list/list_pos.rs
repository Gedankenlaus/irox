use std::rc::Rc;

use super::List;

pub struct ListPos<'a, T> {
    position_index: Option<usize>,
    referenced_entry: &'a List<T>,
}

impl<'a, T> ListPos<'a, T> {
    pub fn new(ref_list: &'a List<T>) -> ListPos<'a, T> {
        ListPos {
            position_index: None,
            referenced_entry: ref_list,
        }
    }

    pub fn start_at(position: usize, ref_list: &'a List<T>) -> Option<ListPos<'a, T>> {
        let mut pos_index = ref_list.head_index;

        if position >= ref_list.len(){
            return None;
        }

        for _ in 0..position{
            pos_index = ref_list.all_elements[pos_index].next_index;
        }

        Some(ListPos {
            position_index: Some(pos_index),
            referenced_entry: ref_list,
        })
    }
}

impl<'a, T> Iterator for ListPos<'a, T> {
    type Item = Rc<T>;

    fn next(&mut self) -> Option<Self::Item> {
        let found_index: usize;
        match self.position_index {
            None => {
                if self.referenced_entry.all_elements.is_empty() {
                    return None;
                }
                found_index = self.referenced_entry.head_index;
            }
            Some(valid_pos_index) => {
                let cur_next_index = self.referenced_entry.all_elements[valid_pos_index].next_index;
                let tail_index = self.referenced_entry.tail_index;
                if valid_pos_index == tail_index && cur_next_index == tail_index {
                    return None;
                }
                found_index = cur_next_index;
            }
        }

        self.position_index.replace(found_index);
        let holded_data = Rc::clone(&self.referenced_entry.all_elements[found_index].hold_data);

        Some(holded_data)
    }
}
