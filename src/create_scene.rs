use crate::material::dielectric::Dielectric;
use crate::material::lambertian::Lambertian;
use crate::material::metal::Metal;
use crate::material::scatterable::Material;
use crate::shape::hittable::Shape;
use crate::shape::sphere::Sphere;
use crate::vec3::Vec3;
use crate::{camera::Camera, shape::triangle_mesh::TriangleMesh};
use obj::Obj;
use rand::{Rng, SeedableRng};
use rand_pcg::Pcg64Mcg;

#[allow(dead_code)]
pub fn create_5_sphere_scene<'a>(width: f64, height: f64) -> (Camera, Vec<Shape<'a, 'a>>) {
    let look_from = Vec3::new(3.0, 3.0, 2.0);
    let look_at = Vec3::new(0.0, 0.0, -1.0);
    let up = Vec3::new(0.0, 1.0, 0.0);
    let focus_distance = (look_from - look_at).length();
    let aperture = 0.0;
    let camera = Camera::new(
        look_from,
        look_at,
        up,
        20.0,
        width / height,
        aperture,
        focus_distance,
    );
    let scene: Vec<Shape> = vec![
        Shape::Sph(Sphere::new(
            Vec3::new(0.0, 0.0, -1.0),
            0.5,
            Material::Lambertian(Lambertian {
                albedo: Vec3::new(0.1, 0.2, 0.5),
            }),
        )),
        Shape::Sph(Sphere::new(
            Vec3::new(0.0, -100.5, -1.0),
            100.0,
            Material::Lambertian(Lambertian {
                albedo: Vec3::new(0.8, 0.8, 0.0),
            }),
        )),
        Shape::Sph(Sphere::new(
            Vec3::new(1.0, 0.0, -1.0),
            0.5,
            Material::Metal(Metal {
                albedo: Vec3::new(0.8, 0.6, 0.2),
                roughness: 0.2,
            }),
        )),
        Shape::Sph(Sphere::new(
            Vec3::new(-1.0, 0.0, -1.0),
            0.5,
            Material::Dielectric(Dielectric {
                index_of_refraction: 1.5,
            }),
        )),
        Shape::Sph(Sphere::new(
            Vec3::new(-1.0, 0.0, -1.0),
            -0.45,
            Material::Dielectric(Dielectric {
                index_of_refraction: 1.5,
            }),
        )),
    ];

    (camera, scene)
}

pub fn create_suzanne_scene<'a>(
    width: f64,
    height: f64,
) -> (Camera, TriangleMesh, Vec<Shape<'a, 'a>>) {
    let look_from = Vec3::new(5.0, 0.5, 9.0);
    let look_at = Vec3::new(0.0, 0.0, 0.0);
    let up = Vec3::new(0.0, 1.0, 0.0);
    let focus_distance = (look_from - look_at).length();
    let aperture = 0.0;
    let camera = Camera::new(
        look_from,
        look_at,
        up,
        20.0,
        width / height,
        aperture,
        focus_distance,
    );
    let mesh = get_suzanne();

    let mut objects: Vec<Shape> = vec![];
    let mut rng = Pcg64Mcg::seed_from_u64(10);
    let origin = Vec3::origin();

    objects.push(Shape::Sph(Sphere::new(
        Vec3::new(0.0, -1001.0, 0.0),
        1000.0,
        Material::Lambertian(Lambertian {
            albedo: Vec3::new(0.5, 0.5, 0.5),
        }),
    )));
    for a in -11..11 {
        for b in -11..11 {
            if a == 0 && b == 0 {
                continue;
            }
            let choose_material = rng.gen::<f64>();
            let center = Vec3::new(
                a as f64 + 0.9 * rng.gen::<f64>(),
                -1.0 + 0.2,
                b as f64 + 0.9 * rng.gen::<f64>(),
            );
            if (center - origin).length() > 0.9 {
                if choose_material < 0.7 {
                    objects.push(Shape::Sph(Sphere::new(
                        center,
                        0.2,
                        Material::Lambertian(Lambertian {
                            albedo: Vec3::new(
                                rng.gen::<f64>() * rng.gen::<f64>(),
                                rng.gen::<f64>() * rng.gen::<f64>(),
                                rng.gen::<f64>() * rng.gen::<f64>(),
                            ),
                        }),
                    )));
                } else if choose_material < 0.85 {
                    objects.push(Shape::Sph(Sphere::new(
                        center,
                        0.2,
                        Material::Metal(Metal {
                            albedo: Vec3::new(
                                0.5 * (1.0 + rng.gen::<f64>()),
                                0.5 * (1.0 + rng.gen::<f64>()),
                                0.5 * (1.0 + rng.gen::<f64>()),
                            ),
                            roughness: 0.5 * rng.gen::<f64>(),
                        }),
                    )));
                } else {
                    objects.push(Shape::Sph(Sphere::new(
                        center,
                        0.2,
                        Material::Dielectric(Dielectric {
                            index_of_refraction: 1.5,
                        }),
                    )));
                }
            }
        }
    }

    (camera, mesh, objects)
}

