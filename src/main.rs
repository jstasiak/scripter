use std::fs;

fn main() {
    println!("Hello, world!");
}

fn script_to_tex(input: &str) -> String {
    let mut output: Vec<u8> = Vec::new();
    let mut lines = input
        .split('\n')
        .map(|line| line.trim())
        .filter(|line| *line != "");
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
            } else if line.contains(':') {
                let parts: Vec<&str> = line.split(':').collect();
                assert_eq!(parts.len(), 2);
                let speaker = &parts[0];
                let text = &parts[1].trim();
                format!(
                    "\\begin{{dialogue}}{{{}}}{}\\end{{dialogue}}",
                    speaker, text
                )
            } else {
                line.to_string()
            }
            .bytes(),
        );
        output.extend("\n".bytes());
    }
    output.extend(
        "\\theend
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
