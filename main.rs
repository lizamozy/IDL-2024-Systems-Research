//importing tree sitter crate
use tree_sitter::Parser;
use tree_sitter::Tree;
// use tree_sitter::Node;
use tree_sitter::TreeCursor;

// #[derive(Debug)]
// struct ArgField{
//     arg_name: String,
// }
//Struct for funcitons
#[derive(Debug)]
struct Function{
    f_name: String,
    // f_ret_type: String,
    // arg_count: u32,
    args: Vec<String>,
}

fn main() {
    let source_code = r#"
        int foo(int a);
    "#;
    let mut parser = Parser::new();
    parser.set_language(tree_sitter_c::language()).expect("Error loading C grammar");
    let parsed: Tree = parser.parse(source_code, None).unwrap();
    println!("ast: {:#?}", parsed.root_node());

    let root_node = parsed.root_node();
    let mut cursor = root_node.walk();

    let parsed_functions = walk(&mut cursor, source_code);

    println!("Parsed functions: {:#?}", parsed_functions);
}

fn walk(cursor: &mut TreeCursor, source_code: &str) -> Vec<Function> {
    let mut parsed_functions = Vec::new();

    loop {
        print!("in the loop");
        let node = cursor.node();

        // Check if this node is a function declaration
        if node.kind() == "function_declaration" {
            let function_name = get_function_name(cursor, source_code);
            let parameters = get_parameters(cursor, source_code);

            let parsed_function = Function {
                f_name: function_name,
                args: parameters,
            };
            parsed_functions.push(parsed_function);
            continue;
        }

        // Move to the next sibling or child
        if cursor.goto_first_child() {
            continue;
        }

        // If no child, try to move to the next sibling
        while !cursor.goto_next_sibling() {
            // If there are no more siblings, move back up to the parent
            if !cursor.goto_parent() {
                break;
            }
        }

        // If we've reached the root node again, break the loop
        if cursor.node().kind() == "root" {
            break;
        }
    }

    parsed_functions
}


fn get_function_name(cursor: &TreeCursor, source_code: &str) -> String {
    print!("in get function name");
    let node = cursor.node();
    let declarator_node = node.child_by_field_name("declarator").unwrap();
    let identifier_node = declarator_node.child(0).unwrap();
    let _parent_node = identifier_node.parent().unwrap(); // Get the parent node (declarator)
    let start_byte = identifier_node.start_byte();
    let end_byte = identifier_node.end_byte();
    let source_code_bytes = source_code.as_bytes();
    //Get the text using byte offsets
    let cursor_text = std::str::from_utf8(&source_code_bytes[start_byte..end_byte])
        .expect("Invalid UTF-8")
        .to_string();

    cursor_text
}

fn get_parameters(cursor: &mut TreeCursor, source_code: &str) -> Vec<String> {
    print!("in get param");
    let node = cursor.node();
    let parameter_list_node = node.child_by_field_name("parameter_list").unwrap();
    let mut parameters = Vec::new();


    
    for param in parameter_list_node.children(cursor) {
        if param.kind() == "parameter_declaration" {
            if let Some(declarator_node) = param.child_by_field_name("declarator") {
                let identifier_node = declarator_node.child(0).unwrap();

                let start_byte = identifier_node.start_byte();
                let end_byte = identifier_node.end_byte();
                let source_code_bytes = source_code.as_bytes();
                //Get the text using byte offsets
                let cursor_text = std::str::from_utf8(&source_code_bytes[start_byte..end_byte])
                    .expect("Invalid UTF-8")
                    .to_string();
                parameters.push(cursor_text);
            }
        }
    }

    parameters
}