fn get_suzanne() -> TriangleMesh {
    let obj = Obj::load("./models/suzanne.obj").unwrap();

    TriangleMesh {
        vertices: obj
            .data
            .position
            .iter()
            .map(|v| Vec3::new(v[0] as f64, v[1] as f64, v[2] as f64))
            .collect(),
        vertex_indices: obj
            .data
            .objects
            .iter()
            .flat_map(|obj| {
                obj.groups.iter().flat_map(|g| {
                    g.polys
                        .iter()
                        .map(|poly| [poly.0[0].0, poly.0[1].0, poly.0[2].0])
                })
            })
            .collect(),
        material: Material::Lambertian(Lambertian {
            albedo: Vec3::new(0.6, 0.6, 0.6),
        }),
    }
}

pub fn create_book_1_final_scene<'a>(width: f64, height: f64) -> (Camera, Vec<Shape<'a, 'a>>) {
    let look_from = Vec3::new(13.0, 2.0, 3.0);
    let look_at = Vec3::new(0.0, 0.0, 0.0);
    let up = Vec3::new(0.0, 1.0, 0.0);
    let focus_distance = 10.0;
    let aperture = 0.1;
    let camera = Camera::new(
        look_from,
        look_at,
        up,
        20.0,
        width / height,
        aperture,
        focus_distance,
    );

    let mut objects: Vec<Shape> = vec![];
    let mut rng = Pcg64Mcg::seed_from_u64(10);
    let origin = Vec3::origin();

    objects.push(Shape::Sph(Sphere::new(
        Vec3::new(0.0, -1000.0, 0.0),
        1000.0,
        Material::Lambertian(Lambertian {
            albedo: Vec3::new(0.5, 0.5, 0.5),
        }),
    )));
    for a in -11..11 {
        for b in -11..11 {
            let choose_material = rng.gen::<f64>();
            let center = Vec3::new(
                a as f64 + 0.9 * rng.gen::<f64>(),
                0.2,
                b as f64 + 0.9 * rng.gen::<f64>(),
            );
            if (center - origin).length() > 0.9 {
                if choose_material < 0.8 {
                    objects.push(Shape::Sph(Sphere::new(
                        center,
                        0.2,
                        Material::Lambertian(Lambertian {
                            albedo: Vec3::new(
                                rng.gen::<f64>() * rng.gen::<f64>(),
                                rng.gen::<f64>() * rng.gen::<f64>(),
                                rng.gen::<f64>() * rng.gen::<f64>(),
                            ),
                        }),
                    )));
                } else if choose_material < 0.95 {
                    objects.push(Shape::Sph(Sphere::new(
                        center,
                        0.2,
                        Material::Metal(Metal {
                            albedo: Vec3::new(
                                0.5 * (1.0 + rng.gen::<f64>()),
                                0.5 * (1.0 + rng.gen::<f64>()),
                                0.5 * (1.0 + rng.gen::<f64>()),
                            ),
                            roughness: 0.5 * rng.gen::<f64>(),
                        }),
                    )));
                } else {
                    objects.push(Shape::Sph(Sphere::new(
                        center,
                        0.2,
                        Material::Dielectric(Dielectric {
                            index_of_refraction: 1.5,
                        }),
                    )));
                }
            }
        }
    }
    objects.push(Shape::Sph(Sphere::new(
        Vec3::new(0.0, 1.0, 0.0),
        1.0,
        Material::Dielectric(Dielectric {
            index_of_refraction: 1.5,
        }),
    )));
    objects.push(Shape::Sph(Sphere::new(
        Vec3::new(-4.0, 1.0, 0.0),
        1.0,
        Material::Lambertian(Lambertian {
            albedo: Vec3::new(0.4, 0.2, 0.1),
        }),
    )));
    objects.push(Shape::Sph(Sphere::new(
        Vec3::new(4.0, 1.0, 0.0),
        1.0,
        Material::Metal(Metal {
            albedo: Vec3::new(0.7, 0.6, 0.5),
            roughness: 0.0,
        }),
    )));
    (camera, objects)
}
