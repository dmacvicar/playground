

// str_findchr(const char *start, const char *end, int c)
// 
// const char * str_findstr(const char*start, const char*end, const char *needle)
//
// bool str_startswith(const char*start, const char*end, const char*prefix)
// static bool Whitespace(int c)


// static bool NameStartChar (int)

pub enum Result {
    ErrorXmlInvalid,
    Success,
    ErrorBufferDry,
    ErrorTokensFull,
}

/*
 Unlike most XML parsers, SXML does not use SAX callbacks or allocate a DOM tree.
 Instead you will have to interpret the XML structure through a table of tokens.

 A token can describe any of the following types:
*/
pub enum TokenType {
	StartTag,	/* Start tag describes the opening of an XML element */
	EndTag,	/* End tag is the closing of an XML element */
	Character,		/* Character data may be escaped - check if the first character is an ampersand '&' to identity a XML character reference */
	CData,			/* Character data should be read as is - it is not escaped */

  /* And some other token types you might be interested in: */
	Instruction,	/* Can be used to identity the text encoding */
	DocType,		/* If you'd like to interpret DTD data */
	Comment		/* Most likely you don't care about comments - but this is where you'll find them */
}

/*
 If you are familiar with the structure of an XML document most of these type names should sound familiar.
 A token has the following data:
*/
pub struct Token {
	/* A token is one of the above sxmltype_t */
  token_type: TokenType,
  /* The following number of tokens contain additional data related to this token -
     used for describing attributes */
  size: uint,
  /* 'startpos' and 'endpos' together define a range within the provided text buffer
   * use these offsets with the buffer to extract the text value of the token */
	startpos: uint,
  endpos: uint,
}

/*
 Let's walk through how to correctly interpret a token of type SXML_STARTTAG.

 <example zero='' one='Hello there!' three='Me, Myself &amp; I' />

 The element name ('example') can be extracted from the text buffer using 'startpos' and 'endpos'.

 The attributes of the XML element are described in the following 'size' tokens.
 Each attribute is divided by a token of type SXML_CDATA - this is the attribute key.
 There will be zero or more tokens of type SXML_CHARACTER following the key - together they describe one attribute value.

 In our example you will get the following number of SXML_CHARACTER tokens after the attribute key:
 * 'zero' will use no tokens to describe the empty attribute value.
 * 'one' will have one token describing the attribute value ('Hello there!').
 * 'three' will have three tokens describing the attribute value ('Me, Myself ')('&amp;')(' I')

 In our example the token of type SXML_STARTTAG will have a 'size' of 7 (3 SXML_CDATA and 4 SXML_CHARACTER).
 When processing the tokens do not forget about 'size' - for any token you want to skip, also remember to skip the additional token data!
*/

pub struct Parser {
    /* Current offset into buffer - all XML data before this position has been successfully parsed */
    bufferpos: uint,
    /* Number of tokens filled with valid data by the parser */
    ntokens: uint,
    /* Used internally - keeps track of number of unclosed XML elements to detect start and end of document */
    taglevel: uint,
}

impl Parser {

    fn new() {
        Parser {bufferpos: 0, ntokens: 0, taglevel: 0}
    }
}

#[cfg(test)]
mod test {

    #[test]
    fn test_parser() {
        let parser = Parser:new();
        
    }
}


