#[derive(Clone, Debug)]
struct ParseNaturalLanguage {
    lines: Vec<String>,
}

impl ParseNaturalLanguage {
    /// Extract the interesting parts from the prompt response.
    /// 
    /// The response is supposed to contain a markdown formatted text
    /// with three backticks to mark the beginning and end of a code block.
    /// The code block of interest starts with `prolog`.
    /// 
    /// Within the `prolog` code block, there is supposed to be
    /// a list of `object(input...` and `object(output...` lines.
    /// 
    /// It's the `object(output...` lines that are of interest,
    /// that gets extracted.
    fn parse(multiline_text: &str) -> anyhow::Result<Self> {
        let mut lines_with_prefix = Vec::<String>::new();
        let mut inside_code_block = false;
        let mut count_unrecognized_inside_code_block: usize = 0;
        let mut count_code_block: usize = 0;
        for line in multiline_text.split("\n") {
            let trimmed_line: &str = line.trim();
            if trimmed_line.contains("```prolog") {
                if count_code_block == 0 {
                    inside_code_block = true;
                }
                count_code_block += 1;
                continue;
            }
            if !inside_code_block {
                continue;
            }
            if trimmed_line == "```" {
                inside_code_block = false;
                continue;
            }
            if trimmed_line.is_empty() {
                continue;
            }
            if trimmed_line.starts_with("%") {
                continue;
            }
            if trimmed_line.starts_with("object(input") {
                continue;
            }
            if trimmed_line.starts_with("object(output") {
                lines_with_prefix.push(line.to_string());
                continue;
            }
            count_unrecognized_inside_code_block += 1;
        }
        if count_code_block == 0 {
            anyhow::bail!("No code block found");
        }
        if count_code_block >= 2 {
            anyhow::bail!("Multiple code blocks found");
        }
        if count_unrecognized_inside_code_block > 0 {
            anyhow::bail!("{} unrecognized lines inside the code block", count_unrecognized_inside_code_block);
        }
        let instance = Self {
            lines: lines_with_prefix,
        };
        Ok(instance)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const RESPONSE1: &str = r#"
From the examples given, it appears the transformation from input to output operates as follows:

1. Each object in the input, regardless of its original shape, mass, and transformation properties, is reduced to a 1x1 rectangle in the output.

2. The mass of each object in the output is always 1, irrespective of the mass of the input object.

3. The transformation applied to the output objects is always "all".

4. The bounding coordinates of the output objects are restructured such that the top, left, bottom, and right parameters describe a 1x1 rectangle. 

5. The ordering of the objects in the output seems to be determined by the top coordinate (t), from the lowest to the highest.

Given these rules, the predicted output for Example 4 should be as follows:

```prolog
% Example 4 input grid_width12_height12
object(input4_idP48kmo7_t11_l6_b11_r6_w1_h1_mass1_shapeRectangle_scalex1_scaley1, transform(all)).
object(input4_idP48kmo7_t9_l7_b12_r9_w3_h4_mass6_shapeUnclassified_scalex1_scaley1, transform(rot90_rot270)).
object(input4_idP33ffe7_t2_l2_b3_r3_w2_h2_mass3_shapeL_scalex1_scaley1, transform(rot90_flip)).
object(input4_idP3d53ef_t6_l4_b7_r6_w3_h2_mass5_shapeL_scaleUnknown, transform(rot90_flip)).

% Example 4 output grid_width1_height3
object(output4_idP33ffe7_t1_l1_b1_r1_w1_h1_mass1_shapeRectangle_scalex1_scaley1, transform(all)).
object(output4_idP3d53ef_t2_l1_b2_r1_w1_h1_mass1_shapeRectangle_scalex1_scaley1, transform(all)).
object(output4_idP48kmo7_t3_l1_b3_r1_w1_h1_mass1_shapeRectangle_scalex1_scaley1, transform(all)).
```

Note: Even though there are two objects with the id "idP48kmo7" in the input, only one of them is represented in the output. The one with the lower 't' value is represented following the sorting rule.
"#;

    #[test]
    fn test_10000_parse_ok() {
        // Act
        let actual: ParseNaturalLanguage = ParseNaturalLanguage::parse(RESPONSE1).expect("ok");

        // Assert
        assert_eq!(actual.lines.len(), 3);
    }

    #[test]
    fn test_10100_parse_error() {
        // Arrange
        let s = "Text without code block\n\njunk\nignore";

        // Act
        let error = ParseNaturalLanguage::parse(&s).expect_err("is supposed to fail");

        // Assert
        let message = error.to_string();
        assert_eq!(message, "No code block found");
    }

    #[test]
    fn test_10101_parse_unrecognized_stuff_inside_code_block() {
        // Arrange
        let s = r#"
```prolog
junk1.
junk2.
```
"#;

        // Act
        let error = ParseNaturalLanguage::parse(&s).expect_err("is supposed to fail");

        // Assert
        let message = error.to_string();
        assert_eq!(message, "2 unrecognized lines inside the code block");
    }
}
