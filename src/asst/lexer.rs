static TOKEN_NAMES : [(&str, u8);67]  = [
    //Kewords
    ("auto",      0),
    ("break",     1),
    ("case",      2),
    ("char",      3),
    ("const",     4),
    ("continue",  5),
    ("default",   6),
    ("do",        7),
    ("double",    8),
    ("else",      9),
    ("enum",     10),
    ("extern",   11),
    ("float",    12),
    ("for",      13),
    ("goto",     14),
    ("if",       15),
    ("int",      16),
    ("long",     17),
    ("register", 18),
    ("return",   19),
    ("short",    20),
    ("signed",   21),
    ("sizeof",   22),
    ("static",   23),
    ("struct",   24),
    ("switch",   25),
    ("typedef",  26),
    ("union",    27),
    ("unsigned", 28),
    ("void",     29),
    ("volatile", 30),
    ("while",    31),


    //Identifier
    ("identifier", 32),

    //String
    ("string", 33),

    //Arithmatic Operators
    ("plus",        34),
    ("minus",       35),
    ("multiply",    36),
    ("divide",      37),
    ("modulas",     38),
    ("unary_plus",  39),
    ("unary_minus", 40),
    ("increment",   41),
    ("decrement",   42),

    //Relational Operators
    ("less",            43),
    ("greater",         44),
    ("less_equal",      45),
    ("greater_equal",   46),
    ("equal",           47),
    ("not_equal",       48),

    //Logical Operators
    ("logical_and",       49),
    ("logical_or",        50),
    ("logical_not",       51),

    //Bitwise Operators
    //@Incomplete
    ("and",                 52),
    ("or",                  53),
    ("xor",                 54),
    ("first_comment",       55),
    ("leftshift",           56),
    ("rightshift",          57),

    //Other Operators
    ("comma",                    58),
    ("question",                 59),
    ("colon",                    60),
    ("semi_colon",               61),
    ("dot",                      62),
    ("arrow",                    63),
    ("address_of",               64),
    ("derefence",                65),

    //EOF
    ("eof",                66),

];


struct Token {
    token_name : u8,
    attribute : Option<String>,
}


impl Token {
    fn new(token_name : u8, attribute : Option<String>) -> Self {
        Self {
            token_name,
            attribute,
        }
    }
}


struct Scanner {
    content       : Vec<Vec<String>>,
    start_pos     : usize,
    current_pos   : usize,
}


impl Scanner {
    pub fn new(content : Vec<Vec<String>>) -> Self {

        let start_pos     = 0;
        let current_pos   = 0;

        Self { 
            content,
            start_pos,
            current_pos,
        }
    }
}
