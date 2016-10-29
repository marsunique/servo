/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */
use dom::element::Element;
use dom::validitystate::ValidityStatus;

pub trait Validatable {
    fn as_element_validatable(&self) -> &Element;
    fn is_instance_validatable(&self) -> bool;
}
