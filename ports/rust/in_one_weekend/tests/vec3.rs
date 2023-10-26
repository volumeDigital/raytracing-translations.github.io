use in_one_weekend::vec3::Vec3;

#[test]
fn default() {
    let v = Vec3::default();
    assert_eq!(v.x(), 0.0);
    assert_eq!(v.y(), 0.0);
    assert_eq!(v.z(), 0.0);
}

#[test]
fn new_and_getters() {
    let v = Vec3::new(-1.0, -2.0, -3.0);
    assert_eq!(v.x(), -1.0);
    assert_eq!(v.y(), -2.0);
    assert_eq!(v.z(), -3.0);
}

#[test]
fn equal() {
    let v1 = Vec3::new(-1.0, -2.0, -3.0);
    let v2 = Vec3::new(-1.0, -2.0, -3.0);
    assert_eq!(v1, v2);
}

#[test]
fn not_equal() {
    let v1 = Vec3::new(-1.0, -2.0, -3.0);
    assert_ne!(v1, Vec3::new(1.0, -2.0, -3.0));
    assert_ne!(v1, Vec3::new(-1.0, 2.0, -3.0));
    assert_ne!(v1, Vec3::new(-1.0, -2.0, 3.0));
}

#[test]
fn length() {
    assert_eq!(Vec3::new(1.0, 2.0, 2.0).length(), 3.0);
    assert_eq!(Vec3::new(-2.0, 3.0, 6.0).length(), 7.0);
    assert_eq!(Vec3::new(1.0, -4.0, 8.0).length(), 9.0);
    assert_eq!(Vec3::new(4.0, 4.0, -7.0).length(), 9.0);
}

#[test]
fn length_squared() {
    assert_eq!(Vec3::new(1.0, 2.0, 2.0).length_squared(), 9.0);
    assert_eq!(Vec3::new(-2.0, 3.0, 6.0).length_squared(), 49.0);
    assert_eq!(Vec3::new(1.0, -4.0, 8.0).length_squared(), 81.0);
    assert_eq!(Vec3::new(4.0, 4.0, -7.0).length_squared(), 81.0);
}

#[test]
fn dot() {
    assert_eq!(
        Vec3::dot(Vec3::new(1.0, 2.0, 3.0), Vec3::new(6.0, 5.0, 4.0)),
        28.0
    );

    assert_eq!(
        Vec3::dot(Vec3::new(1.0, -2.0, 3.0), Vec3::new(6.0, 5.0, 4.0)),
        8.0
    );
}

#[test]
fn cross() {
    assert_eq!(
        Vec3::cross(Vec3::new(1.0, 0.0, 0.0), Vec3::new(1.0, 0.0, 0.0)),
        Vec3::new(0.0, 0.0, 0.0)
    );
    assert_eq!(
        Vec3::cross(Vec3::new(1.0, 0.0, 0.0), Vec3::new(0.0, 1.0, 0.0)),
        Vec3::new(0.0, 0.0, 1.0)
    );
    assert_eq!(
        Vec3::cross(Vec3::new(1.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 1.0)),
        Vec3::new(0.0, -1.0, 0.0)
    );
}

#[test]
fn unit_vector() {
    assert_eq!(
        Vec3::unit_vector(Vec3::new(2.0, 0.0, 0.0)),
        Vec3::new(1.0, 0.0, 0.0)
    );
}

#[test]
fn neg() {
    assert_eq!(Vec3::new(1.0, -2.0, 3.0), -Vec3::new(-1.0, 2.0, -3.0));
}

#[test]
fn index() {
    let v = Vec3::new(0.0, 1.0, 2.0);
    assert_eq!(v[0], 0.0);
    assert_eq!(v[1], 1.0);
    assert_eq!(v[2], 2.0);
}

#[test]
fn index_mut() {
    let mut v = Vec3::default();

    assert_eq!(v[0], 0.0);
    assert_eq!(v[1], 0.0);
    assert_eq!(v[2], 0.0);

    v[0] = 4.0;
    v[1] = 5.0;
    v[2] = 6.0;

    assert_eq!(v[0], 4.0);
    assert_eq!(v[1], 5.0);
    assert_eq!(v[2], 6.0);
}

#[test]
fn add_assign_vec3() {
    let mut v = Vec3::new(1.0, -2.0, 0.0);
    v += Vec3::new(1.0, 2.0, 3.0);
    assert_eq!(v, Vec3::new(2.0, 0.0, 3.0))
}
#[test]
fn mul_assign_f64() {
    let mut v = Vec3::new(1.0, -2.0, -1.0);
    v *= 2.0;
    assert_eq!(v, Vec3::new(2.0, -4.0, -2.0))
}
#[test]
fn div_assign_f64() {
    let mut v = Vec3::new(1.0, -2.0, -1.0);
    v /= 2.0;
    assert_eq!(v, Vec3::new(0.5, -1.0, -0.5))
}

#[test]
fn div_assign_vec3() {
    let mut v = Vec3::new(12.0, -12.0, 12.0);
    v /= Vec3::new(1.0, 2.0, -3.0);
    assert_eq!(v, Vec3::new(12.0, -6.0, -4.0))
}

#[test]
fn add() {
    assert_eq!(
        Vec3::new(1.0, 1.0, 1.0) + Vec3::new(-1.0, 2.0, 3.0),
        Vec3::new(0.0, 3.0, 4.0)
    );
}

#[test]
fn sub() {
    assert_eq!(
        Vec3::new(1.0, 1.0, 1.0) - Vec3::new(-1.0, 2.0, 3.0),
        Vec3::new(2.0, -1.0, -2.0)
    )
}

#[test]
fn mul_by_vec3() {
    assert_eq!(
        Vec3::new(1.0, 1.0, 1.0) * Vec3::new(-1.0, 2.0, 3.0),
        Vec3::new(-1.0, 2.0, 3.0)
    );
}

#[test]
fn mul_by_f64_commutative() {
    assert_eq!(Vec3::new(1.0, 1.0, 2.0) * 2.0, Vec3::new(2.0, 2.0, 4.0));

    assert_eq!(2.0 * Vec3::new(1.0, 1.0, 2.0), Vec3::new(2.0, 2.0, 4.0));
}

#[test]
fn div_by_f64() {
    assert_eq!(Vec3::new(-4.0, 6.0, 8.0) / 2.0, Vec3::new(-2.0, 3.0, 4.0));
}
