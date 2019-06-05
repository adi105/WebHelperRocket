use std::collections::LinkedList;
//===============================================
// strips the whitespace from a string, returning
// that string without whitespace.
//===============================================
fn strip_whitespace(input: String) -> String {
    let to_strip = " ";
    input.chars().filter(|&c| !to_strip.contains(c)).collect()
}

//UNIT TESTS=====================================
#[test]
fn test_whitespace() {
    assert_eq!(strip_whitespace("George   likes      bananas!   ".to_string()), "Georgelikesbananas!");
    assert_eq!(strip_whitespace("     whitespace   at the start.".to_string()), "whitespaceatthestart.");
}

//===============================================
// returns true if the char provided is an operator.
// operator is defined by our own terms for our calculator here.
//===============================================
fn is_operator(c: char) -> bool {
    if c == '+' || c == '-' || c == '/' || c == '*' || c == '(' || c == ')' || c == '^' {
        return true;
    } else {
        return false;
    }
}

//UNIT TESTS======================================
#[test]
fn test_operator_inspection() {
    assert_eq!(is_operator('+'), true); //an operator
    assert_eq!(is_operator('2'), false); //a  digit
    assert_eq!(is_operator('z'), false); //a alphabetic letter
    assert_eq!(is_operator(' '), false); //whitespace
}

//=======================================================
// Function that checks if the input string is valid.
// Valid = either a digit (0-9) or an operator (defined in a previous function)
//=======================================================
fn is_string_valid(to_check: String) -> bool {
    //test for empty string first?
    if to_check == "" {
        return false;
    }
    for c in to_check.chars() {
        if c.is_digit(10) == false {
            if !is_operator(c) {
                return false;
            }
        }
    }

    return true
}

//UNIT TESTS================================================
#[test]
fn test_string_validation() {
    assert_eq!(is_string_valid("2+3-7".to_string()), true); //numbers and operators
    assert_eq!(is_string_valid("2 + 3 - 7".to_string()), false); //whitespace
    assert_eq!(is_string_valid("(3*2)-7".to_string()), true); //parenthesis
    let value:String = "(3*2)/7+5".to_string();
    assert_eq!(is_string_valid(value), true); //using variables
}
//======================================================================
// Function that returns the weight of the given operator. This is used
// to ensure that PEMDAS is followed.
//======================================================================
fn op_weight(operator: &char) -> u32 {
    let weight:u32;
    match operator {
        '*' | '/' => weight = 2,
        '+' | '-' => weight = 1,
        _ => weight = 0
    }
    weight
}

//UNIT TESTS=============================================================
#[test]
fn test_op_weight() {
    assert_eq!(op_weight(&'+'), 1);
    assert_eq!(op_weight(&'*'), 2);
    assert_eq!(op_weight(&' '), 0);
}

//======================================================================
// Function that takes two arguments, c1 and c2, and returns true if c1
// has higher precendence than c2 in context of PEMDAS.
//=======================================================================
fn has_higher_precedence(c1: &char, c2: &char) -> bool {
    let w1 = op_weight(c1);
    let w2 = op_weight(c2);

    //If w1 >= w2, we want to return true
    if w1 == w2 {
        true
    } else if w1 > w2 {
        true
    } else {
        false
    }
}

//UNIT TESTS=============================================================
#[test]
fn test_precedence() {
    assert_eq!(has_higher_precedence(&'+', &'*'), false);
    assert_eq!(has_higher_precedence(&'/', &'-'), true);
}

//=======================================================================
// Function that converts a given infix string to a postfix string using
// a stack. To be able to represent numbers greater than 9, we use a whitespace
// to imply a term. 23+523 => 23 523 +
//=======================================================================
fn infix_to_postfix(input: &String) -> String {
    let mut postfix:String = "".to_string();
    let mut stack: LinkedList<char> = LinkedList::new();
    //this bool is a flag to symbolize that we are recording a multi-digit number
    let mut number:bool = false;

    //algorithm using the "stack" to create postfix string
    for c in input.chars() {
        //if c is a number, add it to the string
        if c.is_digit(10) {
            postfix.push(c);
            number = true;
        }
        //if c is a open paren, push it onto the stack
        else if c == '(' {
            stack.push_back(c);
            //paren implies the end of an existing number.
            if number == true {
                postfix.push(' ');
                number = false;
            }
        }
        //enforce the rules of operator precedence when creating postfix
        else if c == '+' || c == '-' || c == '*' || c == '/' {
            if number == true {
                postfix.push(' ');
                number = false;
            }
            //if the stack has things, and the thing on the stack has equal or higher precedence,
            //we pop off the stack, add to string. Once that stops being the case, we push the given
            //operator onto the stack. This enforces PEMDAS.
            if !stack.is_empty() {
                while stack.is_empty() == false && (has_higher_precedence(stack.back().unwrap(), &c)) {
                    postfix.push(*stack.back().unwrap());
                    if is_operator(*stack.back().unwrap()) == true { //I believe this SHOULD always be true, but just in case...
                        postfix.push(' ');
                    }
                    stack.pop_back();
                }
                stack.push_back(c);
            }
            else if stack.is_empty() == true {
                stack.push_back(c);
            }
        }
        //when we find a close paren, we need to find the open paren and push the inside of it on the postfix
        else if c == ')' {
            while stack.is_empty() == false && *stack.back().unwrap() != '(' {
                postfix.push(' ');
                postfix.push(*stack.back().unwrap());
                stack.pop_back();
            }
            stack.pop_back();
        }
    }

    //once the string is consumed, deal with the remaining stack values
    while stack.is_empty() == false {
        if *stack.back().unwrap() != '(' && *stack.back().unwrap() != ')' {
            postfix.push(' ');
            postfix.push(*stack.back().unwrap());
            stack.pop_back();
        } else {
            stack.pop_back();
        }
    }
    return postfix
}

