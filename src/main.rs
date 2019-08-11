use rusoto_core::Region;
use rusoto_ec2::{DescribeInstancesRequest, Ec2, Ec2Client, Instance};

fn main() {
    let ec2_client = Ec2Client::new(Region::ApNortheast2);
    let request = DescribeInstancesRequest::default();

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
                            println!(
                                "{} ssh -i <ssh-key> ec2-user@{}",
                                name, public_dns
                            );
                        }
                    })
            }
        }
        Err(error) => {
            println!("error: {:?}", error);
        }
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
