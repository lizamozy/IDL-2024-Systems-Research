# Spring 2024 Research

The goal of this project was to create an Interface Definition Language for the microkennel composite. More specifically, I needed to take C headerfiles and parse them to put them into structs and serliaze those structs into components to then be put into a readable human TOML format. While I did not acheive all of my goals, I learned a lot about a the programming language Rust and how the treesitter library works.

# The Rust Programming Language

The big appeal about rust is that it is very secure and efficeint language that is different from the object oritented programming language Java and different from convential langauges used for systems programming like C. For instance, the way that variables are created and how they are passed into fucntions creates a very secure devloping environment. Moreover, the principles of moving and borrowing enable rust to remove dangling pointers which are a great concern for security vulnerabilties. Another cool thing about rust is the static typed component. An example of this is the `Option` type that allows a user to use the defined type or null. The rust compiler sees this and will give errors during compile time rather than give dreaded runtime errors. The reasons above are some of many reasons why rust is a great programming langauge to use for systems and non-systems programming tasks. For this research I used rust while implementing the rust bindings of the tree sitter library. 

- cargo

# Tree Sitter 

The tree sitter library is a parser generator tool and an incremental parsing library. Some other capabilities of the library are that it includes AST generation and AST data retrieval. I used this library to parse a C header file into an AST. Then I created a tree search function based on the types of parsed data (`function_identifier`,`declerator`,`parameter_list`). This was in efforts to find the funciton name, arguments, and types of the functions so that it could be stored in structs with all of the function and struct information. I ran into a few issues issues with the treesitter library. One was that while I was learning rust and getting familiar with the rust library, determining how to download tree sitter and actually used the library stumped me. Once I was pointed to the right direction by my research overseer, Gabe Parmer, I implemented tree sitter using the tree sitter crate. Another issue I had was directly accessing the data inside of the nodes of the AST. After a lot of a lot reseraching, I found that the easiest way to access data through the tree sitter AST is use byte offsets of the source code and then use toString to get the data from the nodes. Once I was able to get the data from teh tree sitter nodes I was working on a way to traverse through the tree. While I did not get my walk function to work, in the future I could consider using one of the walk APIs in treesitter. 

- sexpression representation 
- structure of children
- walk api

