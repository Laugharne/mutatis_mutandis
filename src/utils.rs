use std::process::Command;

pub fn shell_call(cmd: &str, args: &str) {
	let output = Command::new(cmd)
		.arg(args)
		.output()
		.expect("Fail to execute command");

	if output.status.success() {
		print!("{}", String::from_utf8_lossy(&output.stdout));
	} else {
		eprintln!("Execution error :\n{}", String::from_utf8_lossy(&output.stderr));
	}
}
