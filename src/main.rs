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

    let mut descriptions_by_name = HashMap::new();
    match ec2_client.describe_instances(request).sync() {
        Ok(response) => {
            // TODO next_token
            if let Some(reservations) = response.reservations {
                reservations
                    .iter()
                    .flat_map(|reservation| reservation.instances.iter())
                    .flat_map(|instances| instances.iter())
                    .for_each(|instance| {
                        let description = extract_description(&instance);
                        descriptions_by_name
                            .insert(description.name.clone(), description);
                    })
            }
        }
        Err(error) => {
            println!("error: {:?}", error);
        }
    }

    for instance_name in descriptions_by_name.keys() {
        println!("instance: {}", instance_name);
    }

    let selected_instance = config::request_string("insert instance name: ")
        .expect("error while requesting instance name");
    let ssh_key_filepath = config::request_ssh_key_path(&region)
        .expect("error while requesting ssh key filepath");
    match descriptions_by_name.get(&selected_instance) {
        Some(description) => {
            let user_name = get_user_name(ec2_client, description);

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

struct InstanceDescription {
    name: String,
    public_address: Option<String>,
    image_id: Option<String>,
}

fn extract_description(instance: &Instance) -> InstanceDescription {
    let name = extract_name(&instance).unwrap_or("<unnamed>");
    let public_dns = instance
        .public_ip_address
        .as_ref()
        .unwrap_or_else(|| instance.public_dns_name.as_ref().unwrap());

    InstanceDescription {
        name: name.to_string(),
        public_address: if public_dns != "" {
            Some(public_dns.to_string())
        } else {
            None
        },
        image_id: instance.image_id.clone(),
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

fn get_user_name(ec2_client: Ec2Client, description: &InstanceDescription) -> String {
    let image_request = DescribeImagesRequest {
        filters: Some(vec![Filter {
            name: Some("image-id".to_string()),
            values: (&description.image_id)
                .as_ref()
                .map(|image_id| vec![image_id.to_string()]),
        }]),
        ..DescribeImagesRequest::default()
    };
    ec2_client
        .describe_images(image_request)
        .sync()
        .map(|response| extract_user_name(&response.images.unwrap()[0]))
        .unwrap_or("ec2-user".to_string())
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
