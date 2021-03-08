// Take a look at the license at the top of the repository in the LICENSE file.

use glib::subclass::prelude::*;

use super::container::ContainerImpl;
use crate::Container;
use crate::Fixed;

pub trait FixedImpl: ContainerImpl {}

unsafe impl<T: FixedImpl> IsSubclassable<T> for Fixed {
    fn class_init(class: &mut ::glib::Class<Self>) {
        <Container as IsSubclassable<T>>::class_init(class);
    }
}
