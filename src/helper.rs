use std::process::{Command, Stdio};

pub fn get_dev_name()-> String {
    let mut command = Command::new("sh");
    command.arg("-c");
    let command_str = r#"(
        dir=$(
            ls -l /proc/*/exe 2>/dev/null |
            grep -E '/data/[^/]* \(deleted\)' |
            sed 's/ /\n/g' |
            grep '/proc' |
            sed 's/\/[^/]*$//g'
        );
        if [[ "$dir" ]]; then
            sbwj=$(head -n 1 "$dir/comm");
            open_file="";
            for file in "$dir"/fd/*; do
                link=$(readlink "$file");
                if [[ "$link" == "/dev/$sbwj (deleted)" ]]; then
                    open_file="$file";
                    break;
                fi;
            done;
            if [[ -n "$open_file" ]]; then
                nhjd=$(echo "$open_file");
                sbid=$(ls -L -l "$nhjd" | sed 's/[^,]*,//' | sed 's/.*root //');
                echo "/dev/$sbwj";
                rm -rf "/dev/$sbwj";
                mknod "/dev/$sbwj" c "$sbid" 0;
            fi;
        fi
    )"#
    .replace("\n", " ");
    command.arg(command_str);
    match command.output() {
        Ok(output) => String::from_utf8(output.stdout).unwrap().replace("\n", ""),
        Err(_) => String::from(""),
    }
}


pub fn kernel_version() -> Option<f64> {

    let command = "uname -r | sed 's/\\.[^.]*$//g'";


    let output = Command::new("sh")

        .arg("-c")

        .arg(command)

        .stdout(Stdio::piped())

        .output()

        .ok()?;


    let binding = String::from_utf8_lossy(&output.stdout);
    let version_line = binding.lines().next()?;

    let version = version_line.trim_end_matches(char::is_whitespace).to_string();


    version.parse::<f64>().ok()

}