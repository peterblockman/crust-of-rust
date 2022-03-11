use std::fmt::Debug;

#[derive(Debug)]
pub struct StrSplit<'haystack, 'delimiter> {
    remainder: Option< &'haystack str>,
    delimiter: &'delimiter str
}

// '_ anonymous lifetime are places you tell compiler to guess lifetime
// where there is only only one possible
// but we use '_ here give error due to lifetime error
// the ref is valid for the lifetime "_" 
// StrSplit has '_, but remainder has the same lifetime with StrSplit
// the complier does not know haystack lives for as long as '_
// problem: no relationship between '_ and haystack and deliieter 
// solution: add 'a 
impl<'haystack, 'delimiter> StrSplit<'haystack, 'delimiter>{
    pub fn new(haystack: &'haystack str, delimiter: &'delimiter str) -> Self {
       Self {
           remainder: Some(haystack),
           delimiter,
       }
    }
}
// why we need 'a in impl: this impl block is generic over 'a
impl<'haystack, 'delimiter> Iterator for StrSplit<'haystack, 'delimiter> {
    type Item = &'haystack str;

    fn next(&mut self) -> Option<Self::Item> {
        // if we do Some(remainder) = self.remainder the remainder on the left will own self.remainder
        // so we dont want that. We want to ref to the value inside the self.remainder
        // ref mut: self.remainder Option<&'a str>, ref mute make type of the remainder on the left 
        // to be &mut &'a str so that we can modify below
        // if we dont have ref mut keyword the remainder on the left will be &'a str
        // ref: reference of the thing that I want to match rather than the thing itself
        // if we write Some(&mut remainder) = self.remainder, this mean take what on the right and try to match 
        // it against the left pattern. Some(&mut remainder) is Some(&mut T), so the remainder will be assigned to T

        /*My Summary
            Some(remainder) = self.remainder makes the remainder on the left to own self.remainder 
            Some(&mut remainder) = self.remainder means take what on the right and try to match against the left.
            Some(ref mut remainder) = self.remainder we ref to the value inside self.remainder
         */
        /* 
            if let Some(ref mut remainder) = self.remainder

            equals  let  remainder = self.remainder? (shorthand)
        */
        let remainder = self.remainder.as_mut()?;
        if let Some(next_delim) = remainder.find(self.delimiter) {
            let until_delimiter = &remainder[..next_delim];
            /* dereference remainder to store new value */
            *remainder = &remainder[(next_delim + self.delimiter.len())..];
            Some(until_delimiter)
        } else {
            self.remainder.take()
            // if the Option is None return None
            // otherwise set the Option to None and return Some in there
        }
    }
}

pub fn until_char<'s>(s: &'s str, c: char) -> &'s str {
    let delim = format!("{}", c);
    // delim lifetime ends after this function
    // it use lifetime of delim (shorter) to the returned str

    StrSplit::new(s,&delim)
    .next()
    .expect("StrSplit always gives at least one Result")
}
#[test]
fn it_works(){
    let haystack = "a b c d";
    let letters = StrSplit::new(haystack, " ");
    assert!(letters.eq(vec!["a", "b", "c", "d"].into_iter()));
}




#[test]
fn until_char_test(){
    assert_eq!(until_char("hello world", 'o'), "hello")
}

#[test]
fn tail(){
    let haystack = "a b c d ";
    let letters = StrSplit::new(haystack, " ");
    assert!(letters.eq(vec!["a", "b", "c", "d", ""].into_iter()));
}