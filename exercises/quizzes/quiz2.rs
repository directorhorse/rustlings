// This is a quiz for the following sections:
// - Strings
// - Vecs
// - Move semantics
// - Modules
// - Enums
//
// Let's build a little machine in the form of a function. As input, we're going
// to give a list of strings and commands. These commands determine what action
// is going to be applied to the string. It can either be:
// - Uppercase the string
// - Trim the string
// - Append "bar" to the string a specified amount of times
//
// The exact form of this will be:
// - The input is going to be a Vector of 2-length tuples,
//   the first element is the string, the second one is the command.
// - The output element is going to be a vector of strings.

enum Command {
    Uppercase,
    Trim,
    Append(usize),
}

mod my_module {
    use std::char::ToUppercase;

    use super::Command;

    // TODO: Complete the function.
    pub fn transformer(input: Vec<(String,Command)>) -> Vec<String> {
		let mut s = Vec::new();
		for i in &input{
			match i.1 {
				Command::Uppercase=>{s.push((i.0).to_uppercase())},
				Command::Trim=>{s.push(String::from((i.0).trim()))},
				Command::Append(num)=>{s.push({
					let mut s_t = i.0.clone();
					for j in (0..num){
						s_t += &i.0;
					}
					s_t
				})},
			}
		}
		s
	}
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    // TODO: What do we need to import to have `transformer` in scope?
    use super::my_module::transformer;
    use super::Command;

    #[test]
    fn it_works() {
        let input = vec![
            ("hello".to_string(), Command::Uppercase),
            (" all roads lead to rome! ".to_string(), Command::Trim),
            ("foo".to_string(), Command::Append(1)),
            ("bar".to_string(), Command::Append(5)),
        ];
        let output = transformer(input);

        assert_eq!(
            output,
            [
                "HELLO",
                "all roads lead to rome!",
                "foofoo",
                "barbarbarbarbarbar",
            ]
        );
    }
}
