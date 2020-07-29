/// Module holding all the mathematical logic to actually calculate a result
mod logic {
    use std::str::FromStr;
    
    // Trait to allow subscription access to chars in Strings using ch function.
    trait StringSubscription {
        fn ch(&self, pos: usize) -> char; 
    }
    impl StringSubscription for String {
        fn ch(&self, pos: usize) -> char {
            self.chars().nth(pos).unwrap()
        }
    }



    /*****************************
     * Function to check if given character is an operator or not.
     *
     * # Arguments
     * * `c` - Character to check.
     *
     * # Return
     * Boolean. Returns true if `c` is an operator.
     *
     * # Example
     * ```
     * assert_eq!(is_operator('+'), true);
     * assert_eq!(is_operator('x'), false);
     * ```
     ******************************/
    fn is_operator(c: char) -> bool {
        let operators : [char; 4] = ['+', '-', '*', '/'];
        if operators.contains(&c) {
            true
        } else {
            false
        }
    }

    /**
     * Function to get precedence of an operator. This to determine
     * which operator to evaluate first.
     *
     * # Arguments
     * * `operator` - Char. Operator to check
     *
     * # Return
     * i8. Returns an integer representing the precedence of the `operator`.
     *
     * # Example
     * ```
     * assert_eq!(precedence('+') < precedence('*'), true);
     * assert_eq!(precedence('*') == precedence('/'), true);
     * assert_eq!(precedence('-') >= precedence('/'), false);
     * ```
     */
    fn precedence(operator: char) -> i8 {
        match operator {
            '+' => 1,
            '-' => 1,
            '*' => 2,
            '/' => 2,
            _ => -1,
        }
    }

    /**
     * Function to convert an infix string to postfix notation.
     *
     * # Arguments
     * * `infix` - String containing the infix.
     *
     * # Return
     * Returns a string in postfix notation.
     *
     * # Example
     * ```
     * let s : String = "5+5";
     * assert_eq!(to_postfix(s), "5 5 +");
     * ```
     *
     */
    fn to_postfix(mut infix: String) -> String {
        let mut postfix : String = "".to_string();

        // Stack to temporarily hold operators, and nr is
        // a helper variable to group digits together in their number
        let mut stack : std::vec::Vec<char> = std::vec::Vec::new();
        let mut nr : String = "".to_string();

        // If first character in infix is a minus-operator (AKA first number is a negative)
        // Add "0" to create "0-[number]"
        if infix.ch(0) == '-' {
            infix = format!("{}{}", "0", infix);
        }

        // Looping over infix string
        let mut i = 0;
        while i < infix.len() {
    
            // If currently evaluated character ain't an operator, it's a digit
            if !is_operator(infix.ch(i)) {
               
                // If digit is first one in a group of digits (AKA a number)
                // put that number in nr
                while i < infix.len() && !is_operator(infix.ch(i))  {
                    nr = format!("{}{}", nr, infix.chars().nth(i).unwrap());
                    i = i + 1;
                }

                i = i - 1;

                // Append number to postfix string
                postfix = format!("{}{}{}", postfix, nr, ' ');
                nr = "".to_string();
            } else {
                // This block is executed when the evaluated character is an operator

                // If the stack is empty, or the evaluated operator has a higher precedence than the
                // one in the stack, push it (Needs to be appended to the postfix string later)
                if stack.is_empty() || precedence(infix.ch(i)) > precedence(*stack.last().unwrap()) {
                    stack.push(infix.ch(i));
                } else {
                    // While the stack contains a higher or equally high precedence as the
                    // evaluated character: append top of stack to postfix string
                    while precedence(*stack.last().unwrap()) >= precedence(infix.ch(i)) {
                        postfix = format!("{}{}{}", postfix, stack.pop().unwrap(), ' ');
                        
                        if stack.is_empty() {
                            break;
                        }
                    }

                    // Push evaluated operator to stack
                    stack.push(infix.ch(i));
                }

            }

            i = i + 1;
        }

        // Append all the remaining operators from stack to postfix string
        while !stack.is_empty() {
            postfix = format!("{}{}", postfix, stack.pop().unwrap());
        }

        postfix
    }


