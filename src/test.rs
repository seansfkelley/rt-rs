// pub const ORIGIN: Vec3 = Vec3 { x: 0f64, y: 0f64, z: 0f64 };

// const TEST_VECTOR: Vec3 = Vec3 { x: 1f64, y: 2f64, z: 3f64 };
// const TEST_POINT: Point = Point { x: 1f64, y: 2f64, z: 3f64 };
// const TEST_NORMAL: Normal = Normal { x: 1f64, y: 2f64, z: 3f64 };

// const YELLOW_MATTE: material::FlatMaterial = material::FlatMaterial { color: Color { r: 0.7f64, g: 0.7f64, b: 0f64 }, specular_exponent: 0f64, reflectivity: 0f64 };
// const STRAIGHT_RAY: Ray = Ray { origin: Point { x: 0f64, y: 0f64, z: 5f64 }, direction: Vec3 { x: 0f64, y: 0f64, z: -1f64 } };
// const OFFSET_RAY: Ray = Ray { origin: Point { x: 5f64, y: 0f64, z: 5f64 }, direction: Vec3 { x: 0f64, y: 0f64, z: -1f64 } };

// describe! mat4 {

//     // describe! rotation {
//     //     it "should rotate around x-axis" {
//     //         // #floats
//     //         let expected = Vec3 { x: 1f64, y: -2.0000000000000004f64, z: -2.9999999999999996f64 };
//     //         assert_close_to!(Mat4::create_rotation(PI, X_AXIS) * TEST_VECTOR, expected);
//     //     }

//     //     it "should rotate around x-axis at different angles" {
//     //         let expected = Vec3 { x: 4f64, y: -6.392304845413263f64, z: 12.92820323027551f64 };
//     //         assert_eq!(Mat4::create_rotation(PI / 3f64, X_AXIS) * (TEST_VECTOR * 4f64), expected);
//     //     }

//     //     it "should rotate around y-axis" {
//     //         let transform = Mat4::create_rotation(PI, Y_AXIS);
//     //         let start = Vec3 { x: 1f64, y: -2f64, z: -3f64 };
//     //         let expected = Vec3 { x: -1.0000000000000004f64, y: -2f64, z: 3f64 };
//     //         assert_eq!(transform * start, expected);
//     //     }

//     //     it "should compose rotations" {
//     //         let transform = Mat4::create_rotation(PI, X_AXIS).rotate(PI, Y_AXIS);
//     //         let expected = Vec3 { x: -0.9999999999999997f64, y: -1.9999999999999996f64, z: 3.0000000000000004f64 };
//     //         assert_eq!(transform * TEST_VECTOR, expected);
//     //     }

//     //     it "should be reversed by inverse" {
//     //         let transform = Mat4::create_rotation(PI, X_AXIS).rotate(PI, Y_AXIS);
//     //         let inverse = transform.invert().unwrap();
//     //         assert_eq!(inverse * (transform * TEST_VECTOR), TEST_VECTOR);
//     //     }
//     // }

//     // describe! translation {
//     //     it "should translate" {
//     //         let expected = Vec3 { x: 2f64, y: 2f64, z: 3f64 };
//     //         assert_eq!(Mat4::create_translation(X_AXIS) * TEST_VECTOR, expected);
//     //     }

//     //     it "should compose translations" {
//     //         let expected = Vec3 { x: 3f64, y: 4f64, z: 6f64 };
//     //         let transform = Mat4::create_translation(X_AXIS)
//     //             .translate(TEST_VECTOR);
//     //         assert_eq!(transform * TEST_VECTOR, expected);
//     //     }
//     // }

//     // describe! scale {
//     //     it "should scale" {
//     //         let expected = Vec3 { x: 1f64, y: 0f64, z: 0f64 };
//     //         assert_eq!(Mat4::create_scale(X_AXIS) * TEST_VECTOR, expected);
//     //     }

//     //     it "should compose scales" {
//     //         let expected = Vec3 { x: 2f64, y: 0.5f64, z: 0f64 };
//     //         let transform = Mat4::create_scale(Vec3 { x: 2f64, y: 0.5f64, z: 1f64 })
//     //             .scale(Vec3 { x: 1f64, y: 0.5f64, z: 0f64 });
//     //         assert_eq!(transform * TEST_VECTOR, expected);
//     //     }
//     // }

//     // describe! composition {
//     //     it "should compose" {
//     //         let expected = Vec3 { x: 10f64, y: -6.000000000000002f64, z: -6.999999999999999f64 };
//     //         let transform = Mat4::create_scale(Vec3 { x: 2f64, y: 1f64, z: 1f64 })
//     //             .rotate(PI, X_AXIS)
//     //             .translate(Vec3 { x: 4f64, y: 4f64, z: 4f64 });
//     //         assert_eq!(transform * TEST_VECTOR, expected);
//     //     }
//     // }
// }

// describe! transform {
//     describe! vec3_transformation {
//         it "should not change the vector" {
//             let transform = Transform::new(IDENTITY_MATRIX);
//             let expected = Vec3::new(1f64, 2f64, 3f64);
//             let actual = expected.transform(&transform);
//             assert_eq!(actual, expected);
//         }

//         it "should scale the vector" {
//             panic!("1");
//             let v = Vec3::new(2f64, 3f64, 4f64);
//             panic!("2");
//             let matrix = Mat4::create_scale(v);
//             panic!("3");
//             let transform = Transform::new(matrix);
//             panic!("4");
//             let expected = Vec3::new(2f64, 6f64, 12f64);
//             panic!("5");
//             let actual = expected.transform(&transform);
//             panic!("6");
//             assert_eq!(actual, expected);
//             panic!("7");
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
