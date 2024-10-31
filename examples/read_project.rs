use clap::Parser;
use std::path::PathBuf;
use supervisely_format::{DatasetKind, Project};

/// Supervisely dataset reader.
#[derive(Parser)]
struct Opts {
    /// The Supervisely project directory to be read.
    project_dir: PathBuf,
}

fn main() -> eyre::Result<()> {
    let Opts { project_dir } = Opts::parse();

    println!("project: {}", project_dir.display());
    let project = Project::open(&project_dir)?;

    for (dataset_name, dataset) in &project.datasets {
        match &dataset.kind {
            DatasetKind::Image(dataset) => {
                println!(
                    "- image dataset {dataset_name}: {}",
                    project_dir.join(dataset_name).display()
                );
                println!("  images:");

                for image_name in dataset.image_names.iter() {
                    println!("    - {image_name}");

                    let media = dataset.get_image(image_name).unwrap();
                    let ann = media.ann()?;
                    println!("      objects (n={})", ann.objects.len());

                    for obj in &ann.objects {
                        println!("      - object: {}", obj.id);

                        if let Some(class_id) = obj.class_id {
                            println!("        class: {class_id}");
                        }
                    }
                }
            }
            DatasetKind::Video(dataset) => {
                println!(
                    "- video dataset {dataset_name}: {}",
                    project_dir.join(dataset_name).display()
                );
                println!("  videos:");

                for video_name in dataset.video_names.iter() {
                    println!("    - {video_name}");

                    let media = dataset.get_video(video_name).unwrap();
                    let ann = media.ann()?;
                    println!("      objects (n={})", ann.objects.len());

                    for obj in &ann.objects {
                        println!("      - object: {}", obj.key);

                        if let Some(class_title) = &obj.class_title {
                            println!("        class: {}", class_title);
                        }
                    }
                }
            }
            DatasetKind::PointCloud(dataset) => {
                println!(
                    "- point_cloud dataset {dataset_name}: {}",
                    project_dir.join(dataset_name).display()
                );
                println!("  point_clouds:");

                for point_cloud_name in dataset.point_cloud_names.iter() {
                    println!("    - {point_cloud_name}");

                    let media = dataset.get_point_cloud(point_cloud_name).unwrap();
                    let ann = media.ann()?;
                    println!("      objects (n={})", ann.objects.len());

                    for obj in &ann.objects {
                        println!("      - object: {}", obj.key);
                        println!("        class: {}", obj.class_title);
                    }
                }
            }
            DatasetKind::PointCloudEpisode(dataset) => {
                println!(
                    "- point cloud espisode dataset {dataset_name}: {}",
                    project_dir.join(dataset_name).display()
                );

                let frame_iter = dataset.frame_id_iter();

                let objects = &dataset.annotation.objects;
                println!("  - {} objects", dataset.annotation.objects.len());
                for obj in objects {
                    println!("    - object: {}", obj.key);
                    println!("      class: {}", obj.class_title);
                }

                println!("  - {} frames", frame_iter.len());

                for frame_id in frame_iter {
                    let frame = dataset.get_frame(frame_id).unwrap();

                    println!("    - frame {}", frame_id);
                    println!("      {} figures", frame.annotation.figures.len());
                }
            }
        }
    }

    Ok(())
}
