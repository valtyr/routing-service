pub fn dist_haversine(a: &[f32], b: &[f32]) -> f32 {
    let lat1 = a[0].to_radians();
    let lon1 = a[1].to_radians();

    let lat2 = b[0].to_radians();
    let lon2 = b[1].to_radians();

    let dlathalf = (lat2 - lat1) * 0.5;
    let dlonhalf = (lon2 - lon1) * 0.5;

    let sqrth =
        (dlathalf.sin().powi(2) + (lat1.cos() * lat2.cos() * dlonhalf.sin().powi(2))).sqrt();

    return sqrth.asin() * 2.0 * 6371.0;
}
