/*********************************************************************
 *
 * Finances Manager -- A command line utility to manage domestic financial
 * activities.
 *
 * Copyright (C) 2024
 *
 * This file is part of Finances manager. The full code is available at:
 *      https://github.com/lluisalemanypuig/finances_manager.git
 *
 * Finances Manager is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as published
 * by the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * Finances Manager is distributed in the hope that it will be useful, but
 * WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY
 * or FITNESS FOR A PARTICULAR PURPOSE. See the GNU Affero General Public
 * License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with Finances Manager.  If not, see <http://www.gnu.org/licenses/>.
 *
 * Contact:
 *
 *     Lluís Alemany Puig
 *         email: lluis.alemany.puig@gmail.com
 *         https://github.com/lluisalemanypuig
 *         lluisalemanypuig.github.io
 *
 ********************************************************************/

use std::str::FromStr;
use std::io::Write;

use crate::concepts::tree::Tree;

pub fn read_input_string(header: &str) -> String {
	if header != "" {
		print!("{}: ", header);
		let _ = std::io::stdout().flush().unwrap();
	}
	let mut s = String::new();
	let stdin = std::io::stdin();
	stdin
		.read_line(&mut s)
		.expect("I was expecting standard input");
	s.trim().to_string()
}

pub fn read_string_or_empty(header: &str) -> Option<String> {
	let str = read_input_string(header);
	if str == "".to_string() {
		return None;
	}
	Some(str)
}

pub fn read_string(header: &str) -> String {
	loop {
		if let Some(str) = read_string_or_empty(header) {
			break str;
		}
	}
}

pub trait Numeric: FromStr {}
impl Numeric for u8 {}
impl Numeric for u16 {}
impl Numeric for u32 {}
impl Numeric for u64 {}
impl Numeric for i8 {}
impl Numeric for i16 {}
impl Numeric for i32 {}
impl Numeric for i64 {}
impl Numeric for usize {}
impl Numeric for f32 {}
impl Numeric for f64 {}

pub fn read_num_or_empty<T>(header: &str) -> Option<T>
where
	T: Numeric,
{
	loop {
		if let Some(str) = read_string_or_empty(header) {
			if let Ok(value) = str.parse::<T>() {
				break Some(value);
			}
		} else {
			break None;
		}
	}
}

pub fn read_num<T: FromStr>(header: &str) -> T
where
	T: Numeric,
{
	loop {
		if let Ok(value) = read_string(header).parse::<T>() {
			return value;
		}
	}
}

pub trait Integral: Numeric {}
impl Integral for u8 {}
impl Integral for u16 {}
impl Integral for u32 {}
impl Integral for u64 {}
impl Integral for usize {}

pub fn read_int_or_empty<T: FromStr>(s: &str) -> Option<T>
where
	T: Integral,
{
	read_num_or_empty::<T>(s)
}

pub fn read_int<T: FromStr>(s: &str) -> T
where
	T: Integral,
{
	read_num::<T>(s)
}

pub trait Decimal: Numeric {}
impl Decimal for f32 {}
impl Decimal for f64 {}

pub fn read_float_or_empty<T: FromStr>(s: &str) -> Option<T>
where
	T: Decimal,
{
	read_num_or_empty::<T>(s)
}

pub fn read_float<T: FromStr>(s: &str) -> T
where
	T: Decimal,
{
	read_num::<T>(s)
}

/* ------------------------------------------------------------------------- */

pub fn read_from_options_or_empty(header: &str, options: &Vec<String>) -> Option<String> {
	loop {
		if let Some(str) = read_string_or_empty(header) {
			if str == "?".to_string() {
				for opt in options.iter() {
					println!("    {opt}");
				}
				println!("");
			} else if options.contains(&str) {
				return Some(str);
			}
		} else {
			return None;
		}
	}
}

pub fn read_from_tree_options(header: &str, options: &Tree) -> Vec<String> {
	let mut res: Vec<String> = Vec::new();

	let available = options.get_keys().iter().map(|s| s.to_string()).collect();
	let opt = read_from_options_or_empty(header, &available);

	if let Some(s) = opt {
		let st = options.get_child(&s);
		res.push(s);

		if let Some(stt) = st {
			let mut more = read_from_tree_options(header, &stt);
			res.append(&mut more);
		}
	}

	res
}
