use std::option::Option;
use std::fs;
use std::path::Path;

pub fn find_program(progname: &str) -> Result<Option<Vec<String>>,std::io::Error> {
	let mut list_pid = Vec::new();
    for entry in fs::read_dir(Path::new("/proc"))? {
            let entry = entry?;
            let path = entry.path().join("exe");
            if let Ok(filename) = get_basename_symlink(&path) {
            	// println!("{}", filename); // debug
            	if filename == progname.to_string() {
            		if let Some(pathtmp) = entry.path().as_path().to_str() {
            			if let Some(index) = pathtmp.to_string().rfind("/") {
            				let pid : String = pathtmp[index+1..].to_string();
            				list_pid.push(pid);
            			}
            		}
            	}
            }
    }
    if list_pid.is_empty() {
    	Ok(None)
    } else {
    	Ok(Some(list_pid))
    }
}
/*
pub fn find_program() {
	/* for every directory in /proc */
	get_procs();
		/* get basename of symlink pid/exe */
			/* if it's same as program name */
				/* add in a list */
}*/

pub fn get_basename_symlink(entry: &Path) -> Result<String,&'static str> {
	if let Ok(path) = fs::read_link(entry) {
		if let Some(filename) = path.file_name() {
			if let Some(file) = filename.to_str() {
				return Ok(file.to_string());
			}
		}
	}
	Err("impossible d'obtenir le nom du lien symbolique : permission non accordée")
}