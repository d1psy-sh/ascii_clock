// these are the numbers that will be rendered to the screen
const CHAR_HEIGHT: usize = 6;

/// match the ascii chars in the given string to the big ascii art chars here
/// return a Text struct for the tui lib
pub fn render_ascii(text: &String) -> String {
    let mut ascii_text: Vec<String> = vec!["".to_string(); CHAR_HEIGHT];
    for c in text.chars() {
        ascii_text = push_letter(ascii_text, c);
    }
    let res = ascii_text.join("\n");
    // dbg!(res)
    res
}

fn push_letter(current: Vec<String>, letter: char) -> Vec<String> {
    let mut tmp: Vec<String> = vec![];
    match letter {
        '0' => {
            tmp = ZERO
                .split("\n")
                .into_iter()
                .map(|x| x.to_string())
                .collect();
        }
        '1' => {
            tmp = ONE.split("\n").into_iter().map(|x| x.to_string()).collect();
        }
        '2' => {
            tmp = TWO.split("\n").into_iter().map(|x| x.to_string()).collect();
        }
        '3' => {
            tmp = THREE
                .split("\n")
                .into_iter()
                .map(|x| x.to_string())
                .collect();
        }
        '4' => {
            tmp = FOUR
                .split("\n")
                .into_iter()
                .map(|x| x.to_string())
                .collect();
        }
        '5' => {
            tmp = FIVE
                .split("\n")
                .into_iter()
                .map(|x| x.to_string())
                .collect();
        }
        '6' => {
            tmp = SIX.split("\n").into_iter().map(|x| x.to_string()).collect();
        }
        '7' => {
            tmp = SEVEN
                .split("\n")
                .into_iter()
                .map(|x| x.to_string())
                .collect();
        }
        '8' => {
            tmp = EIGHT
                .split("\n")
                .into_iter()
                .map(|x| x.to_string())
                .collect();
        }
        '9' => {
            tmp = NINE
                .split("\n")
                .into_iter()
                .map(|x| x.to_string())
                .collect();
        }
        ':' => {
            tmp = COLON
                .split("\n")
                .into_iter()
                .map(|x| x.to_string())
                .collect::<Vec<String>>();
        }
        _ => {
            dbg!(letter);
        }
    }
    let mut res: Vec<String> = vec![];
    for (idx, line) in current.into_iter().enumerate() {
        res.push(format!("{}{}", line, tmp[idx]));
    }
    res
}

pub fn set_margin(text: Vec<String>, horizontal: usize, vertical: usize) -> Vec<String> {
    let mut hmargined: Vec<String> = text.into_iter()
        .map(|x| format!("{}{}", " ".repeat(horizontal), x))
        .collect();
    let mut top_margin = vec!["\n".to_string(); vertical];
    top_margin.append(&mut hmargined);
    top_margin
}

const ONE: &str = r#" ██╗
███║
╚██║
 ██║
 ██║
 ╚═╝"#;
const TWO: &str = r#"██████╗ 
╚════██╗
 █████╔╝
██╔═══╝ 
███████╗
╚══════╝"#;
const THREE: &str = r#"██████╗ 
╚════██╗
 █████╔╝
 ╚═══██╗
██████╔╝
╚═════╝ "#;
const FOUR: &str = r#"██╗  ██╗
██║  ██║
███████║
╚════██║
     ██║
     ╚═╝"#;
const FIVE: &str = r#"███████╗
██╔════╝
███████╗
╚════██║
███████║
╚══════╝"#;
const SIX: &str = r#" ██████╗ 
██╔════╝ 
███████╗ 
██╔═══██╗
╚██████╔╝
 ╚═════╝ "#;
const SEVEN: &str = r#"███████╗
╚════██║
    ██╔╝
   ██╔╝ 
   ██║  
   ╚═╝  "#;
const EIGHT: &str = r#" █████╗ 
██╔══██╗
╚█████╔╝
██╔══██╗
╚█████╔╝
 ╚════╝ "#;
const NINE: &str = r#" █████╗ 
██╔══██╗
╚██████║
 ╚═══██║
 █████╔╝
 ╚════╝ "#;
const ZERO: &str = r#" ██████╗ 
██╔═████╗
██║██╔██║
████╔╝██║
╚██████╔╝
 ╚═════╝ "#;

const COLON: &str = r#"   
██╗
╚═╝
██╗
╚═╝
   "#;
