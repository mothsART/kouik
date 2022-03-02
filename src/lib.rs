extern crate levenshtein;
use levenshtein::levenshtein;
use std::fs;
use std::path::Path;
use std::io::Error;

pub struct Proc {
	pub pid: i32,
	pub names: Vec<String>,
}

pub fn get_procs() -> Result<Vec<Proc>,Error> {

	let mut processus = Vec::<Proc>::new();

	for entry in fs::read_dir(Path::new("/proc"))? {

		let entry = entry?;
		let path = entry.path().join("exe");

		if let Ok(filename) = get_basename_symlink(&path) {

			if let Some(pathtmp) = entry.path().as_path().to_str() {

				if let Some(index) = pathtmp.to_string().rfind('/') {

					let pid: Result<i32,_> = pathtmp[index+1..].parse();

					match pid {
						Ok(pid_ok) => {
							let vec_s = vec![filename];
							let process = Proc {pid: pid_ok, names: vec_s};
							processus.push(process);
						},
						Err(_) => {},
					}
				}
			}
		}
	}
	Ok(processus)
}

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

pub struct LevensteinProc {
	pub proc : Proc,
	pub levensthein_distance :usize,
}

pub fn obtain_levensthein_distance(progname: &str, liste_procs: Vec<Proc>) -> Vec<LevensteinProc> {
	
	let mut leven_vec_of_proc = Vec::<LevensteinProc>::new();

	for processus in liste_procs {

		let mut min: usize = 999;

		for name in &processus.names {
			let dist = levenshtein(name,progname);
			if dist < min {
				min = dist;
			}
		}

		leven_vec_of_proc.push(LevensteinProc{proc: processus, levensthein_distance: min});
	}

	leven_vec_of_proc
}

