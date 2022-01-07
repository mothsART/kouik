use nix::unistd::Pid;
use nix::sys::signal::{kill,Signal};
use crate::lib::Proc;

pub fn kill_proc_by_name(progname: String, liste_procs: Vec<Proc>) -> u32 {
	let mut compteur : u32 = 0;
	for process in liste_procs {
		for procname in process.names {
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