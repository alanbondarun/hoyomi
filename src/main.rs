use rusoto_core::Region;
use rusoto_ec2::{Ec2, Ec2Client, DescribeInstancesRequest};

fn main() {
    let ec2_client = Ec2Client::new(Region::ApNortheast2);
    let request = DescribeInstancesRequest::default();

    match ec2_client.describe_instances(request).sync() {
        Ok(response) => {
            println!("ok: {:?}", response);
        }
        Err(error) => {
            println!("error: {:?}", error);
        }
    }
}
