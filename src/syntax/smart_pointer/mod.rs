#![allow(dead_code, unused_variables)]

mod box_pointer;
mod deref_trait;
mod drop_trait;
mod rc_pointer;
mod refcell_pointer;
mod weak_pointer;

pub fn tests() {
    // box_pointer::tests();

    // deref_trait::tests();

    // drop_trait::tests();

    // rc_pointer::tests();

    // refcell_pointer::tests();

    weak_pointer::tests();
}
