use super::*;

macro_rules! assert_some_rc {
    ($left:expr, $right:expr $(,)?) => {
        assert!($left.is_some_and(|rc_val| *rc_val == $right));
    };
}

#[test]
fn tc0_instantion_empty() {
    let empty_linked_list: List<i32> = List::from_array([]);
    assert_eq!(empty_linked_list.head(), None);
}

#[test]
fn tc1_instantion_single_zero() {
    let lists_with_single_zero = [List::new(0), List::from_array([0])];

    for tested_list in lists_with_single_zero {
        assert_some_rc!(tested_list.head(), 0);
    }
}

#[test]
fn tc2_append_insert_element() {
    let mut dynamic_list: List<i32> = List::from_array([]);
    assert!(dynamic_list.head().is_none());

    dynamic_list.append(0);
    assert_some_rc!(dynamic_list.head(), 0);

    dynamic_list.append(1);
    assert_some_rc!(dynamic_list.at(1), 1);

    assert!(dynamic_list.at(2).is_none());

    dynamic_list.insert_before(1, 3);
    assert_some_rc!(dynamic_list.at(1), 3);

    // insert before head elemnt
    dynamic_list.insert_before(0, 4);
    assert_some_rc!(dynamic_list.head(), 4);
    // see if the head points to the right element
    let test_iter = dynamic_list.head_iter();
    assert!(test_iter.is_some_and(|mut unwrapped_iter| {
        unwrapped_iter.next().is_some_and(|rc_value| *rc_value == 0)
    }));
}

#[test]
fn tc3_next_element() {
    let test_array = [0, 1, 2, 3];
    let test_list = List::from_array([0, 1, 2, 3]);
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
fn tc4_remove_element() {
    let mut test_list = List::from_array([0, 1, 2]);

    let rem_iter_opt = test_list.pos_iter_mut(1);
    assert!(rem_iter_opt.is_some());
    rem_iter_opt.unwrap().remove(); // remove consumes the iterator

    assert_some_rc!(test_list.at(0), 0);
    assert_some_rc!(test_list.at(1), 2);

    let rem_iter_opt2 = test_list.head_iter_mut();
    assert!(rem_iter_opt2.is_some());
    rem_iter_opt2.unwrap().remove();
    assert_some_rc!(test_list.at(0), 2);

    let rem_iter_opt2 = test_list.head_iter_mut();
    assert!(rem_iter_opt2.is_some());
    rem_iter_opt2.unwrap().remove(); // Remove last element
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

    strong_ref_list.pos_iter_mut(1).unwrap().remove();

    // Now the reference is decreased since the element in
    // the list doesn't exist anymore
    assert_eq!(Rc::strong_count(&element1), 1);

    // We also need to test the append after removal
    strong_ref_list.append_shared(Rc::new(4));
    assert_some_rc!(strong_ref_list.at(2), 4);
}

#[test]
fn tc5_test_remove_at() {
    let mut test_list = List::from_array([0, 1, 2]);

    test_list.remove_at(1);
    assert_some_rc!(test_list.at(0), 0);
    assert_some_rc!(test_list.at(1), 2);

    test_list.remove_at(0);
    assert_some_rc!(test_list.at(0), 2);
    test_list.remove_at(0);
    assert!(test_list.at(0).is_none());
}
