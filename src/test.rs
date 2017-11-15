// pub const ORIGIN: Vec3 = Vec3 { x: 0f64, y: 0f64, z: 0f64 };

// const TEST_VECTOR: Vec3 = Vec3 { x: 1f64, y: 2f64, z: 3f64 };
// const TEST_POINT: Point = Point { x: 1f64, y: 2f64, z: 3f64 };
// const TEST_NORMAL: Normal = Normal { x: 1f64, y: 2f64, z: 3f64 };

// const YELLOW_MATTE: material::FlatMaterial = material::FlatMaterial { color: Color { r: 0.7f64, g: 0.7f64, b: 0f64 }, specular_exponent: 0f64, reflectivity: 0f64 };
// const STRAIGHT_RAY: Ray = Ray { origin: Point { x: 0f64, y: 0f64, z: 5f64 }, direction: Vec3 { x: 0f64, y: 0f64, z: -1f64 } };
// const OFFSET_RAY: Ray = Ray { origin: Point { x: 5f64, y: 0f64, z: 5f64 }, direction: Vec3 { x: 0f64, y: 0f64, z: -1f64 } };

// describe! mat4 {

// describe! transform {
//     describe! vec3_transformation {
//         it "should not change the vector" {
//             let transform = Transform::new(IDENTITY_MATRIX);
//             let expected = Vec3::new(1f64, 2f64, 3f64);
//             let actual = expected.transform(&transform);
//             assert_eq!(actual, expected);
//         }

//         it "should scale the vector" {
//             let v = Vec3::new(2f64, 3f64, 4f64);
//             let matrix = Mat4::create_scale(v);
//             let transform = Transform::new(matrix);
//             let expected = Vec3::new(2f64, 6f64, 12f64);
//             let actual = expected.transform(&transform);
//             assert_eq!(actual, expected);
//         }

//         // it "should rotate the vector" {

//         // }

//         // it "should not translate the vector" {

//         // }
//     }
// }

// // describe! ray {
// //     it "should transform" {
// //         let transform = Mat4::create_rotation(PI, X_AXIS);
// //         let ray = Ray::new(ORIGIN, Vec3::new(0f64, 0f64, -1f64));
// //         let transformed_ray = ray.transform(transform, transform.without_translation());
// //         assert_eq!(transformed_ray.origin, ORIGIN);
// //         assert_eq!(transformed_ray.direction, Vec3::new(0f64, 0.00000000000000012246467991473532f64, 1f64));
// //     }

// //     it "should transform and direction should ignore translation" {
// //         let transform = Mat4::create_rotation(PI, X_AXIS)
// //             .translate(X_AXIS);
// //         let ray = Ray::new(ORIGIN, Vec3::new(0f64, 0f64, -1f64));
// //         let transformed_ray = ray.transform(transform, transform.without_translation());
// //         assert_eq!(transformed_ray.origin, X_AXIS);
// //         assert_eq!(transformed_ray.direction, Vec3::new(0f64, 0.00000000000000012246467991473532f64, 1f64));
// //     }
// // }

// // describe! sphere {
// //     it "should simply intersect" {
// //         let yellow_matte: Rc<material::Material> = Rc::new(YELLOW_MATTE);
// //         let sphere = Sphere::new(1f64, IDENTITY_MATRIX, &yellow_matte);
// //         assert!(sphere.intersect(&STRAIGHT_RAY).is_some());
// //         assert!(sphere.intersect(&OFFSET_RAY).is_none());
// //     }

// //     it "should intersect translations" {
// //         let yellow_matte: Rc<material::Material> = Rc::new(YELLOW_MATTE);
// //         let sphere = Sphere::new(1f64, Mat4::create_translation(Vec3::new(5f64, 0f64, 0f64)), &yellow_matte);
// //         assert!(sphere.intersect(&STRAIGHT_RAY).is_none());
// //         assert!(sphere.intersect(&OFFSET_RAY).is_some());
// //     }
// // }
