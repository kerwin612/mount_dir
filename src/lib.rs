extern crate win_subst;
extern crate random_string;

use std::env::{temp_dir};
use std::io::{Error, ErrorKind};
use std::path::{Component, Path};
use std::os::windows::fs::symlink_dir;
use std::fs::{create_dir_all, remove_dir_all};

use random_string::generate;
use win_subst::{add, del};

pub fn mount(tpath: &str, lpath: &str, force: bool) -> Result<bool, Error> {

    if ! Path::new(&tpath).exists() {
        return Err(Error::new(ErrorKind::NotFound, format!("[{}] not found", &tpath)));
    }

    if Path::new(&lpath).exists() {
        if force {
            match remove_dir_all(&lpath) {
                Err(e) => return Err(e),
                _ => (),
            }
        } else {
            return Err(Error::new(ErrorKind::AlreadyExists, format!("[{}] already exists", &lpath)));
        }
    }

    let ldisk = get_disk(lpath);

    if ! Path::new(&ldisk).exists() {
        let mut tmp_dir = temp_dir();
        tmp_dir.push(format!(".link.{}.tmp", generate(6, "1234567890")));
        let mpath = tmp_dir.into_os_string().as_os_str().to_str().unwrap().to_string();
        match create_dir_all(&mpath) {
            Err(e) => return Err(e),
            _ => (),
        }
        add(&ldisk, &mpath);
    }

    match symlink_dir(&tpath, &lpath) {
        Err(e) => return Err(e),
        _ => (),
    }

    return Ok(true);
}

pub fn unmount(link: &str) -> bool {
    return del(get_disk(link));
}

fn get_disk(path: &str) -> &str {
    match Path::new(path).components().next().unwrap() {
        Component::Prefix(prefix_component) => {
            return prefix_component.as_os_str().to_str().unwrap();
        }
        _ => unreachable!(),
    }
}

#[cfg(test)]
mod it_works {
    use super::*;
    use std::env;

    #[test]
    fn test_for_mount() {
        assert_eq!(mount(env::current_dir().unwrap().as_path().to_str().unwrap(), "T:\\work", true).unwrap(), true);
    }

    #[test]
    fn test_for_unmount() {
        assert_eq!(unmount("T:\\work"), true);
    }
}
