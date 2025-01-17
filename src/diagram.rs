use artimonist::{Diagram, SimpleDiagram};
use unicode_width::UnicodeWidthChar;

/// get unicode middle string
fn unicode_middle(c: char, width: usize) -> String {
    let w = c.width().unwrap_or_default();
    let right = (width - w) / 2;
    let left = width - w - right;
    format!("{:left$}{c}{:right$}", "", "")
}

#[derive(Debug)]
pub struct TDiagram(pub SimpleDiagram);

const CW: usize = 5; // cell width

/// draw line by content, left, middle, right
fn grid_line(content: char, left: char, middle: char, right: char) -> String {
    let s = content.to_string().repeat(CW);
    format!("{left}{}{s}{right}", format!("{s}{middle}").repeat(6))
}

impl std::fmt::Display for TDiagram {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        macro_rules! grid_line {
            ($c: expr, $l: expr, $m: expr, $r: expr) => {{
                writeln!(f, "{}", grid_line($c, $l, $m, $r))?;
            }};
        }
        let content_line = |row| {
            (0..7)
                .map(|col| match self.0.get(row, col) {
                    Some(&Some(ch)) => unicode_middle(ch, CW),
                    _ => format!("{:CW$}", ""),
                })
                .collect::<Vec<String>>()
                .join("│")
        };

        grid_line!('─', '┌', '┬', '┐'); // top line
        for row in 0..7 {
            writeln!(f, "│{}│", content_line(row))?; // content line
            match row {
                6 => grid_line!('─', '└', '┴', '┘'), // bottom line
                _ => grid_line!('─', '├', '┼', '┤'), // middle line
            };
        }
        Ok(())
    }
}
