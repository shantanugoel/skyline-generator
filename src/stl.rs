use crate::model::Contribution;

use std::fs::File;
use std::io::Result;
use stl_io::{Normal, Triangle, Vector};

pub fn create_3d_model(user: &str, year: u32, contributions: Vec<Contribution>) -> Result<()> {
    let mut triangles = Vec::new();

    // Dimensions
    let base_width = 142.5;
    let base_depth = 27.5;
    let max_height = 25.0;
    let cell_width = 2.5;
    let cell_depth = 2.5;

    // Find the maximum count to scale the heights
    let max_count = contributions.iter().map(|c| c.count).max().unwrap_or(1) as f32;

    for contribution in contributions {
        let week = contribution.week as f32;
        let day = contribution.day as f32 - 1.0; // Adjust day to be 0-indexed
        let height = (contribution.count as f32 / max_count) * max_height;

        // Base vertices
        let base_x = 5. + week * cell_width;
        let base_y = 2.5 + (6.0 - day) * cell_depth;
        let base_z = 0.0;

        let v0 = Vector::new([base_x, base_y, base_z]);
        let v1 = Vector::new([base_x + cell_width, base_y, base_z]);
        let v2 = Vector::new([base_x + cell_width, base_y + cell_depth, base_z]);
        let v3 = Vector::new([base_x, base_y + cell_depth, base_z]);

        // Top vertices
        let v4 = Vector::new([base_x, base_y, height]);
        let v5 = Vector::new([base_x + cell_width, base_y, height]);
        let v6 = Vector::new([base_x + cell_width, base_y + cell_depth, height]);
        let v7 = Vector::new([base_x, base_y + cell_depth, height]);

        // Define triangles for each face of the cube
        let face_triangles = vec![
            // Bottom face
            Triangle {
                normal: Normal::new([0.0, 0.0, -1.0]),
                vertices: [v0, v1, v2],
            },
            Triangle {
                normal: Normal::new([0.0, 0.0, -1.0]),
                vertices: [v0, v2, v3],
            },
            // Top face
            Triangle {
                normal: Normal::new([0.0, 0.0, 1.0]),
                vertices: [v4, v5, v6],
            },
            Triangle {
                normal: Normal::new([0.0, 0.0, 1.0]),
                vertices: [v4, v6, v7],
            },
            // Front face
            Triangle {
                normal: Normal::new([0.0, 1.0, 0.0]),
                vertices: [v3, v2, v6],
            },
            Triangle {
                normal: Normal::new([0.0, 1.0, 0.0]),
                vertices: [v3, v6, v7],
            },
            // Back face
            Triangle {
                normal: Normal::new([0.0, -1.0, 0.0]),
                vertices: [v0, v1, v5],
            },
            Triangle {
                normal: Normal::new([0.0, -1.0, 0.0]),
                vertices: [v0, v5, v4],
            },
            // Left face
            Triangle {
                normal: Normal::new([-1.0, 0.0, 0.0]),
                vertices: [v0, v3, v7],
            },
            Triangle {
                normal: Normal::new([-1.0, 0.0, 0.0]),
                vertices: [v0, v7, v4],
            },
            // Right face
            Triangle {
                normal: Normal::new([1.0, 0.0, 0.0]),
                vertices: [v1, v2, v6],
            },
            Triangle {
                normal: Normal::new([1.0, 0.0, 0.0]),
                vertices: [v1, v6, v5],
            },
        ];

        triangles.extend(face_triangles);
    }

    // Add the base plate
    let base_vertices = [
        Vector::new([0.0, 0.0, 0.0]),
        Vector::new([base_width, 0.0, 0.0]),
        Vector::new([base_width, base_depth, 0.0]),
        Vector::new([0.0, base_depth, 0.0]),
        Vector::new([0.0, 0.0, -0.1]),
        Vector::new([base_width, 0.0, -0.1]),
        Vector::new([base_width, base_depth, -0.1]),
        Vector::new([0.0, base_depth, -0.1]),
    ];

    let base_triangles = vec![
        // Top face
        Triangle {
            normal: Normal::new([0.0, 0.0, 1.0]),
            vertices: [base_vertices[0], base_vertices[1], base_vertices[2]],
        },
        Triangle {
            normal: Normal::new([0.0, 0.0, 1.0]),
            vertices: [base_vertices[0], base_vertices[2], base_vertices[3]],
        },
        // Bottom face
        Triangle {
            normal: Normal::new([0.0, 0.0, -1.0]),
            vertices: [base_vertices[4], base_vertices[5], base_vertices[6]],
        },
        Triangle {
            normal: Normal::new([0.0, 0.0, -1.0]),
            vertices: [base_vertices[4], base_vertices[6], base_vertices[7]],
        },
        // Front face
        Triangle {
            normal: Normal::new([0.0, 1.0, 0.0]),
            vertices: [base_vertices[3], base_vertices[2], base_vertices[6]],
        },
        Triangle {
            normal: Normal::new([0.0, 1.0, 0.0]),
            vertices: [base_vertices[3], base_vertices[6], base_vertices[7]],
        },
        // Back face
        Triangle {
            normal: Normal::new([0.0, -1.0, 0.0]),
            vertices: [base_vertices[0], base_vertices[1], base_vertices[5]],
        },
        Triangle {
            normal: Normal::new([0.0, -1.0, 0.0]),
            vertices: [base_vertices[0], base_vertices[5], base_vertices[4]],
        },
        // Left face
        Triangle {
            normal: Normal::new([-1.0, 0.0, 0.0]),
            vertices: [base_vertices[0], base_vertices[3], base_vertices[7]],
        },
        Triangle {
            normal: Normal::new([-1.0, 0.0, 0.0]),
            vertices: [base_vertices[0], base_vertices[7], base_vertices[4]],
        },
        // Right face
        Triangle {
            normal: Normal::new([1.0, 0.0, 0.0]),
            vertices: [base_vertices[1], base_vertices[2], base_vertices[6]],
        },
        Triangle {
            normal: Normal::new([1.0, 0.0, 0.0]),
            vertices: [base_vertices[1], base_vertices[6], base_vertices[5]],
        },
    ];

    triangles.extend(base_triangles);

    // Add the trapezoidal base below the plate
    let trapezoid_height = 10.0;
    let trapezoid_bottom_top_diff = 2.5;
    let trapezoid_vertices = [
        Vector::new([0., 0., -0.1]),
        Vector::new([base_width, 0., -0.1]),
        Vector::new([base_width, base_depth, -0.1]),
        Vector::new([0., base_depth, -0.1]),
        Vector::new([
            -trapezoid_bottom_top_diff,
            -trapezoid_bottom_top_diff,
            -(0.1 + trapezoid_height),
        ]),
        Vector::new([
            base_width + trapezoid_bottom_top_diff,
            -trapezoid_bottom_top_diff,
            -(0.1 + trapezoid_height),
        ]),
        Vector::new([
            base_width + trapezoid_bottom_top_diff,
            base_depth + trapezoid_bottom_top_diff,
            -(0.1 + trapezoid_height),
        ]),
        Vector::new([
            -trapezoid_bottom_top_diff,
            base_depth + trapezoid_bottom_top_diff,
            -(0.1 + trapezoid_height),
        ]),
    ];

    let trapezoid_triangles = vec![
        // Top face
        Triangle {
            normal: Normal::new([0.0, 0.0, 1.0]),
            vertices: [
                trapezoid_vertices[0],
                trapezoid_vertices[1],
                trapezoid_vertices[2],
            ],
        },
        Triangle {
            normal: Normal::new([0.0, 0.0, 1.0]),
            vertices: [
                trapezoid_vertices[0],
                trapezoid_vertices[2],
                trapezoid_vertices[3],
            ],
        },
        // Bottom face
        Triangle {
            normal: Normal::new([0.0, 0.0, -1.0]),
            vertices: [
                trapezoid_vertices[4],
                trapezoid_vertices[5],
                trapezoid_vertices[6],
            ],
        },
        Triangle {
            normal: Normal::new([0.0, 0.0, -1.0]),
            vertices: [
                trapezoid_vertices[4],
                trapezoid_vertices[6],
                trapezoid_vertices[7],
            ],
        },
        // Front face
        Triangle {
            normal: Normal::new([0.0, 1.0, 0.0]),
            vertices: [
                trapezoid_vertices[3],
                trapezoid_vertices[2],
                trapezoid_vertices[6],
            ],
        },
        Triangle {
            normal: Normal::new([0.0, 1.0, 0.0]),
            vertices: [
                trapezoid_vertices[3],
                trapezoid_vertices[6],
                trapezoid_vertices[7],
            ],
        },
        // Back face
        Triangle {
            normal: Normal::new([0.0, -1.0, 0.0]),
            vertices: [
                trapezoid_vertices[0],
                trapezoid_vertices[1],
                trapezoid_vertices[5],
            ],
        },
        Triangle {
            normal: Normal::new([0.0, -1.0, 0.0]),
            vertices: [
                trapezoid_vertices[0],
                trapezoid_vertices[5],
                trapezoid_vertices[4],
            ],
        },
        // Left face
        Triangle {
            normal: Normal::new([-1.0, 0.0, 0.0]),
            vertices: [
                trapezoid_vertices[0],
                trapezoid_vertices[3],
                trapezoid_vertices[7],
            ],
        },
        Triangle {
            normal: Normal::new([-1.0, 0.0, 0.0]),
            vertices: [
                trapezoid_vertices[0],
                trapezoid_vertices[7],
                trapezoid_vertices[4],
            ],
        },
        // Right face
        Triangle {
            normal: Normal::new([1.0, 0.0, 0.0]),
            vertices: [
                trapezoid_vertices[1],
                trapezoid_vertices[2],
                trapezoid_vertices[6],
            ],
        },
        Triangle {
            normal: Normal::new([1.0, 0.0, 0.0]),
            vertices: [
                trapezoid_vertices[1],
                trapezoid_vertices[6],
                trapezoid_vertices[5],
            ],
        },
    ];

    triangles.extend(trapezoid_triangles);

    // Write to an STL file
    let mut file = File::create(format!("{user}_{year}.stl"))?;
    stl_io::write_stl(&mut file, triangles.iter().cloned())?;

    Ok(())
}