    /**
     * Evaluate two numbers regarding operator
     *
     * # Arguments
     * * `x` - First number to do evaluation witch
     * * `y` - Second number to do evaluation with
     * * `operator` - Operator (+, -, *, /) to evaluate `x` and `y` with
     *
     * # Return
     * Returns the result of the evaluation
     *
     * # Example
     * ```
     * assert_eq!(evaluate(5, 2, '+'), 7);
     * assert_eq!(evaluate(2, 2, '*'), 4);
     * assert_eq!(evaluate(5, 10, '-'), -5);
     * assert_eq!(evaluate(0, -5, '-'), 5);
     * ```
     *
     */
    fn evaluate(x: f64, y: f64, operator: char) -> f64 {
        match operator {
            '+' => x + y,
            '-' => x - y,
            '*' => x * y,
            '/' => x / y,
            _ => 0.00
        } 
    }

    /**
     * Calculate the result of an infix string (`s` gets converted to postfix inside this
     * function)
     *
     *
     * # Return
     * Returns the result as a double.
     *
     * # Example
     * ```
     * assert_eq!(calculate("5+5"), 10);
     * assert_eq!(calculate("5+5*2"), 15);
     * assert_eq!(calculate("5-20/2"), -5);
     * ```
     */
    pub fn calculate(s: String) -> f64 {
       
        // Convert to postfix
        let s = to_postfix(s);

        // Stack for holding operators and nr for grouping digits who belong together as a number
        let mut stack : std::vec::Vec<f64> = std::vec::Vec::new();
        let mut nr : String = "".to_string();

        let mut i = 0;
        while i < s.len() {
            if s.ch(i) == ' ' {
                i = i + 1;
                continue;
            } 
            
            // If evaluated character is a digit, put it in nr
            if s.ch(i).is_digit(10) {
                // If digit is first in a group of digits (AKA a number), put that
                // whole number in nr
                
                while s.ch(i).is_digit(10) {
                    nr = format!("{}{}", nr, s.ch(i));
                    i = i + 1;
                }

                // Pushing nr to stack
                stack.push(f64::from_str(&nr).unwrap());
                nr = "".to_string();
            } else {
                // If current evaluated character is not a digit
                // but an operator, do a calculation
                
                // Retrieve first number of calculation
                let x : f64 = stack.pop().unwrap();
                let y : f64 = stack.pop().unwrap();

                // Put evaluation result in integer and push into stack
                let result : f64 = evaluate(y, x, s.ch(i));
                stack.push(result);
            }

            i = i + 1;
        }

        // Final number is in stack
        *stack.last().unwrap()
    }


    // Testing this module.
    #[cfg(test)]
    mod tests {
        use super::*;

        // We have made a custom implementation to retrieve a n-th
        // character from a string. Let's test if it works correctly here.
        #[test]
        fn test_string_subscription() {
            let my_str : String = "abc123".to_string();

            assert_eq!(my_str.ch(0), 'a');
            assert_eq!(my_str.ch(1), 'b');
            assert_eq!(my_str.ch(2), 'c');
            assert_eq!(my_str.ch(3), '1');
            assert_eq!(my_str.ch(4), '2');
            assert_eq!(my_str.ch(5), '3');
        }

        #[test]
        fn test_is_operator() {
            assert_eq!(is_operator('+'), true);
            assert_eq!(is_operator('-'), true);
            assert_eq!(is_operator('*'), true);
            assert_eq!(is_operator('/'), true);
            assert_eq!(is_operator('a'), false);
            assert_eq!(is_operator('1'), false);
            assert_eq!(is_operator('&'), false);
        }

        #[test]
        fn test_precedence() {
            assert!(precedence('*') > precedence('+'));
            assert!(precedence('/') > precedence('+'));
            assert!(precedence('*') > precedence('-'));
            assert!(precedence('/') > precedence('-'));
            assert!(precedence('/') == precedence('*'));
            assert!(precedence('-') == precedence('-'));
        }

