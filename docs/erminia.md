# Erminia Description

Erminia is a domain specific language for describing an ARC AGI grid example problem.
The purpose of the language is to create a formatted manner in which to describe these problems using 
abstractions for shapes in the grid.

## Language

### Lexical Units 

---

> **__Keywords:__**

    "def"   "object"    "superobject"    "shape"    "color"

    "example"   "test"   "input"    "output"

> **__Identifiers:__**

    {Letter}({Letter}|{Digit})*

> **__Symbolic Operators:__**
    
    =

> **__Separators:__**

    (   )   [   ]   {   }   ,   :   ;

> **__Comments:__**
    
    (*  [INSERT COMMENT HERE]  *)


---

### Complete ARC-Script Grammar

~~~

 1. <program>              ::= <problem_declaration>
 
 2. <problem_declaration>  ::= "def" <id> "(" [<int_const>] ")" <compound_stmt>
 
 3. <compound_stmt>        ::= "{" (<stmt>)* "}"
 
 4. <stmt>                 ::= <object_def> 
                             | <example_def> 
                             | <test_def> 
                             | <func_call> 
                             | <var_def>
 
 5. <object_def>           ::= "object" <id> <object_desc>
 
 6. <object_desc>          ::= "{" <object_prior_decl> "}"
 
 7. <object_prior_decl>    ::= <shape_decl> "," <color_decl> 
                             | <color_decl> "," <shape_decl>

 
 8. <shape_decl>           ::= "shape" ":" <shapes_list>
 
 9. <shapes_list>          ::= "[" <shape_desc> ("," <shape_desc>)* "]"

10. <shape_desc>           ::= <coordinates_list>

11. <coordinates_list>     ::= "[" <tuples_list> "]"

12. <tuples_list>          ::= <tuple> ("," <tuple>)*  
                             | <tuple_compr>

13. <tuple>                ::= <tuple_prior> "," <tuple_prior>

14. <tuple_prior>          ::= <int_const> 
                             | <coordinate_prior>

15. <coordinate_prior>    ::= "x" | "y"

16. <tuple_compr>         ::= <tuple> "|" <tuple_iter>

17. <tuple_iter>          ::= <coordinate_iter> ["," <coordinate_iter>]

18. <coordinate_iter>     ::= <coordinate_prior> "<-" "[" <int_const> ".." <int_const> "]"

19. <color_desc>          ::= "color" ":" <color_const>

20. <example_def>         ::= "example" <id> <inner_compound_stmt>

21. <test_def>            ::= "test" <id> <inner_compound_stmt>

22. <func_call>           ::= <id> "(" [<expr_list>] ")" ";"

23. <expr_list>           ::= <expr> ("," <expr>)

24. <var_def>             ::= <data_type> <id> "=" <object_call> ";"

25. <data_type>           ::= "object" 
                            | "superobject"

26. <object_call>         ::= <id> "(" ([<offset_x> ["," <offset_y>] | <offset_y>]) ")" 

27. <offset_x>            ::= "offset_x" ":" <int_const>

28. <offset_y>            ::= "offset_y" ":" <int_const>

29. <expr>                ::= 

~~~


<!-- 20. \<input_decl> ::= **"input"** \<id> **"("** <int_const> **","** <int_const> **")"** <compound_stmt> -->
<!-- 21. \<output_decl> ::= **"output"** \<id> **"("** <int_const> **","** <int_const> **")"** <compound_stmt> -->
<!-- 22. \<func_decl> ::= **"func"** \<id> **"("** [<fpar_list>] **")"** [**"->"** \<rtype>] <compound_stmt> -->
<!-- 23. \<fpar_list> ::= <fpar_def> (**","** <fpar_def>)* -->
<!-- 24. \<fpar_def> ::= \<type> \<id> -->
<!-- 25. \<type> ::= **"object"** | **"color"** | **"int"** | **"string"** -->
<!-- 26. \<rtype> ::= \<type> | **"void"** -->
<!-- 27. \<func_call> ::= \<id> **"("** [<fop_list>] **")"** **";"** -->
<!-- 28. \<fop_list> ::= \<type> **":"** \<expr> (**","** \<type> **":"** \<expr>)* -->
<!-- 28. \<expr> ::= <int_const> | \<id> | \<expr> (**"+"** | **"-"**) \<expr> -->
<!-- 29. \<include_call> ::= **"include"** \<id> **";"** -->

### Compiler's Course Guide

[Compiler's Course Guide](https://courses.softlab.ntua.gr/compilers/2024a/#lectures)





















    
