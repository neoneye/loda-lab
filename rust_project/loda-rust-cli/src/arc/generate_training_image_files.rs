use super::{Image, ImageExport, ImageOverlay, ImageStack, ImagePadding, Color, ImageSize, OverlayPositionId};
use super::arc_work_model::{Task, Pair, PairType};
use std::path::PathBuf;

// Future experiments
// Order of training pairs: ascending, descending, permuted.
// Order of test pairs: ascending, descending, permuted.
// Position of the image: top-left corner, centered, bottom-right, random x, random y.
// Positions within a single pair between input image and output image: follows same position, no correspondence.
// Positions across pairs: follows same position, no correspondence.
// Colors of the image: normal histogram, reversed histogram, different colors, random colors.
// Amount of previously predicted data: none, pixels from input, all pixels from output, mix of input/output, random junk.
// Use mispredicted output from logistic regression as content in the prediction area.
// Noise pixels in the input data. Can it still make correct predictions despite some noise.
// Image size. Don't always use 30x30 as the image size. Sometimes use a compact representation, such as 10x10.
// Transformation: Double the size of the input images, as long as they stay below 30x30.
// Transformation: Double the size of the output images, as long as they stay below 30x30.
// Transformation: Flip x, flip y, rotate 90, rotate 180, rotate 270.
// Transformation: Pad the input image with 1..3 pixel wide border.
// Transformation: Pad the output image with 1..3 pixel wide border.

pub struct GenerateTrainingImageFiles;

impl GenerateTrainingImageFiles {
    fn take_position_id(index: &mut u64) -> OverlayPositionId {
        let variant: u64 = *index % 5;
        *index /= 5;
        match variant {
            0 => OverlayPositionId::Zero,
            1 => OverlayPositionId::OneThird,
            2 => OverlayPositionId::Half,
            3 => OverlayPositionId::TwoThird,
            _ => OverlayPositionId::One,
        }
    }

    fn generate_pair_image(pair: &Pair, test_index: u8, x: u8, y: u8, mutation_index: u64) -> anyhow::Result<Image> {
        let color_outside: u8 = Color::DarkGrey as u8;
        let color_padding: u8 = Color::LightGrey as u8;
        let color_padding_highlight: u8 = Color::White as u8;

        let mut in_x = OverlayPositionId::Half;
        let mut in_y = OverlayPositionId::Half;
        let mut out_x = OverlayPositionId::Half;
        let mut out_y = OverlayPositionId::Half;

        {
            let mut current_mutation: u64 = mutation_index;

            // Position of the image: top-left corner, centered, bottom-right, random x, random y.
            in_x = Self::take_position_id(&mut current_mutation);
            in_y = Self::take_position_id(&mut current_mutation);
            out_x = Self::take_position_id(&mut current_mutation);
            out_y = Self::take_position_id(&mut current_mutation);
        }

        let mut input = Image::color(30, 30, color_outside);
        input = input.overlay_with_position_id(&pair.input.image, in_x, in_y)?;
        input = input.padding_with_color(1, color_padding)?;

        let the_output: Image;
        match pair.pair_type {
            PairType::Train => {
                let mut output = Image::color(30, 30, color_outside);
                output = output.overlay_with_position_id(&pair.output.image, out_x, out_y)?;
                output = output.padding_with_color(1, color_padding)?;
                the_output = output;
            },
            PairType::Test => {
                let output_size: ImageSize = pair.output.test_image.size();
                let mut image: Image = Image::color(output_size.width, output_size.height, color_outside);
                if pair.test_index == Some(test_index) {
                    _ = image.set(x as i32, y as i32, color_padding_highlight);
                }
                image = image.padding_with_color(1, color_padding_highlight)?;
                let mut output = Image::color(30, 30, color_outside);
                output = output.padding_with_color(1, color_padding)?;
                output = output.overlay_with_position_id(&image, out_x, out_y)?;
                the_output = output;
            }
        }
        let pair_image: Image = input.vjoin(the_output)?;
        Ok(pair_image)
    }

    fn export_image(task: &Task, test_index: u8, x: u8, y: u8, mutation_index: u64, classification: u8) -> anyhow::Result<()> {
        let mut images = Vec::<Image>::new();
        for (_pair_index, pair) in task.pairs.iter().enumerate() {
            let pair_image: Image = Self::generate_pair_image(pair, test_index, x, y, mutation_index)?;
            images.push(pair_image);
        }
        let task_image: Image = Image::hstack(images)?;

        let filename = format!("{}_test{}_x{}_y{}_color{}_mutation{}.png", task.id, test_index, x, y, classification, mutation_index);
        let basepath: PathBuf = PathBuf::from("/Users/neoneye/Downloads/image_save");
        let path: PathBuf = basepath.join(filename);
        task_image.save_as_file(&path)?;
        Ok(())
    }
    
    fn export_test_pairs(task: &Task, test_index: u8, mutation_index: u64) -> anyhow::Result<()> {
        for (_pair_index, pair) in task.pairs.iter().enumerate() {
            if pair.test_index != Some(test_index) {
                continue;
            }
            let output_image: Image = pair.output.test_image.clone();
            let output_size: ImageSize = output_image.size();
            for y in 0..output_size.height {
                for x in 0..output_size.width {
                    let classification: u8 = output_image.get(x as i32, y as i32).unwrap_or(255);
                    Self::export_image(task, test_index, x, y, mutation_index, classification)?;
                }
            }
        }
        Ok(())
    }

    pub fn export_task_with_mutation(task: &Task, mutation_index: u64) -> anyhow::Result<()> {
        let count_test: u8 = task.count_test().min(255) as u8;
        for test_index in 0..count_test {
            Self::export_test_pairs(task, test_index, mutation_index)?;
        }        
        Ok(())
    }

    pub fn export_task(task: &Task) -> anyhow::Result<()> {
        let mutation_indexes: [u64; 4] = [
            0,
            4,
            // 12,
            // 20,
            // 24,
            312,
            624,
        ];
        for mutation_index in mutation_indexes {
            Self::export_task_with_mutation(task, mutation_index)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::arc::arc_json_model;
    use crate::arc::arc_work_model::Task;

    fn save_as_file(name: &str) -> anyhow::Result<()> {
        let json_task: arc_json_model::Task = arc_json_model::Task::load_testdata(name)?;
        let task: Task = Task::try_from(&json_task)?;
        GenerateTrainingImageFiles::export_task(&task)?;
        Ok(())
    }

    // #[test]
    fn test_90000_overlay_cf98881b() {
        save_as_file("cf98881b").expect("ok");
    }

    // #[test]
    fn test_90001_overlay_281123b4() {
        save_as_file("281123b4").expect("ok");
    }

    // #[test]
    fn test_90002_overlay_e98196ab() {
        save_as_file("e98196ab").expect("ok");
    }
}
