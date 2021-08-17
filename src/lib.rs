fn get_tokens(string: &str) -> Vec<String> {
    let mut tokens = Vec::new();
    let mut tok = String::new();
    for c in string.chars() {
        if !c.is_ascii_digit() {
            tokens.push(tok.clone().trim().to_string());
            tok = String::new();
        }
        tok += &c.to_string();
    }
    tokens.push(tok.trim().to_string());

    if tokens[0].starts_with(|c: char| c.is_ascii_digit()) {
        tokens[0].insert(0, '+');
    } else if tokens[0] == "" {
        tokens.remove(0);
    }

    tokens
}

// When u see +- num, while theres no next +- tok get all tokens and calculate them
// get_tokens(), then collect to string, then replace +- with ' +', ' -' ('-34' => ' -35)
// then split at spaces, and collect to vector(then get_tokens() for every element in vec...)
// for easy calculation
fn sort_tokens(string: &str) -> Vec<String> {
    let tokens = get_tokens(string).into_iter().collect::<String>();
    let tokens = tokens.replace('+', " +").replace('-', " -");
    let mut temp = Vec::new();
    for t in tokens.split(' ') {
        temp.push(t.to_string())
    }
    if temp[0] == "" {
        temp.remove(0);
    }

    temp
}

// Works, but not with ()
pub fn evaluate(string: &str) -> Result<f32, String> {
    let mut tokens = sort_tokens(string);
    println!("{:?}", tokens);
    let mut calculated_tokens = Vec::new();
    for t in tokens.iter_mut() {
        let mut to_eval = get_tokens(&t);
        if to_eval.len() == 1 {
            println!("Skipping: {:?}", to_eval);
            calculated_tokens.push(to_eval.remove(0).parse::<f32>().unwrap());
        } else {
            println!("Calculating: {:?}", to_eval);
            // Its known that first el in to_eval is +-
            let mut result = to_eval.remove(0).parse::<f32>().unwrap();
            for mut t in to_eval {
                let sign = t.remove(0);
                let num = t.parse::<f32>().unwrap();
                match sign {
                    '*' => {
                        result *= num;
                    }
                    '/' => {
                        result /= num;
                    }
                    _ => {}
                }
            }
            calculated_tokens.push(result);
        }
    }
    println!("Parsed: {:?}", calculated_tokens);
    let mut result = 0.;
    for t in calculated_tokens {
        result += t;
    }
    Ok(result)
}

#[cfg(test)]
mod tests {
    use crate::evaluate;

    #[test]
    fn no_bracket_test() {
        assert_eq!(evaluate("+1-2").unwrap(), -1.);
        assert_eq!(evaluate("5*5").unwrap(), 25.);
        assert_eq!(evaluate("7/2").unwrap(), 3.5);
        assert_eq!(evaluate("-3*3/3+1").unwrap(), -2.);
    }
}
