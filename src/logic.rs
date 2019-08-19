use rusoto_core::Region;
use std::str::FromStr;

pub fn most_similar_region(region: &str) -> Option<Region> {
    Region::from_str(region).ok()
}

fn matched_positions(query: &str, haystack: &str) -> Option<Vec<usize>> {
    unimplemented!()
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
    }

    #[test]
    fn test_matched_positions() {
        assert_eq!(None, matched_positions("abc", ""));
        assert_eq!(None, matched_positions("abc", "dddd"));
        assert_eq!(None, matched_positions("abc", "abdd"));
        assert_eq!(vec![0, 1, 3], matched_positions("abc", "abdc"));
    }
}