        #[test]
        fn test_to_postfix() {
            assert_eq!(to_postfix("1+1".to_string()), "1 1 +".to_string());
            assert_eq!(to_postfix("5+5-1".to_string()), "5 5 + 1 -".to_string());
            assert_eq!(to_postfix("-1+5".to_string()), "0 1 - 5 +".to_string());
            assert_eq!(to_postfix("-1".to_string()), "0 1 -".to_string());
            assert_eq!(to_postfix("2*2/3-1+3".to_string()), "2 2 * 3 / 1 - 3 +".to_string());
            assert_eq!(to_postfix("0*3/4*2".to_string()), "0 3 * 4 / 2 *".to_string());
            //assert_eq!(to_postfix("-5+10*-5".to_string()), "-5 10 -5 * +".to_string());
        }

        #[test]
        fn test_evaluate() {
            assert_eq!(evaluate(5.0, 5.0, '+'), 10.0);
            assert_eq!(evaluate(5.0, 15.0, '-'), -10.0);
            assert_eq!(evaluate(5.0, 5.0, '*'), 25.0);
            assert_eq!(evaluate(5.0, 5.0, '/'), 1.0);
            assert_eq!(evaluate(5.0, 5.0, '-'), 0.0);
            assert_eq!(evaluate(23030.0, 93939.0, '+'), 116969.0);
        }

        //Most important test
        #[test]
        fn test_calculate() {
            assert_eq!(calculate("5+5".to_string()), 10.0);
            assert_eq!(calculate("5-5".to_string()), 0.0);
            assert_eq!(calculate("5-10".to_string()), -5.0);
            assert_eq!(calculate("5*5".to_string()), 25.0);
            assert_eq!(calculate("5/5".to_string()), 1.0);
            assert_eq!(calculate("-5*2".to_string()), -10.0);
            assert_eq!(calculate("-5*2".to_string()), -10.0);
            //assert_eq!(calculate("-5+10*-5".to_string()), -55.0);
            assert_eq!(calculate("5+5+5+5".to_string()), 20.0);
            assert_eq!(calculate("5+5-5+5+5+5-5-5-5-5".to_string()), 0.0);
            assert_eq!(calculate("5*4/2+10-5".to_string()), 15.0);
            assert_eq!(calculate("5/2".to_string()), 2.5);
            assert_eq!(calculate("5+5-2-2+5*2*2*4-4/5+6".to_string()), 91.2);
            assert_eq!(calculate("5+5*2-2/4".to_string()), 14.5);
            assert_eq!(calculate("578873873*322/2222+32932-323232-222+28032".to_string()), 83624722.9189919);
            assert_eq!(calculate("0".to_string()), 0.0);
            assert_eq!(calculate("1".to_string()), 1.0);
            assert_eq!(calculate("-1".to_string()), -1.0);
        }
    }

}



// Including OrbTk
use orbtk::*;
use orbtk::theme::DEFAULT_THEME_CSS;

// Including file containing css for a dark theme
static DARK_THEME: &'static str = include_str!("styling.css");

// Helper function to get the correct theme
fn get_theme() -> ThemeValue {
    ThemeValue::create_from_css(DEFAULT_THEME_CSS)
        .extension_css(DARK_THEME)
        .build()
}

// MainView is our widget
widget!(MainView<MainViewState>);

// Enumeration of possible Actions to execute when a button is clicked
#[derive(Copy, Clone)]
enum Action {
    Char(char),
}

// Our state of the MainView
#[derive(AsAny, Default)]
pub struct MainViewState {
    screen: String,
    action: Option<Action>,
}

impl MainViewState {
    // Setting the correct action
    fn action(&mut self, action: impl Into<Option<Action>>) { //?
        self.action = action.into();
    }
}

