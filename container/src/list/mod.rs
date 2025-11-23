use std::rc::Rc;
use std::usize;
use std::{collections::VecDeque, vec};

mod list_pos;
use self::list_pos::ListPos;

mod list_pos_mut;
use self::list_pos_mut::MutListPos;

#[cfg(test)]
mod tests;


/// A single linked list that can be used for dynamic insertion/removal
///
/// The "default" usage of this type as a list is to use [`append`] to add to
/// the list. Iterating over `List` goes front to back.
/// 
///
/// A `List` with a known list of items can be initialized from an array:
///
/// ```
/// use container::List;
///
/// let mut linked_list = List::from_array([-1, 0, 1]);
/// linked_list.append(2);
/// ```
/// 
/// If you want to insert an element at a specific index you can use [`insert_before`]:
/// ```
/// use container::List;
///
/// let mut linked_list = List::from_array([-1, 0, 1]);
/// linked_list.insert_before(1, 2);
/// ```
/// A unique behavior of `List` is that it uses referenced counted elements, effectively allowing multi-ownership
/// of the stored data. While methods like [`append`] or [`insert_before`] take ownership of the passed element, the
/// element is stored as a referenced counted pointer. All methods taking an element have a correspoing method with
/// suffix _shared (like [`append_shared`]):
/// ```
/// use std::rc::Rc;
/// use container::List;
///
/// let mut linked_list = List::from_array([-1, 0, 1]);
/// let rc_element = Rc::new(2);
/// linked_list.append_shared(Rc::clone(&rc_element));
/// 
/// assert_eq!(Rc::strong_count(&rc_element), 2);
/// ```
/// This is more aligned to the behavior of a linked list.
///
/// [`append`]: List::append
/// [`append_shared`]: List::append_shared
/// [`insert_before`]: List::insert_before
pub struct List<T> {
    head_index: usize,
    tail_index: usize,
    all_elements: Vec<ListEntry<T>>,
    free_indices: VecDeque<usize>,
}

struct ListEntry<T> {
    hold_data: Rc<T>, // we hold a reference counted value here for convenience
    next_index: usize,
}

impl<T> List<T> {
    // Take full ownership
    pub fn new(first_element: T) -> List<T> {
        List {
            head_index: 0,
            tail_index: 0,
            all_elements: vec![ListEntry {
                hold_data: Rc::new(first_element),
                next_index: 0,
            }],
            free_indices: VecDeque::new(),
        }
    }

    pub fn empty() -> List<T> {
        List {
            head_index: 0,
            tail_index: 0,
            all_elements: Vec::new(),
            free_indices: VecDeque::new(),
        }
    }

    pub fn from_array<const COUNT: usize>(given_array: [T; COUNT]) -> List<T> {
        let mut instantiated_list = List::empty();

        if given_array.is_empty() {
            return instantiated_list;
        }

        let mut consumed_elements = VecDeque::from(given_array);

        for cur_index in 0..consumed_elements.len() {
            instantiated_list.all_elements.push(ListEntry {
                hold_data: Rc::new(consumed_elements.pop_front().unwrap()),
                next_index: cur_index + 1,
            });
        }
        instantiated_list.tail_index = instantiated_list.all_elements.len() - 1;
        instantiated_list
            .all_elements
            .last_mut()
            .unwrap()
            .next_index = instantiated_list.tail_index;

        instantiated_list
    }

    pub fn append(&mut self, element: T) {
        self.append_shared(Rc::new(element));
    }

    pub fn append_shared(&mut self, element: Rc<T>) {
        match self.free_indices.pop_front() {
            None => {
                match self.all_elements.last_mut() {
                    Some(last_element) => {
                        self.tail_index += 1;
                        last_element.next_index = self.tail_index;
                    }
                    None => {}
                }

                // tail index references itself
                self.all_elements.push(ListEntry {
                    hold_data: element,
                    next_index: self.tail_index,
                });
            }
            Some(free_index) => {
                self.all_elements[self.tail_index].next_index = free_index;
                self.tail_index = free_index;
                self.all_elements[free_index].hold_data = element;
                self.all_elements[free_index].next_index = free_index;
            }
        }
    }

    pub fn head(&self) -> Option<Rc<T>> {
        if self.len() == 0 {
            return None;
        }
        Some(Rc::clone(&self.all_elements[self.head_index].hold_data))
    }

    pub fn head_iter(&self) -> Option<ListPos<'_, T>> {
        ListPos::start_at(0, self)
    }

    pub fn head_iter_mut(&mut self) -> Option<MutListPos<'_, T>> {
        MutListPos::start_at(0, self)
    }

    pub fn iter(&self) -> ListPos<'_, T> {
        ListPos::new(self)
    }

    pub fn iter_mut(&mut self) -> MutListPos<'_, T> {
        MutListPos::new(self)
    }

    pub fn pos_iter(&self, pos: usize) -> Option<ListPos<'_, T>> {
        ListPos::start_at(pos, self)
    }

    pub fn pos_iter_mut(&mut self, pos: usize) -> Option<MutListPos<'_, T>> {
        MutListPos::start_at(pos, self)
    }

    pub fn at(&self, index: usize) -> Option<Rc<T>> {
        if index >= self.len() {
            return None;
        }

        let mut actual_insertion_index = self.head_index;
        for _ in 0..index {
            actual_insertion_index = self.all_elements[actual_insertion_index].next_index;
        }
        Some(Rc::clone(&self.all_elements[actual_insertion_index].hold_data))
    }

    pub fn insert_before(&mut self, insert_index: usize, element: T) {
        self.insert_before_shared(insert_index, Rc::new(element));
    }

    pub fn insert_before_shared(&mut self, insert_index: usize, element: Rc<T>) {
        if insert_index >= self.len() {
            return;
        }
        // this element will be the new head
        if insert_index == self.head_index {
            match self.free_indices.pop_front() {
                None => {
                    self.all_elements.push(ListEntry {
                        hold_data: element,
                        next_index: self.head_index,
                    });
                    self.head_index = self.all_elements.len() - 1;
                }
                Some(free_index) => {
                    self.all_elements[free_index].hold_data = element;
                    self.all_elements[free_index].next_index = self.head_index;
                    self.head_index = free_index;
                }
            };

            return;
        }

        let mut actual_insertion_index = self.head_index;
        let mut prev_insertion_index = self.head_index;

        // Go to the element before the referenced one
        for _ in 0..insert_index {
            prev_insertion_index = actual_insertion_index;
            actual_insertion_index = self.all_elements[actual_insertion_index].next_index;
        }
        let referenced_index = self.all_elements[actual_insertion_index].next_index;
        let inserted_element = ListEntry {
            hold_data: element,
            next_index: referenced_index,
        };

        match self.free_indices.pop_front() {
            None => {
                self.all_elements[prev_insertion_index].next_index = self.all_elements.len();
                self.all_elements.push(inserted_element);
            }
            Some(free_index) => {
                self.all_elements[prev_insertion_index].next_index = free_index;
                self.all_elements[free_index] = inserted_element;
            }
        };
    }

    pub fn len(&self) -> usize
    {
        // free_indices is always smaller or equal then all_elements
        self.all_elements.len() - self.free_indices.len()
    }

    pub fn remove_at(&mut self, index: usize)
        where T: Default
    {
        match self.pos_iter_mut(index) {
            Some(rem_iter) => {
                rem_iter.remove();
            }
            None => {}
        }
    }
}
