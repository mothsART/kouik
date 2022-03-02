use nix::unistd::Pid;
use nix::sys::signal::{kill,Signal};
use crate::lib::Proc;

pub fn kill_proc_by_name(progname: &str, liste_procs: &Vec<Proc>) -> Result<u32,String> {

    let mut compteur : u32 = 0;
    let mut error_desc_trace : String = String::new();
	
    for process in liste_procs {
		for procname in &process.names {
			if procname == progname {
				match kill_proc(&process) {
                    None => compteur += 1,
                    Some(error_description) => {
                        error_desc_trace.push_str("process : ");
                        error_desc_trace.push_str(&process.pid.to_string());
                        error_desc_trace.push(' ');
                        error_desc_trace.push_str(&error_description);
                        error_desc_trace.push('\n');
                    },
                }
            }
        }
    }

    if error_desc_trace.is_empty() == false {
        return Err(error_desc_trace);
    }
    Ok(compteur)
}

pub fn kill_proc(processus: &Proc) -> Option<String> {
    let result_of_kill : nix::Result<()> = kill(Pid::from_raw(processus.pid),Signal::SIGTERM);
    if let Err(error) = result_of_kill {
        return Some(error.to_string());
    }
    None
}