impl State for MainViewState {
    // Updating to the correct state (e.g add digit to screen). Update
    // regarding the executed action, and char.
    fn update(&mut self, _: &mut Registry, ctx: &mut Context) {
        if let Some(action) = self.action {
            match action {
                Action::Char(c) => match c {
                    '=' => {
                        // When user presses '='-button we need to display the result after
                        // calculating it
                        let mut screen = ctx.child("screen");
                        let screen_text : String = screen.get_mut::<String16>("text").as_string();
                        let result : f64 = logic::calculate(screen_text.to_string());
                       
                        // Clearing screen
                        ctx.child("screen").get_mut::<String16>("text").clear();
               
                        // Pushing the result to the screen
                        for c in result.to_string().chars() {
                            ctx.child("screen").get_mut::<String16>("text").push(c);
                        }
                    },
                    'C' => {
                        // When user presses the Clear-button, we clear the screen
                        ctx.child("screen").get_mut::<String16>("text").clear();
                    },
                    _ => {
                        // Otherwise a digit is pressed. Push digit to screen.
                        ctx.child("screen").get_mut::<String16>("text").push(c);
                    },
                }
            }

            // Reset action
            self.action = None;
        }
    }
}

// helper to request MainViewState
fn state<'a>(id: Entity, states: &'a mut StatesContext) -> &'a mut MainViewState {
    states.get_mut(id)
}

// Here all the widgets get added
impl Template for MainView {
    fn template(self, id: Entity, ctx: &mut BuildContext) -> Self {
        
        // Initializing Grid
        let mut grid = Grid::create(); 

        // Configuring grid (amount of rows and columns)
        grid = grid
            .columns(
                Columns::create()
                    .column("*")
                    .column("*")
                    .column("*")
                    .column("*")
                    .build()
            )
            .rows(
                Rows::create()
                    .row(50.0)
                    .row(50.0)
                    .row(50.0)
                    .row(50.0)
                    .row(50.0)
                    .row(50.0)
                    .row(50.0)
                    .build()
            );
       
        //Adding textbox holding entered numbers and operators
        grid = grid.child(
            TextBox::create()
                .text("")
                .id("screen")
                .attach(Grid::row(0))
                .attach(Grid::column(0))
                .attach(Grid::column_span(4))
                .build(ctx)
        );

        // Adding all buttons from 1-9 to the grid
        // in calculator format
        let mut counter : u8 = 9;
        for i in 1..4 {
            for j in 0..3 {
                grid = grid.child(
                    Button::create()
                        .text(counter.to_string())
                        .class("digit_btn")
                        .attach(Grid::column(2-j))
                        .attach(Grid::row(i))
                        .on_click({
                            move |states, _| -> bool {
                                state(id, states).action(Action::Char(std::char::from_digit(counter as u32, 10).unwrap()));
                                true
                            }
                        })
                        .build(ctx)
                );

                counter = counter - 1;
            }
        }

        // Adding +, -, x, /
        let operators : Vec<char> = vec!['+', '-', '*', '/', '='];
        let mut i = 1;
        for operator in operators {
                grid = grid.child(
                    Button::create()
                        .text(operator.to_string())
                        .class("action_btn")
                        .attach(Grid::column(3))
                        .attach(Grid::row(i))
                        .on_click({
                            move |states, _| -> bool {
                                state(id, states).action(Action::Char(operator));
                                true
                            }
                        })
                        .build(ctx)
                );

                i = i + 1;
        }

        // Adding zero-button (Seperate because of special column-span)
        grid = grid.child(
            Button::create()
                .text("0")
                .class("digit_btn")
                .attach(Grid::column(0))
                .attach(Grid::row(4))
                .attach(Grid::column_span(3))
                .on_click({
                    move |states, _| -> bool {
                        state(id, states).action(Action::Char(std::char::from_digit(0, 10).unwrap()));
                        true
                    }
                }).build(ctx)
        );

        // Adding clear-button
        grid = grid.child(
            Button::create()
                .text("C")
                .class("action_btn")
                .attach(Grid::column(0))
                .attach(Grid::row(5))
                .attach(Grid::column_span(3))
                .on_click({
                    move |states, _| -> bool {
                        state(id, states).action(Action::Char('C'));
                        true
                    }
                }).build(ctx)
        );

        self.name("MainView").child(
            grid.build(ctx)
        )

    }
}

fn main() {

    Application::new()
        .window(|ctx| {
            Window::create()
                .title("CALCULATOR_GITSALAH")
                .position((100.0, 100.0))
                .size(212.0, 336.0)
                .theme(get_theme())
                .child(MainView::create().build(ctx))
                .build(ctx)
        })
        .run();
}