use crate::model::Contribution;

use std::fs::File;
use std::io::Result;
use stl_io::{Normal, Triangle, Vector};

pub fn create_3d_model(contributions: Vec<Contribution>) -> Result<()> {
    let mut vertices = Vec::new();
    let mut triangles = Vec::new();

    // Define the size of each cell
    let cell_width = 1.0;
    let cell_depth = 1.0;

    for contribution in contributions {
        let week = contribution.week as f32;
        let day = contribution.day as f32 - 1.0; // Adjust day to be 0-indexed
        let height = contribution.count as f32;

        // Base vertices
        let base_x = week * cell_width;
        let base_y = day * cell_depth;
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

        // Add vertices to list
        vertices.push(v0);
        vertices.push(v1);
        vertices.push(v2);
        vertices.push(v3);
        vertices.push(v4);
        vertices.push(v5);
        vertices.push(v6);
        vertices.push(v7);
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

    // Write to an STL file
    let mut file = File::create("contributions.stl")?;
    stl_io::write_stl(&mut file, triangles.iter().cloned())?;

    Ok(())
}
