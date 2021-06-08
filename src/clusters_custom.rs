use logo_lib::Segment;
use logo_lib::Cluster;


pub fn build_cluster<'a>(yellow_segment: &'a Segment, blue_segments: &'a Vec<Segment>, red_segments: &'a Vec<Segment>) -> Option<Cluster<'a>> {
    let mut matched_blue_segments = Vec::new();
    for bs in blue_segments.iter() {
        if yellow_segment.contains(bs) {
            matched_blue_segments.push(bs);
        }
    }
    let mut matched_red_segments = Vec::new();
    for rs in red_segments.iter() {
        if yellow_segment.contains(rs) {
            matched_red_segments.push(rs);
        }
    }
    // let res = draw_bounding_boxes(&image, &matched_blue_segments).unwrap();
    // imshow("test", &res);
    if logo_rules(yellow_segment, &matched_blue_segments, &matched_red_segments) {
        return Some(Cluster::new(
            yellow_segment.get_row_min(),
            yellow_segment.get_row_max(),
            yellow_segment.get_col_min(),
            yellow_segment.get_col_max(),
            yellow_segment,
            matched_blue_segments,
            matched_red_segments,
        ));
    }
    None
}

fn logo_rules(_yellow_segment: &Segment, blue_segments: &Vec<&Segment>, red_segments: &Vec<&Segment>) -> bool {
    (blue_segments.len() == 2 || blue_segments.len() == 3) &&
        (red_segments.len() == 1 || red_segments.len() == 2)
}

pub fn build_clusters<'a>(yellow_segments: &'a Vec<Segment>, blue_segments: &'a Vec<Segment>, red_segments: &'a Vec<Segment>) -> Vec<Cluster<'a>> {
    let mut clusters = Vec::new();
    for ys in yellow_segments.iter() {
        if let Some(cluster) = build_cluster(ys, &blue_segments, &red_segments) {
            clusters.push(cluster);
        }
    }
    clusters
}