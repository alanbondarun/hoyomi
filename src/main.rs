use rusoto_core::Region;
use std::process::Command;
use std::str::FromStr;

mod client;
mod config;

fn main() {
    let region = Region::from_str(
        &config::request_string("insert region: ")
            .expect("error while requesting region"),
    )
    .expect("error while parsing region");

    let client = crate::client::Client::new(&region);
    let descriptions_by_name = client.get_descriptions_by_name();

    for instance_name in descriptions_by_name.keys() {
        println!("instance: {}", instance_name);
    }

    let selected_instance = config::request_string("insert instance name: ")
        .expect("error while requesting instance name");
    let ssh_key_filepath = config::request_ssh_key_path(&region)
        .expect("error while requesting ssh key filepath");
    match descriptions_by_name.get(&selected_instance) {
        Some(description) => {
            let user_name = client.get_user_name(description);

            let mut child = Command::new("ssh")
                .arg("-i")
                .arg(ssh_key_filepath)
                .arg(format!(
                    "{}@{}",
                    user_name,
                    description.public_address.as_ref().unwrap()
                ))
                .spawn()
                .expect("ssh failed to start");
            child.wait().expect("failed to wait ssh");
        }
        None => println!("instance name {} not known", selected_instance),
    }
}
