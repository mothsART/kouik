use nix::unistd::Pid;
use nix::sys::signal::{kill,Signal};
use crate::lib::Proc;

pub fn kill_proc_by_name(progname: &str, liste_procs: &Vec<Proc>) -> u32 {
	let mut compteur : u32 = 0;
	for process in liste_procs {
		for procname in &process.names {
			if procname == progname {
				match kill(Pid::from_raw(process.pid),Signal::SIGTERM) {
                    Ok(_) => compteur += 1,
                    Err(e) => println!("kill send : {:?}",e),
                }
            }
        }
    }
    compteur
}

pub fn kill_proc(processus: &Proc) -> Option<String> {
    let result_of_kill : nix::Result<()> = kill(Pid::from_raw(processus.pid),Signal::SIGTERM);
    if let Err(error) = result_of_kill {
        return Some(error.to_string());
    }
    return None;
}