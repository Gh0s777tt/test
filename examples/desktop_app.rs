//! Desktop Application Example
//! 
//! This example demonstrates how to create a simple desktop application
//! using VantisOS Flux graphics framework and Classic Shell.

use vantis::verified::flux::window::Window;
use vantis::verified::flux::renderer::Renderer;
use vantis::verified::flux::widget::{Button, Label, TextBox};
use vantis::userspace::ui::shells::classic::ClassicShell;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("VantisOS Desktop Application Example");
    println!("=====================================\n");
    
    // Initialize Classic Shell desktop environment
    let shell = ClassicShell::new()?;
    println!("Classic Shell initialized");
    
    // Create main window
    let mut window = Window::new("VantisOS Calculator", 400, 500)?;
    window.set_position(100, 100)?;
    window.set_resizable(false)?;
    println!("Main window created");
    
    // Initialize renderer
    let renderer = Renderer::new(&window)?;
    
    // Create UI widgets
    // Display label
    let display = Label::new("0", 20, 20, 360, 50)?;
    display.set_font_size(32)?;
    display.set_alignment("right")?;
    window.add_widget(Box::new(display))?;
    
    // Number buttons (0-9)
    let button_positions = [
        (20, 100, "7"), (130, 100, "8"), (240, 100, "9"),
        (20, 170, "4"), (130, 170, "5"), (240, 170, "6"),
        (20, 240, "1"), (130, 240, "2"), (240, 240, "3"),
        (20, 310, "0"),
    ];
    
    for (x, y, label) in button_positions.iter() {
        let button = Button::new(label, *x, *y, 100, 50)?;
        button.set_click_handler(move || {
            println!("Button {} clicked", label);
            // Add button click logic here
        })?;
        window.add_widget(Box::new(button))?;
    }
    
    // Operation buttons
    let op_positions = [
        (350, 100, "/"), (350, 170, "*"), (350, 240, "-"),
        (350, 310, "+"), (350, 380, "="),
    ];
    
    for (x, y, op) in op_positions.iter() {
        let button = Button::new(op, *x, *y, 40, 50)?;
        button.set_color(100, 149, 237)?; // Cornflower blue
        button.set_click_handler(move || {
            println!("Operation {} clicked", op);
            // Add operation logic here
        })?;
        window.add_widget(Box::new(button))?;
    }
    
    // Clear button
    let clear = Button::new("C", 130, 380, 100, 50)?;
    clear.set_color(220, 20, 60)?; // Crimson
    clear.set_click_handler(|| {
        println!("Clear button clicked");
        // Clear display logic here
    })?;
    window.add_widget(Box::new(clear))?;
    
    // Show window
    window.show()?;
    println!("Window displayed");
    
    // Start main event loop
    println!("Starting desktop application...\n");
    shell.run_event_loop()?;
    
    Ok(())
}

// Calculator logic
struct Calculator {
    display: String,
    current_value: f64,
    previous_value: f64,
    operation: Option<char>,
    new_number: bool,
}

impl Calculator {
    fn new() -> Self {
        Calculator {
            display: "0".to_string(),
            current_value: 0.0,
            previous_value: 0.0,
            operation: None,
            new_number: true,
        }
    }
    
    fn press_number(&mut self, digit: char) {
        if self.new_number {
            self.display = digit.to_string();
            self.new_number = false;
        } else {
            self.display.push(digit);
        }
        self.current_value = self.display.parse().unwrap_or(0.0);
    }
    
    fn press_operation(&mut self, op: char) {
        self.operation = Some(op);
        self.previous_value = self.current_value;
        self.new_number = true;
    }
    
    fn calculate(&mut self) {
        if let Some(op) = self.operation {
            match op {
                '+' => self.current_value = self.previous_value + self.current_value,
                '-' => self.current_value = self.previous_value - self.current_value,
                '*' => self.current_value = self.previous_value * self.current_value,
                '/' => {
                    if self.current_value != 0.0 {
                        self.current_value = self.previous_value / self.current_value;
                    }
                }
                _ => {}
            }
            self.display = self.current_value.to_string();
            self.operation = None;
            self.new_number = true;
        }
    }
    
    fn clear(&mut self) {
        self.display = "0".to_string();
        self.current_value = 0.0;
        self.previous_value = 0.0;
        self.operation = None;
        self.new_number = true;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_calculator_addition() {
        let mut calc = Calculator::new();
        calc.press_number('5');
        calc.press_operation('+');
        calc.press_number('3');
        calc.calculate();
        assert_eq!(calc.current_value, 8.0);
    }
    
    #[test]
    fn test_calculator_multiplication() {
        let mut calc = Calculator::new();
        calc.press_number('4');
        calc.press_operation('*');
        calc.press_number('6');
        calc.calculate();
        assert_eq!(calc.current_value, 24.0);
    }
    
    #[test]
    fn test_calculator_clear() {
        let mut calc = Calculator::new();
        calc.press_number('9');
        calc.clear();
        assert_eq!(calc.display, "0");
    }
}