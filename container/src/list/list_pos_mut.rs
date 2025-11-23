use std::rc::Rc;

use super::List;

// Implement one for Mutable and Const references
pub struct MutListPos<'a, T> {
    position_index: Option<usize>,
    referenced_entry: &'a mut List<T>,
}

impl<'a, T> MutListPos<'a, T> {
    pub fn new(ref_list: &'a mut List<T>) -> MutListPos<'a, T> {
        MutListPos {
            position_index: None,
            referenced_entry: ref_list,
        }
    }

    pub fn start_at(position: usize, ref_list: &'a mut List<T>) -> MutListPos<'a, T> {
        let mut pos_index = ref_list.head_index;
        for _ in 0..position{
            pos_index = ref_list.all_elements[pos_index].next_index;
        }

        MutListPos {
            position_index: Some(pos_index),
            referenced_entry: ref_list,
        }
    }

    pub fn remove(self)
        where T: Default // we need default for T to clear the element
    {
        // here a prev index would be cool
        match self.position_index{
            Some(current_position) => {
                let head_index = self.referenced_entry.head_index;
                let list_entries = &mut self.referenced_entry.all_elements;
                let free_indices = &mut self.referenced_entry.free_indices;
                
                // if we are at the head index we just need to update the head index
                if current_position == self.referenced_entry.head_index {
                    // we just need to update the head index
                    let next_index = list_entries[head_index].next_index;
                    self.referenced_entry.head_index = next_index;
                }
                else {
                    // find previous index
                    let mut prev_index = head_index;

                    while list_entries[prev_index].next_index != current_position
                    {
                        prev_index = list_entries[prev_index].next_index;
                    }

                    list_entries[prev_index].next_index = list_entries[current_position].next_index;   
                }
                // Clear data with remove
                list_entries[current_position].hold_data = Rc::default();

                // current entry is free, so the entry is free again
                free_indices.push_back(current_position);
            }
            // Iterator points nowhere, so there is nothing to do
            None => {}
        }
    }
}

impl<'a, T> Iterator for MutListPos<'a, T> {
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
