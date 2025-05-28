use std::ops::Deref;

use iocraft::prelude::*;
use once_cell::sync::Lazy;
use rayon::prelude::*;
use regex::Regex;

static WORDS: Lazy<Vec<&str>> = Lazy::new(|| include_str!("words_alpha.txt").lines().collect());

fn main() {
    smol::block_on(element!(MainComponent).fullscreen()).unwrap();
}

#[component]
fn MainComponent(mut hooks: Hooks) -> impl Into<AnyElement<'static>> {
    let (width, height) = hooks.use_terminal_size();
    let mut regex_input = hooks.use_state(|| "".to_string());

    let regex_result = hooks.use_memo(|| Regex::new(regex_input.read().deref()), regex_input);

    let matches = hooks.use_memo(
        || match regex_result {
            Err(e) => vec![e.to_string()],
            Ok(re) => WORDS
                .par_iter()
                .filter(|&&n| re.is_match(n))
                .map(|&n| n.to_string())
                .collect(),
        },
        regex_input,
    );

    element! {
        View(
            flex_direction: FlexDirection::Column,
        ) {
            View(
                border_style: BorderStyle::Round,
                border_color: Color::Blue,
                width: width,
            ){
                TextInput(has_focus: true, value: regex_input.to_string(), on_change: move |new| regex_input.set(new))
            }
            View(
                border_style: BorderStyle::Round,
                border_color: Color::Grey,
                flex_direction: FlexDirection::Column,
                flex_wrap: FlexWrap::Wrap,
                width,
                height: height - 3,
                overflow_y: Overflow::Scroll,
                overflow_x: Overflow::Clip,
            ) {
                #(matches.iter().take(325).enumerate().map(|(i, text)| {
                    element! {
                        View( width: 15 ) { Text(
                            key: i,
                            content: format!("{text}  "),
                        )}
                    }
                }))
            }
        }
    }
}
