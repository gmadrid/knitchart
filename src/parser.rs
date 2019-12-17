/*
  Comment   -> '//' .* EOL
  Document  -> Header Body

  Header    -> Attrs EOL
  Attrs     -> Attr EOL Attrs
  Attr      -> AttrName '=' Value
  AttrName  -> IDENT
  Value     -> IDENT
  
  Body      -> StartBody Chart EndBody
  StartBody -> 'CHART' .* EOL
  EndBody   -> 'OSAAT' .* EOL
  Chart     -> (KnitChar | PurlChar | EmptyChar)* EOL
    
  A Comment must start at the beginning of a line.
  Spaces and TABs are always ignored except as token separators.
  KnitChar and PurlChar are defined in the Header.
  Empty lines are ignored in the Header. 
  Short lines will always be padded with KnitChar in the Chart.


  Attributes:
  
  rows = number of rows in the chart. If missing, then uses the number of rows in the Chart
      section. Missing lines will be padded with KnitChar.
  columns = number of stitches in a row. If missing, it will be padded with KnitChar.
  knit = an ASCII character to represent a Knit in the Chart. Default = '.'
  purl = an ASCII character to represent a Purl in the Chart. Default = '*'
  empty = an ASCII character to represent an empty cell in the Chart. (Neither Knit nor Perl.) Default = SPACE

  Special attribute values:

  SPACE, BLANK = a space char, ' '.
  DOT = a bullet char 
  CIRCLE = a hollow bullet char

 */

#[cfg(test)]
mod test {
    #[test]
    fn foobar() {
	
    }
}
