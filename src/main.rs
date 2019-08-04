use std::env;
use std::fs;
use std::path::Path;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} INPUT", args[0]);
        process::exit(1);
    }
    let input_path = Path::new(&args[1]);
    let input = fs::read_to_string(input_path).expect("Cannot read the input file");
    let tex_output = script_to_tex(&input);
    let tex_path = input_path.with_extension("tex");
    fs::write(&tex_path, tex_output).expect("Cannot write to the output file");

    process::Command::new("pdflatex")
        .args(&[&tex_path.to_str().unwrap()])
        .status()
        .expect("Failed to execute pdflatex");
}

fn script_to_tex(input: &str) -> String {
    let mut output: Vec<u8> = Vec::new();
    let mut lines = input.split('\n').filter(|line| line.trim() != "");
    let title = lines.next().expect("Title not found");
    let author = lines.next().expect("Author not found");

    output.extend(
        format!(
            "\\documentclass{{screenplay}}
\\usepackage[T1]{{fontenc}}
\\usepackage[polish]{{babel}}
\\usepackage[utf8]{{inputenc}}
\\title{{{}}}
\\author{{{}}}
\\begin{{document}}
\\coverpage
\\fadein
",
            title, author,
        )
        .bytes(),
    );

    for line in lines {
        output.extend(
            if line[..5] == *"INT. " {
                format!("\\intslug{{{}}}", &line[5..])
            } else if line[..5] == *"EXT. " {
                format!("\\extslug{{{}}}", &line[5..])
            // If we have whitespace at the start of the line trim_start() will return something
            // different than the original slice and we'll know we have a dialogue.
            } else if line.trim_start() != line {
                let line = line.trim();
                let parts: Vec<&str> = line.splitn(2, ':').collect();
                assert_eq!(parts.len(), 2);
                let speaker = &parts[0];
                let text = &parts[1].trim().replace('(', "\\paren{").replace(')', "}");
                format!(
                    "\\begin{{dialogue}}{{{}}}{}\\end{{dialogue}}",
                    speaker, text
                )
            } else {
                line.to_string()
            }
            .bytes(),
        );
        output.extend("\n\n".bytes());
    }
    output.extend(
        "\\fadeout
\\theend
\\end{document}\n"
            .bytes(),
    );
    String::from_utf8(output).unwrap()
}

#[cfg(test)]
mod tests {
    use crate::script_to_tex;
    use std::fs;

    #[test]
    fn test_something() {
        let input = fs::read_to_string("test_script.script").unwrap();
        let expected_output = fs::read_to_string("test_script.tex").unwrap();
        let actual_output = script_to_tex(&input);
        eprintln!("\n=== Input: ===\n\n{}", input);
        eprintln!("\n=== Expected output: ===\n\n{}", expected_output);
        eprintln!("\n=== Actual output: ===\n\n{}", actual_output);
        assert_eq!(actual_output, expected_output);
    }
}
