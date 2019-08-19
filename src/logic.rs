use rusoto_core::Region;

pub fn most_similar_region(region_input: &str) -> Option<Region> {
    let regions = [
        Region::ApEast1,
        Region::ApNortheast1,
        Region::ApNortheast2,
        Region::ApSouth1,
        Region::ApSoutheast1,
        Region::ApSoutheast2,
        Region::CaCentral1,
        Region::EuCentral1,
        Region::EuWest1,
        Region::EuWest2,
        Region::EuWest3,
        Region::EuNorth1,
        Region::SaEast1,
        Region::UsEast1,
        Region::UsEast2,
        Region::UsWest1,
        Region::UsWest2,
        Region::UsGovEast1,
        Region::UsGovWest1,
        Region::CnNorth1,
        Region::CnNorthwest1,
    ];
    regions
        .iter()
        .map(|r| (r, matched_positions(region_input, r.name())))
        .filter(|positions| positions.1.is_some())
        .map(|positions| (positions.0, positions.1.unwrap()))
        .min_by_key(|item| item.1.clone())
        .map(|item| item.0.to_owned())
}

fn matched_positions(query: &str, haystack: &str) -> Option<Vec<usize>> {
    let mut matched_positions = vec![];
    let mut matched_index = 0;
    let query_chars = query.chars().collect::<Vec<char>>();

    for (index, character) in haystack.chars().enumerate() {
        if matched_index >= query_chars.len() {
            return None;
        }
        if character == query_chars[matched_index] {
            matched_positions.push(index);
            matched_index += 1;
        }
    }

    if matched_index == query_chars.len() {
        Some(matched_positions)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_most_similar_region() {
        assert_eq!(
            Some(Region::ApNortheast1),
            most_similar_region("ap-northeast-1"),
        );
        assert_eq!(Some(Region::ApNortheast1), most_similar_region("an1"));
        assert_eq!(Some(Region::ApEast1), most_similar_region("ae1"));
        assert_eq!(None, most_similar_region("xyz"));
    }

    #[test]
    fn test_matched_positions() {
        assert_eq!(None, matched_positions("abc", ""));
        assert_eq!(None, matched_positions("abc", "dddd"));
        assert_eq!(None, matched_positions("abc", "abdd"));
        assert_eq!(Some(vec![0, 1, 3]), matched_positions("abc", "abdc"));
    }
}
