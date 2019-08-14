
use std::fs;
use std::vec::Vec;
use generic_error::*;
use std::path::Path;

pub fn walk_dir(path: &str) -> Result<Vec<fs::DirEntry>> {
	let mut items: Vec<fs::DirEntry> = Vec::new();
	walk_dir_recursive(path, &mut items)?;
	Ok(items)
}


fn walk_dir_recursive(path: &str, items: &mut Vec<fs::DirEntry>) -> Result<()> {
	for item in fs::read_dir(path)? {
		let item = item?;
		items.push(item);
		let item = items[..].last().unwrap();
		if item.file_type()?.is_dir() {
			if let Some(path) = item.path().to_str() {
				walk_dir_recursive(path, items)?;
			}
		}
	}
	Ok(())
}


pub fn copy_dir_soft(src: &str, dest: &str, allow_failures: usize) -> Result<Vec<String>> {
	let walk = walk_dir(src)?;
	
	let mut failures: Vec<String> = Vec::new();
	
	for entry in walk.iter() {
		//let entry = entry.unwrap();
		let dest = Path::new(dest);
		let ftype = entry.file_type()?;
		let path = entry.path();
		
		let rel_path = path.strip_prefix(src)?;
		let dest_path = dest.join(rel_path);
		let path = path.to_str().unwrap();
		if ftype.is_dir() {
			fs::create_dir_all(dest_path)?;
		}
		else {
			if allow_failures > 0 {
				if let Err(_) = fs::copy(path, dest_path) {
					failures.push(String::from(path));
				}
			}
			else {
				fs::copy(path, dest_path)?;
			}
		}
	}
	if failures.len() > allow_failures {
		GenErr!("Copy of {} failed; could not move {} files", src, failures.len())
	}
	else {
		Ok(failures)
	}
}


pub fn copy_dir(src: &str, dest: &str) -> Result<()> {
	match copy_dir_soft(src, dest, 0) {
		Ok(_) => Ok(()),
		Err(e) => Err(e),
	}
}
