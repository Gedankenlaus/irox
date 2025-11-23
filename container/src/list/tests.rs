use super::*;

#[test]
fn tc0_instantion_empty() {
    let empty_linked_list: List<i32> = List::from_array([]);
    assert_eq!(empty_linked_list.head(), None);
}

#[test]
fn tc1_instantion_single_zero() {
    let lists_with_single_zero = [List::new(0), List::from_array([0])];

    for tested_list in lists_with_single_zero {
        assert_eq!(*tested_list.head().unwrap(), 0);
    }
}

#[test]
fn tc2_append_insert_element() {
    let mut dynamic_list: List<i32> = List::from_array([]);
    assert_eq!(dynamic_list.head(), None);

    dynamic_list.append(0);
    assert_eq!(*dynamic_list.head().unwrap(), *dynamic_list.at(0).unwrap());

    dynamic_list.append(1);
    assert_eq!(*dynamic_list.at(1).unwrap(), 1);

    assert_eq!(dynamic_list.at(2), None);

    dynamic_list.insert_before(1, 3);
    assert_eq!(*dynamic_list.at(1).unwrap(), 3);

    // insert before head elemnt
    dynamic_list.insert_before(0, 4);
    assert_eq!(*dynamic_list.head().unwrap(), 4);
    // see if the head points to the right element
    assert_eq!(*dynamic_list.head_iter().next().unwrap(), 0);
}

#[test]
fn tc3_test_next_element() {
    let test_array = [0, 1, 2, 3];
    let mut test_list = List::from_array([0, 1, 2, 3]);
    let mut current_iter = test_list.iter();

    assert_eq!(current_iter.next(), Some(0.into()));
    assert_eq!(current_iter.next(), Some(1.into()));
    assert_eq!(current_iter.next(), Some(2.into()));
    assert_eq!(current_iter.next(), Some(3.into()));
    assert_eq!(current_iter.next(), None);

    for (index, data) in test_list.iter().enumerate() {
        assert_eq!(*data, test_array[index]);
    }
}

#[test]
fn tc4_test_remove_element() {
    let mut test_list = List::from_array([0, 1, 2]);

    // Remove the 1
    test_list.pos_iter_mut(1).remove(); // remove consumes the iterator

    assert_eq!(*test_list.at(0).unwrap(), 0);
    assert_eq!(*test_list.at(1).unwrap(), 2);

    test_list.head_iter_mut().remove(); // Remove the head 0
    assert_eq!(*test_list.at(0).unwrap(), 2);

    test_list.head_iter_mut().remove(); // Remove last element
    assert_eq!(test_list.at(0), None);

    // We need now verify whether the removal also removes the ownership of
    // the Rc value

    let element0 = Rc::new(1);
    let element1 = Rc::new(2);
    let element2 = Rc::new(3);

    let mut strong_ref_list = List::empty();

    strong_ref_list.append_shared(Rc::clone(&element0));
    strong_ref_list.append_shared(Rc::clone(&element1));
    strong_ref_list.append_shared(Rc::clone(&element2));

    assert_eq!(Rc::strong_count(&element0), 2);
    assert_eq!(Rc::strong_count(&element1), 2);
    assert_eq!(Rc::strong_count(&element2), 2);

    strong_ref_list.pos_iter_mut(1).remove();
    // Now the reference is decreased since the element in
    // the list doesn't exist anymore
    assert_eq!(Rc::strong_count(&element1), 1);

    // We also need to test the removal and append
    strong_ref_list.append_shared(Rc::new(4));
    assert_eq!(*strong_ref_list.at(2).unwrap(), 4);
    //
}

#[test]
fn tc5_test_expand_list() {}
