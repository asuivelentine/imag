/*
 *  imag - The personal information management suite for the commandline
 *  Copyright (C) 2016    Matthias Beyer <mail@beyermatthias.de>
 *
 *  This library is free software; you can redistribute it and/or
 *  modify it under the terms of the GNU Lesser General Public
 *  License as published by the Free Software Foundation; version
 *  2.1 of the License.
 *
 *  This library is distributed in the hope that it will be useful,
 *  but WITHOUT ANY WARRANTY; without even the implied warranty of
 *  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
 *  Lesser General Public License for more details.
 *
 *  You should have received a copy of the GNU Lesser General Public
 *  License along with this library; if not, write to the Free Software
 *  Foundation, Inc., 51 Franklin Street, Fifth Floor, Boston, MA  02110-1301  USA
 */

use std::path::PathBuf;

use libimagstore::store::Entry;

pub trait IsInDiary {

    fn is_in_diary(&self, name: &str) -> bool;

}

impl IsInDiary for Entry {

    fn is_in_diary(&self, name: &str) -> bool {
        self.get_location().is_in_diary(name)
    }

}

impl IsInDiary for PathBuf {

    fn is_in_diary(&self, name: &str) -> bool {
        self.to_str().map(|s| s.contains(name)).unwrap_or(false)
    }

}

