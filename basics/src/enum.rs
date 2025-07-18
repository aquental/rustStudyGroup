#![allow(unused)]

#[derive(Debug, PartialEq)]
enum Command {
    Play,
    Stop,
    Skip(u32),
    Back(u32),
    Resize { width: u32, height: u32 },
}

//Enum
fn main() {
    let cmd: Command = Command::Play;
    let cmd1 = Command::Stop;
    let cmd2 = Command::Skip(2);
    let cmd3 = Command::Back(2);
    let cmd4 = Command::Resize {
        width: 100,
        height: 200,
    };

    println!("{:?}", cmd);
    println!("{:?}", cmd1);
    println!("{:?}", cmd2);
    println!("{:?}", cmd3);
    println!("{:?}", cmd4);

    println!("{}", cmd == cmd1);

    let x: Option<Command> = Some(Command::Play);
    let y: Option<Command> = None;
    println!("{:?}", x);
    println!("{:?}", y);

    let w: Result<Command, String> = Ok(Command::Play);
    let z: Result<Command, String> = Err("nothing".to_string());
    println!("{:?}", w);
    println!("{:?}", z);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_command_simple_variants() {
        let play = Command::Play;
        let stop = Command::Stop;
        
        // Test that simple variants can be created
        assert_eq!(play, Command::Play);
        assert_eq!(stop, Command::Stop);
        
        // Test that different variants are not equal
        assert_ne!(play, stop);
    }

    #[test]
    fn test_command_tuple_variants() {
        let skip = Command::Skip(5);
        let back = Command::Back(3);
        
        // Test that tuple variants can be created with values
        assert_eq!(skip, Command::Skip(5));
        assert_eq!(back, Command::Back(3));
        
        // Test that same variant with different values are not equal
        assert_ne!(skip, Command::Skip(3));
        assert_ne!(back, Command::Back(5));
        
        // Test that different tuple variants are not equal
        assert_ne!(skip, back);
    }

    #[test]
    fn test_command_struct_variant() {
        let resize = Command::Resize { width: 100, height: 200 };
        
        // Test that struct variant can be created
        assert_eq!(resize, Command::Resize { width: 100, height: 200 });
        
        // Test that struct variant with different values are not equal
        assert_ne!(resize, Command::Resize { width: 200, height: 100 });
        assert_ne!(resize, Command::Resize { width: 100, height: 300 });
        assert_ne!(resize, Command::Resize { width: 150, height: 200 });
    }

    #[test]
    fn test_command_debug_trait() {
        let play = Command::Play;
        let skip = Command::Skip(10);
        let resize = Command::Resize { width: 800, height: 600 };
        
        // Test debug output for simple variant
        assert_eq!(format!("{:?}", play), "Play");
        
        // Test debug output for tuple variant
        assert_eq!(format!("{:?}", skip), "Skip(10)");
        
        // Test debug output for struct variant
        assert_eq!(format!("{:?}", resize), "Resize { width: 800, height: 600 }");
    }

    #[test]
    fn test_command_equality() {
        // Test equality for simple variants
        assert_eq!(Command::Play, Command::Play);
        assert_eq!(Command::Stop, Command::Stop);
        
        // Test equality for tuple variants
        assert_eq!(Command::Skip(5), Command::Skip(5));
        assert_eq!(Command::Back(3), Command::Back(3));
        
        // Test equality for struct variants
        assert_eq!(
            Command::Resize { width: 100, height: 200 },
            Command::Resize { width: 100, height: 200 }
        );
        
        // Test inequality between different variants
        assert_ne!(Command::Play, Command::Stop);
        assert_ne!(Command::Skip(5), Command::Back(5));
        assert_ne!(Command::Play, Command::Skip(1));
        assert_ne!(Command::Skip(1), Command::Resize { width: 1, height: 1 });
    }

    #[test]
    fn test_command_tuple_variants_with_zero() {
        let skip_zero = Command::Skip(0);
        let back_zero = Command::Back(0);
        
        assert_eq!(skip_zero, Command::Skip(0));
        assert_eq!(back_zero, Command::Back(0));
        assert_ne!(skip_zero, back_zero);
    }

    #[test]
    fn test_command_tuple_variants_with_large_values() {
        let skip_large = Command::Skip(u32::MAX);
        let back_large = Command::Back(u32::MAX);
        
        assert_eq!(skip_large, Command::Skip(u32::MAX));
        assert_eq!(back_large, Command::Back(u32::MAX));
        assert_ne!(skip_large, back_large);
    }

    #[test]
    fn test_command_struct_variant_with_zero_dimensions() {
        let resize_zero = Command::Resize { width: 0, height: 0 };
        
        assert_eq!(resize_zero, Command::Resize { width: 0, height: 0 });
        assert_ne!(resize_zero, Command::Resize { width: 1, height: 0 });
        assert_ne!(resize_zero, Command::Resize { width: 0, height: 1 });
    }

    #[test]
    fn test_command_struct_variant_with_large_dimensions() {
        let resize_large = Command::Resize { width: u32::MAX, height: u32::MAX };
        
        assert_eq!(resize_large, Command::Resize { width: u32::MAX, height: u32::MAX });
        assert_ne!(resize_large, Command::Resize { width: u32::MAX - 1, height: u32::MAX });
        assert_ne!(resize_large, Command::Resize { width: u32::MAX, height: u32::MAX - 1 });
    }

    #[test]
    fn test_command_in_option() {
        let some_play = Some(Command::Play);
        let some_skip = Some(Command::Skip(5));
        let some_resize = Some(Command::Resize { width: 100, height: 200 });
        let none_command: Option<Command> = None;
        
        // Test that enum variants work in Option
        assert_eq!(some_play, Some(Command::Play));
        assert_eq!(some_skip, Some(Command::Skip(5)));
        assert_eq!(some_resize, Some(Command::Resize { width: 100, height: 200 }));
        assert_eq!(none_command, None);
        
        // Test inequality
        assert_ne!(some_play, none_command);
        assert_ne!(some_play, some_skip);
    }

    #[test]
    fn test_command_in_result() {
        let ok_play: Result<Command, String> = Ok(Command::Play);
        let ok_skip: Result<Command, String> = Ok(Command::Skip(10));
        let ok_resize: Result<Command, String> = Ok(Command::Resize { width: 300, height: 400 });
        let err_command: Result<Command, String> = Err("error".to_string());
        
        // Test that enum variants work in Result
        assert_eq!(ok_play, Ok(Command::Play));
        assert_eq!(ok_skip, Ok(Command::Skip(10)));
        assert_eq!(ok_resize, Ok(Command::Resize { width: 300, height: 400 }));
        assert_eq!(err_command, Err("error".to_string()));
        
        // Test inequality
        assert_ne!(ok_play, err_command);
        assert_ne!(ok_play, ok_skip);
    }

    #[test]
    fn test_command_pattern_matching() {
        let commands = vec![
            Command::Play,
            Command::Stop,
            Command::Skip(5),
            Command::Back(3),
            Command::Resize { width: 100, height: 200 },
        ];
        
        for cmd in commands {
            match cmd {
                Command::Play => assert_eq!(cmd, Command::Play),
                Command::Stop => assert_eq!(cmd, Command::Stop),
                Command::Skip(n) => {
                    assert_eq!(cmd, Command::Skip(n));
                    assert_eq!(n, 5); // We know this is the value we put in
                }
                Command::Back(n) => {
                    assert_eq!(cmd, Command::Back(n));
                    assert_eq!(n, 3); // We know this is the value we put in
                }
                Command::Resize { width, height } => {
                    assert_eq!(cmd, Command::Resize { width, height });
                    assert_eq!(width, 100); // We know this is the value we put in
                    assert_eq!(height, 200); // We know this is the value we put in
                }
            }
        }
    }

    #[test]
    fn test_command_clone_and_copy_behavior() {
        // Test that Command can be moved and used multiple times through references
        let original = Command::Skip(42);
        let reference = &original;
        let another_reference = &original;
        
        assert_eq!(*reference, Command::Skip(42));
        assert_eq!(*another_reference, Command::Skip(42));
        assert_eq!(*reference, *another_reference);
        
        // Test with struct variant
        let resize_original = Command::Resize { width: 800, height: 600 };
        let resize_ref = &resize_original;
        assert_eq!(*resize_ref, Command::Resize { width: 800, height: 600 });
    }
}
