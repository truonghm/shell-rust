enum QuoteState {
	None,
	Single,
	Double,
}

pub fn parse_args(s: &str) -> Vec<String> {
	if  !s.contains("'") && !s.contains("\"") {
		return s.trim().split_whitespace().map(|s| s.to_string()).collect()
	} else {
		let mut quote_state = QuoteState::None;
		let mut current_token: String = String::new();
		let mut tokens: Vec<String> = Vec::new();

		for ch in s.chars() {
			// if ch == '\'' || ch == '"' {
			// 	inside_quotes = !inside_quotes; 
			// } else if ch == ' ' {
			// 	if inside_quotes {
			// 		current_token.push(ch);
			// 	} else if !current_token.is_empty() {
			// 		// TODO: this is not efficient, needs to be improved later
			// 		tokens.push(current_token.clone());
			// 		current_token.clear();
			// 	}
			// } else {
			// 	current_token.push(ch);
			// }
			match ch {
				'\'' => match quote_state {
					QuoteState::None => quote_state = QuoteState::Single,
					QuoteState::Single => quote_state = QuoteState::None,
					QuoteState::Double => current_token.push(ch),
				},
				'"' => match quote_state {
					QuoteState::None => quote_state = QuoteState::Double,
					QuoteState::Double => quote_state = QuoteState::None,
					QuoteState::Single => current_token.push(ch),
				},
				' ' => match quote_state {
					QuoteState::None => if !current_token.is_empty() {
						tokens.push(current_token.clone());
						current_token.clear();
					},
					_ => current_token.push(ch),
				}
				_ => current_token.push(ch)
			}
		}

		if !current_token.is_empty() {
			tokens.push(current_token)
		}

		return tokens
	}
}