//UNIT TESTS=================================================================
#[test]
fn test_postfix() {
    assert_eq!(infix_to_postfix(&"2+3-5".to_string()), "2 3 + 5 -");
    assert_eq!(infix_to_postfix(&"(22-37)*5".to_string()), "22 37 - 5 *");
    let testval:String = "(527 + (254 / 3)) + 71".to_string();
    assert_eq!(infix_to_postfix(&testval), "527 254 3 / + 71 +");
    assert_eq!(infix_to_postfix(&"2/7".to_string()), "2 7 /");
}

//============================================================================
// Function which takes the sanitized, validated, and postfixed string, and performs
// the actual math. This function returns the answer as a string, which should then
// be shown to the user as being the result.
//=============================================================================
fn perform_math(statement:String) -> String {
    let mut varstack: LinkedList<f64> = LinkedList::new();
    let mut var: String = "".to_string();
    //bool flags
    let mut last_was_op:bool = false;

    for c in statement.chars() {
        if c.is_digit(10) {
            var.push(c);
        } else if c == ' ' {
            if last_was_op == false {
                varstack.push_back(var.parse::<f64>().unwrap());
                var = "".to_string();
            } else {
                last_was_op = false;
            }
        } else if is_operator(c) == true {
            //do math, wipe var2
            let result:f64;
            let var2 = varstack.pop_back().unwrap();
            let var1 = varstack.pop_back().unwrap();
            match c {
                '+' => result = var1 + var2,
                '-' => result = var1 - var2,
                '*' => result = var1 * var2,
                '/' => result = var1 / var2,
                _ => result = 0.0,
            }
            varstack.push_back(result);
            last_was_op = true;
        }
    }
    let retval:f64 = varstack.pop_back().unwrap();
    return retval.to_string()
}

#[test]
fn test_math() {
    assert_eq!(perform_math("2 3 +".to_string()), "5");
    assert_eq!(perform_math("24 37 + 15 -".to_string()), "46");
    assert_eq!(perform_math("500 250 / 17 *".to_string()), "34");
    //assert_eq!(perform_math());

}

//============================================================================
// After some testing, I realized that I need to have another sanitization function, not for tokenizing
// but for parsing, and making sure what comes out can be mathematically computed. This boils down to
// having any operator surrounded by numbers. (This function ended up being very complex. I drew a state machine)
// but did not optimize it so it might not need as many states. But it works!
//=============================================================================
fn can_be_computed(statement:String) -> bool {
    let mut prev_isnum:bool = false; //memory for if the previous char was a digit
    let mut next_isnum:bool = false; //flag that states if the next value must be a digit
    let mut next_must_be_op:bool = false; //flag that states if the next value must be an operator
    let mut opened_paren:u64 = 0; //a parenthesis count to make sure count('(') == count(')')

    for c in statement.chars() {
        if is_operator(c) == true {
            //if we have an open paren, check if the prev is a number. if not, "reset" flags.
            if c == '(' {
                if prev_isnum == true {
                    return false;
                }
                opened_paren += 1;
                next_isnum = false;
                prev_isnum = false;
                continue;
            }
            //if we have closed paren, make sure the previous char is a number, and that a ( exists.
            if c == ')' {
                if prev_isnum == false {
                    return false;
                }
                if opened_paren == 0 {
                    return false;
                }
                opened_paren -= 1;
                next_must_be_op = true; //there needs to be an operator, not a digit, after a )
                prev_isnum = true;
                next_isnum = false;
                continue;
            }

            if next_isnum == true {
                return false;
            }
            if prev_isnum == false {
                return false;
            }
            next_isnum = true;
            prev_isnum = false;
            if next_must_be_op == true {
                next_must_be_op = false;
            }
        }
        if c.is_digit(10) == true {
            if next_must_be_op == true {
                return false;
            }
            prev_isnum = true;
            next_isnum = false;
        }

    }
    //once the string is parsed, if we have any unfinished business, we return false
    if next_isnum == false && opened_paren == 0 {
        return true
    } else {
        return false
    }
}

//UNIT TESTS=================================================================
#[test]
fn test_parsing() {
    assert_eq!(can_be_computed("2+3-6".to_string()), true);
    assert_eq!(can_be_computed("23/72+(25-3)".to_string()), true);
    assert_eq!(can_be_computed("23/5(23)".to_string()), false);
    assert_eq!(can_be_computed("2/3/5+7-(2+53".to_string()), false);
    assert_eq!(can_be_computed("22*(350/42)*7".to_string()), true);
}

//============================================================================
// Function that performs the calculation. This should be the function that gets
// called with user input and performs the entire procedure.
//============================================================================
pub fn calculate(statement:String) -> Result<String, String> {
    //first we must strip the whitespace from the input
    let stripped = &strip_whitespace(statement);
    println!("Stripped:{}", stripped);
    //Then, if the string is invalid, we must return
    if is_string_valid(stripped.to_string()) == false {
        return Err(format!("You have entered an invalid statement."));
    }

    //next, we make sure the string is parsable.
    if can_be_computed(stripped.to_string()) == false {
        return Err(format!("Parsing error."));
    }

    //If all this works, we then need to convert to postfix
    let postfix = infix_to_postfix(stripped);
    println!("post:{}", postfix);

    //now, pass the result of the postfix cast to the math function, and return its result.
    let result = perform_math(postfix);
    return Ok(result)
}
