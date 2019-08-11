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
                        let name =
                            extract_name(instance).map_or("<unnamed>", |name| name);
                        println!("{:?}", name)
                    })
            }
        }
        Err(error) => {
            println!("error: {:?}", error);
        }
    }
}

fn extract_name(instance: &Instance) -> Option<&String> {
    match &instance.tags {
        Some(tags) => tags
            .iter()
            .find(|tag| tag.key == Some("Name".to_string()))
            .and_then(|tag| (tag.value).as_ref()),
        None => None,
    }
}
