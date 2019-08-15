use rusoto_core::Region;
use rusoto_ec2::{
    DescribeImagesRequest, DescribeInstancesRequest, Ec2, Ec2Client, Filter, Image,
    Instance,
};
use std::collections::HashMap;
use std::process::Command;
use std::str::FromStr;

mod config;

fn main() {
    let region = Region::from_str(
        &config::request_string("insert region: ")
            .expect("error while requesting region"),
    )
    .expect("error while parsing region");
    let ec2_client = Ec2Client::new(region.clone());
    let request = DescribeInstancesRequest::default();

    let mut public_address_by_name = HashMap::new();
    let mut image_ids_by_name = HashMap::new();
    match ec2_client.describe_instances(request).sync() {
        Ok(response) => {
            // TODO next_token
            if let Some(reservations) = response.reservations {
                reservations
                    .iter()
                    .flat_map(|reservation| reservation.instances.iter())
                    .flat_map(|instances| instances.iter())
                    .for_each(|instance| {
                        let name = extract_name(&instance).unwrap_or("<unnamed>");
                        let public_dns =
                            instance.public_ip_address.as_ref().unwrap_or_else(|| {
                                instance.public_dns_name.as_ref().unwrap()
                            });

                        if public_dns != "" {
                            println!("{}", name);
                            public_address_by_name
                                .insert(name.to_string(), public_dns.to_string());
                        }
                        if let Some(image_id) = &instance.image_id {
                            image_ids_by_name
                                .insert(name.to_string(), image_id.to_string());
                        }
                    })
            }
        }
        Err(error) => {
            println!("error: {:?}", error);
        }
    }

    let selected_instance = config::request_string("insert instance name: ")
        .expect("error while requesting instance name");
    let ssh_key_filepath = config::request_ssh_key_path(&region)
        .expect("error while requesting ssh key filepath");
    match public_address_by_name.get(&selected_instance[..]) {
        Some(public_address) => {
            let image_request = DescribeImagesRequest {
                filters: Some(vec![Filter {
                    name: Some("image-id".to_string()),
                    values: Some(vec![(&image_ids_by_name.get(&selected_instance))
                        .unwrap()
                        .to_string()]),
                }]),
                ..DescribeImagesRequest::default()
            };
            let user_name = ec2_client
                .describe_images(image_request)
                .sync()
                .map(|response| extract_user_name(&response.images.unwrap()[0]))
                .unwrap_or("ec2-user".to_string());

            let mut child = Command::new("ssh")
                .arg("-i")
                .arg(ssh_key_filepath)
                .arg(format!("{}@{}", user_name, public_address))
                .spawn()
                .expect("ssh failed to start");
            child.wait().expect("failed to wait ssh");
        }
        None => println!("instance name {} not known", selected_instance),
    }
}

fn extract_name(instance: &Instance) -> Option<&str> {
    match &instance.tags {
        Some(tags) => tags
            .iter()
            .find(|tag| tag.key == Some("Name".to_string()))
            .and_then(|tag| tag.value.as_ref())
            .map(|name| &name[..]),
        None => None,
    }
}

fn extract_user_name(image: &Image) -> String {
    if let Some(image_name) = &image.name {
        if image_name.starts_with("ubuntu") {
            "ubuntu".to_string()
        } else {
            "ec2-user".to_string()
        }
    } else {
        "ec2-user".to_string()
    }
}
