use rusoto_core::Region;
use rusoto_ec2::{
    DescribeImagesRequest, DescribeInstancesRequest, Ec2, Ec2Client, Filter, Image,
    Instance,
};
use std::collections::HashMap;

pub struct InstanceDescription {
    pub name: String,
    pub public_address: Option<String>,
    pub image_id: Option<String>,
}

pub struct Client {
    ec2_client: Ec2Client,
}

impl Client {
    pub fn new(region: &Region) -> Self {
        Client {
            ec2_client: Ec2Client::new(region.clone()),
        }
    }

    pub fn get_descriptions_by_name(&self) -> HashMap<String, InstanceDescription> {
        let ec2_client = &self.ec2_client;
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
                            let description = Self::extract_description(&instance);
                            descriptions_by_name
                                .insert(description.name.clone(), description);
                        })
                }
            }
            Err(error) => {
                println!("error: {:?}", error);
            }
        }

        descriptions_by_name
    }

    fn extract_description(instance: &Instance) -> InstanceDescription {
        let name = Self::extract_name(&instance).unwrap_or("<unnamed>");
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

    pub fn get_user_name(&self, description: &InstanceDescription) -> String {
        let image_request = DescribeImagesRequest {
            filters: Some(vec![Filter {
                name: Some("image-id".to_string()),
                values: (&description.image_id)
                    .as_ref()
                    .map(|image_id| vec![image_id.to_string()]),
            }]),
            ..DescribeImagesRequest::default()
        };
        self.ec2_client
            .describe_images(image_request)
            .sync()
            .map(|response| {
                Self::extract_user_name(
                    response.images.as_ref().and_then(|images| images.first()),
                )
            })
            .unwrap_or_else(|_| "ec2-user".to_string())
    }

    fn extract_user_name(image: Option<&Image>) -> String {
        image
            .and_then(|img| img.name.as_ref())
            .map(|image_name| {
                if image_name.starts_with("ubuntu") {
                    "ubuntu".to_string()
                } else {
                    "ec2-user".to_string()
                }
            })
            .unwrap_or_else(|| "ec2-user".to_string())
    }
}
