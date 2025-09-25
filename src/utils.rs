enum QuoteState {
    None,
    Single,
    Double,
}

pub fn parse_args(s: &str) -> Vec<String> {
	let mut quote_state = QuoteState::None;
	let mut current_token: String = String::new();
	let mut tokens: Vec<String> = Vec::new();
	let mut is_escaped = false;

	let chars: Vec<char> = s.chars().collect();
	// for ch in s.chars() {
	for i in 0..chars.len() {
		let ch = chars[i];
		match ch {
			'\'' => {
				if !is_escaped {
					match quote_state {
						QuoteState::None => quote_state = QuoteState::Single,
						QuoteState::Single => quote_state = QuoteState::None,
						QuoteState::Double => current_token.push(ch),
					}
				} else {
					current_token.push(ch);
					is_escaped = !is_escaped;
				}
			}
			'"' => {
				if !is_escaped {
					match quote_state {
						QuoteState::None => quote_state = QuoteState::Double,
						QuoteState::Double => quote_state = QuoteState::None,
						QuoteState::Single => current_token.push(ch),
					}
				} else {
					current_token.push(ch);
					is_escaped = !is_escaped;
				}
			}
			' ' => {
				if !is_escaped {
					match quote_state {
						QuoteState::None => {
							if !current_token.is_empty() {
								tokens.push(current_token.clone());
								current_token.clear();
							}
						}
						_ => current_token.push(ch),
					}
				} else {
					current_token.push(ch);
					is_escaped = !is_escaped;
				}
			}
			'\\' => match quote_state {
				QuoteState::None => is_escaped = true,
				QuoteState::Double => if !is_escaped {
					if i + 1 < chars.len() {
						let next_ch = chars[i + 1];
						if next_ch == '\\' || next_ch == '"' {
							is_escaped = true;
						} else {
							current_token.push(ch);
						}
					}
				} else {
					current_token.push(ch);
					is_escaped = false;
				},
				_ => current_token.push(ch),
			},
			_ => current_token.push(ch),
		}
	}

	if !current_token.is_empty() {
		tokens.push(current_token)
	}

	return tokens;
}

// 3.1.2.1 Escape Character

// A non-quoted backslash ‘\’ is the Bash escape character. It preserves the literal value of the next character that follows, removing any special meaning it has, with the exception of newline. If a \newline pair appears, and the backslash itself is not quoted, the \newline is treated as a line continuation (that is, it is removed from the input stream and effectively ignored).

// Next: Double Quotes, Previous: Escape Character, Up: Quoting   [Contents][Index]
// 3.1.2.2 Single Quotes

// Enclosing characters in single quotes (‘'’) preserves the literal value of each character within the quotes. A single quote may not occur between single quotes, even when preceded by a backslash.

// Next: ANSI-C Quoting, Previous: Single Quotes, Up: Quoting   [Contents][Index]
// 3.1.2.3 Double Quotes

// Enclosing characters in double quotes (‘"’) preserves the literal value of all characters within the quotes, with the exception of ‘$’, ‘`’, ‘\’, and, when history expansion is enabled, ‘!’. When the shell is in POSIX mode (see Bash and POSIX), the ‘!’ has no special meaning within double quotes, even when history expansion is enabled. The characters ‘$’ and ‘`’ retain their special meaning within double quotes (see Shell Expansions). The backslash retains its special meaning only when followed by one of the following characters: ‘$’, ‘`’, ‘"’, ‘\’, or newline. Within double quotes, backslashes that are followed by one of these characters are removed. Backslashes preceding characters without a special meaning are left unmodified.

// A double quote may be quoted within double quotes by preceding it with a backslash. If enabled, history expansion will be performed unless an ‘!’ appearing in double quotes is escaped using a backslash. The backslash preceding the ‘!’ is not removed.

// The special parameters ‘*’ and ‘@’ have special meaning when in double quotes (see Shell Parameter Expansion).
