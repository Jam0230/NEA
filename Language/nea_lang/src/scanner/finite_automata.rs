use crate::scanner::transition_table::load_trans_table;
use regex::Regex;
use std::fmt;

#[derive(Debug, Clone)]
pub struct Token {
    pub _type: String,
    pub contents: String,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}: {:?}]", self._type, self.contents)
    }
}

pub fn lexical_analyse(content: String) -> Result<Vec<Token>, String> {
    let (hash, excepting_states) = load_trans_table();
    let mut char_stream = content.chars();

    let mut current_state = &0; // finite automata state
    let mut next_char = char_stream.next(); // next input character
    let mut contents_buf = String::new(); // contents buffer for the next token

    let mut tokens: Vec<Token> = Vec::new();

    while next_char.is_some() {
        match hash.get(current_state) {
            Some(transitions) => {
                // transition exist for this state
                let mut made_match = false;
                for transition in transitions {
                    let re = Regex::new(transition.0).unwrap();

                    if re.is_match(next_char.unwrap().to_string().as_str()) {
                        // regex for transition made match for next character
                        made_match = true;

                        contents_buf = format!("{}{}", contents_buf, next_char.unwrap());
                        current_state = &transition.1;
                        break;
                    }
                }

                if !made_match {
                    // if no match was made for the transitions
                    if let Some(&(_, match_type)) =
                        excepting_states.iter().find(|x| &x.0 == current_state)
                    {
                        // if current state is accepting
                        if match_type != "Comment" && match_type != "WhiteSpace" {
                            tokens.push(Token {
                                _type: String::from(match_type),
                                contents: String::from(contents_buf),
                            });
                        }
                        contents_buf = String::new();
                        current_state = &0;
                    } else {
                        // lexical error
                        return Err(format!(
                            "Error: Unkown character '{:?}'",
                            next_char.unwrap()
                        ));
                    }
                } else {
                    // if match was made for the transitions
                    next_char = char_stream.next();
                }
            }
            None => {
                if let Some(&(_, match_type)) =
                    excepting_states.iter().find(|x| &x.0 == current_state)
                {
                    // if current state is accepting
                    if match_type != "Comment" && match_type != "WhiteSpace" {
                        tokens.push(Token {
                            _type: String::from(match_type),
                            contents: String::from(contents_buf),
                        });
                    }
                    contents_buf = String::new();
                    current_state = &0;
                } else {
                    // lexical error
                    return Err(format!(
                        "Error: Unkown character '{:?}'",
                        next_char.unwrap()
                    ));
                }
            }
        }
    }

    // adding end of file token
    tokens.push(Token {
        _type: String::from("EOF"),
        contents: String::from("$"),
    });

    Ok(tokens)
}
