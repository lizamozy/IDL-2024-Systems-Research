# Spring 2024 Research

The goal of this project was to create an Interface Definition Language for the microkennel composite. More specifically, I needed to take C headerfiles and parse them to put them into structs and serliaze those structs into components to then be put into a readable human TOML format. While I did not acheive all of my goals I learned a lot about a the programming language Rust and how the treesitter library works.

# The Rust Programming Language

The big appeal about rust is that it is very secure and efficeint language that is different from the object oritented programming language Java and different from convential langauges used for systems programming like C. For instance, the way that variables are created and how they are passed into fucntions creates a very secure devloping environment. Moreover, the principles of moving and borrowing enable rust to remove dangling pointers which are a great concern for security vulnerabilties. Another cool thing about rust is the static typed component. An example of this is the `Option` type that allows a user to use the defined type or null. The rust compiler sees this and will give errors during compile time rather than give dreaded runtime errors. 

# Tree Sitter 
