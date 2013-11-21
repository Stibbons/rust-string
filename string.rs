use std::ascii;

/* 
 * Simplified string manipulation
 *
 *
 *
 */
pub trait String
{
  /*
   * Convert to lower case
   *
   * @param s string to convert
   * @return string
   */
  fn to_lower_case(s: ~str)-> ~str;
}


impl String for ~str
{
  #[inline]
  fn to_lower_case(s: ~str)-> ~str
  {
    return s.to_ascii().to_lower()
  }

  #[inline]
  impl to_upper_case(s: ~str)-> ~str
  {
    return s.to_ascii().to_upper()
  } 

  #[inline]
  fn to_title_case(s: ~str)
  {
    return ??
  } 

  #[inline]
  fn to_camel_case(s: ~str)
  {
    return ??
  }
  
  #[inline]
  fn to_underscore_case(s: ~str)
  {
    return ??
  }
}
