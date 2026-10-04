use phf::phf_map;
use std::ffi::CStr;
use std::ffi::c_char;

#[unsafe(no_mangle)]
pub extern "C" fn rust_lib_add(left: u64, right: u64) -> u64 {
    left + right
}

/// Lookup HTML entity from a string slice (not NUL-terminated).
///
/// # Safety
///
/// * c_entity must be null or at least size bytes long.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rust_lib_lookup_html_entity(
    c_entity: *const c_char,
    size: usize,
) -> *const c_char {
    if c_entity.is_null() || size == 0 {
        return std::ptr::null();
    }
    let entity: &[u8] = unsafe { std::slice::from_raw_parts(c_entity.cast::<u8>(), size) };

    match ENTITIES.get(entity) {
        Some(val) => val.as_ptr(),
        None => std::ptr::null(),
    }
}

static ENTITIES: phf::Map<&'static [u8], &'static CStr> = phf_map! {
    b"&AElig;" => c"Æ",
    b"&Aacute;" => c"Á",
    b"&Acirc;" => c"Â",
    b"&Agrave;" => c"À",
    b"&Alpha;" => c"Α",
    b"&Aring;" => c"Å",
    b"&Atilde;" => c"Ã",
    b"&Auml;" => c"Ä",
    b"&Beta;" => c"Β",
    b"&Ccedil;" => c"Ç",
    b"&Chi;" => c"Χ",
    b"&Dagger;" => c"‡",
    b"&Delta;" => c"Δ",
    b"&ETH;" => c"Ð",
    b"&Eacute;" => c"É",
    b"&Ecirc;" => c"Ê",
    b"&Egrave;" => c"È",
    b"&Epsilon;" => c"Ε",
    b"&Eta;" => c"Η",
    b"&Euml;" => c"Ë",
    b"&Gamma;" => c"Γ",
    b"&Iacute;" => c"Í",
    b"&Icirc;" => c"Î",
    b"&Igrave;" => c"Ì",
    b"&Iota;" => c"Ι",
    b"&Iuml;" => c"Ï",
    b"&Kappa;" => c"Κ",
    b"&Lambda;" => c"Λ",
    b"&Mu;" => c"Μ",
    b"&Ntilde;" => c"Ñ",
    b"&Nu;" => c"Ν",
    b"&OElig;" => c"Œ",
    b"&Oacute;" => c"Ó",
    b"&Ocirc;" => c"Ô",
    b"&Ograve;" => c"Ò",
    b"&Omega;" => c"Ω",
    b"&Omicron;" => c"Ο",
    b"&Oslash;" => c"Ø",
    b"&Otilde;" => c"Õ",
    b"&Ouml;" => c"Ö",
    b"&Phi;" => c"Φ",
    b"&Pi;" => c"Π",
    b"&Prime;" => c"″",
    b"&Psi;" => c"Ψ",
    b"&Rho;" => c"Ρ",
    b"&Scaron;" => c"Š",
    b"&Sigma;" => c"Σ",
    b"&THORN;" => c"Þ",
    b"&Tau;" => c"Τ",
    b"&Theta;" => c"Θ",
    b"&Uacute;" => c"Ú",
    b"&Ucirc;" => c"Û",
    b"&Ugrave;" => c"Ù",
    b"&Upsilon;" => c"Υ",
    b"&Uuml;" => c"Ü",
    b"&Xi;" => c"Ξ",
    b"&Yacute;" => c"Ý",
    b"&Yuml;" => c"Ÿ",
    b"&Zeta;" => c"Ζ",
    b"&aacute;" => c"á",
    b"&acirc;" => c"â",
    b"&acute;" => c"´",
    b"&aelig;" => c"æ",
    b"&agrave;" => c"à",
    b"&alefsym;" => c"ℵ",
    b"&alpha;" => c"α",
    b"&amp;" => c"&",
    b"&and;" => c"∧",
    b"&ang;" => c"∠",
    b"&apos;" => c"'",
    b"&aring;" => c"å",
    b"&asymp;" => c"≈",
    b"&atilde;" => c"ã",
    b"&auml;" => c"ä",
    b"&bdquo;" => c"„",
    b"&beta;" => c"β",
    b"&brvbar;" => c"¦",
    b"&bull;" => c"•",
    b"&cap;" => c"∩",
    b"&ccedil;" => c"ç",
    b"&cedil;" => c"¸",
    b"&cent;" => c"¢",
    b"&chi;" => c"χ",
    b"&circ;" => c"ˆ",
    b"&clubs;" => c"♣",
    b"&cong;" => c"≅",
    b"&copy;" => c"©",
    b"&crarr;" => c"↵",
    b"&cup;" => c"∪",
    b"&curren;" => c"¤",
    b"&dArr;" => c"⇓",
    b"&dagger;" => c"†",
    b"&darr;" => c"↓",
    b"&deg;" => c"°",
    b"&delta;" => c"δ",
    b"&diams;" => c"♦",
    b"&divide;" => c"÷",
    b"&eacute;" => c"é",
    b"&ecirc;" => c"ê",
    b"&egrave;" => c"è",
    b"&empty;" => c"∅",
    b"&emsp;" => c" ",
    b"&ensp;" => c" ",
    b"&epsilon;" => c"ε",
    b"&equiv;" => c"≡",
    b"&eta;" => c"η",
    b"&eth;" => c"ð",
    b"&euml;" => c"ë",
    b"&euro;" => c"€",
    b"&exist;" => c"∃",
    b"&fnof;" => c"ƒ",
    b"&forall;" => c"∀",
    b"&frac12;" => c"½",
    b"&frac14;" => c"¼",
    b"&frac34;" => c"¾",
    b"&frasl;" => c"⁄",
    b"&gamma;" => c"γ",
    b"&ge;" => c"≥",
    b"&gt;" => c">",
    b"&hArr;" => c"⇔",
    b"&harr;" => c"↔",
    b"&hearts;" => c"♥",
    b"&hellip;" => c"…",
    b"&iacute;" => c"í",
    b"&icirc;" => c"î",
    b"&iexcl;" => c"¡",
    b"&igrave;" => c"ì",
    b"&image;" => c"ℑ",
    b"&infin;" => c"∞",
    b"&int;" => c"∫",
    b"&iota;" => c"ι",
    b"&iquest;" => c"¿",
    b"&isin;" => c"∈",
    b"&iuml;" => c"ï",
    b"&kappa;" => c"κ",
    b"&lArr;" => c"⇐",
    b"&lambda;" => c"λ",
    b"&lang;" => c"〈",
    b"&laquo;" => c"«",
    b"&larr;" => c"←",
    b"&lceil;" => c"⌈",
    b"&ldquo;" => c"\u{201C}",
    b"&le;" => c"≤",
    b"&lfloor;" => c"⌊",
    b"&lowast;" => c"∗",
    b"&loz;" => c"◊",
    b"&lrm;" => c"\u{200E}",
    b"&lsaquo;" => c"‹",
    b"&lsquo;" => c"\u{2018}",
    b"&lt;" => c"<",
    b"&macr;" => c"¯",
    b"&mdash;" => c"—",
    b"&micro;" => c"µ",
    b"&middot;" => c"·",
    b"&minus;" => c"−",
    b"&mu;" => c"μ",
    b"&nabla;" => c"∇",
    b"&nbsp;" => c"\xC2\xA0",
    b"&ndash;" => c"–",
    b"&ne;" => c"≠",
    b"&ni;" => c"∋",
    b"&not;" => c"¬",
    b"&notin;" => c"∉",
    b"&nsub;" => c"⊄",
    b"&ntilde;" => c"ñ",
    b"&nu;" => c"ν",
    b"&oacute;" => c"ó",
    b"&ocirc;" => c"ô",
    b"&oelig;" => c"œ",
    b"&ograve;" => c"ò",
    b"&oline;" => c"‾",
    b"&omega;" => c"ω",
    b"&omicron;" => c"ο",
    b"&oplus;" => c"⊕",
    b"&or;" => c"∨",
    b"&ordf;" => c"ª",
    b"&ordm;" => c"º",
    b"&oslash;" => c"ø",
    b"&otilde;" => c"õ",
    b"&otimes;" => c"⊗",
    b"&ouml;" => c"ö",
    b"&para;" => c"¶",
    b"&part;" => c"∂",
    b"&permil;" => c"‰",
    b"&perp;" => c"⊥",
    b"&phi;" => c"φ",
    b"&pi;" => c"π",
    b"&piv;" => c"ϖ",
    b"&plusmn;" => c"±",
    b"&pound;" => c"£",
    b"&prime;" => c"′",
    b"&prod;" => c"∏",
    b"&prop;" => c"∝",
    b"&psi;" => c"ψ",
    b"&quot;" => c"\"",
    b"&rArr;" => c"⇒",
    b"&radic;" => c"√",
    b"&rang;" => c"〉",
    b"&raquo;" => c"»",
    b"&rarr;" => c"→",
    b"&rceil;" => c"⌉",
    b"&rdquo;" => c"\u{201D}",
    b"&real;" => c"\u{211C}",
    b"&reg;" => c"®",
    b"&rfloor;" => c"⌋",
    b"&rho;" => c"ρ",
    b"&rlm;" => c"\u{200F}",
    b"&rsaquo;" => c"›",
    b"&rsquo;" => c"\u{2019}",
    b"&sbquo;" => c"‚",
    b"&scaron;" => c"š",
    b"&sdot;" => c"⋅",
    b"&sect;" => c"§",
    b"&shy;" => c"\xC2\xAD",
    b"&sigma;" => c"σ",
    b"&sigmaf;" => c"ς",
    b"&sim;" => c"∼",
    b"&spades;" => c"♠",
    b"&sub;" => c"⊂",
    b"&sube;" => c"⊆",
    b"&sum;" => c"∑",
    b"&sup1;" => c"¹",
    b"&sup2;" => c"²",
    b"&sup3;" => c"³",
    b"&sup;" => c"⊃",
    b"&supe;" => c"⊇",
    b"&szlig;" => c"ß",
    b"&tau;" => c"τ",
    b"&there4;" => c"∴",
    b"&theta;" => c"θ",
    b"&thetasym;" => c"ϑ",
    b"&thinsp;" => c" ",
    b"&thorn;" => c"þ",
    b"&tilde;" => c"˜",
    b"&times;" => c"×",
    b"&trade;" => c"™",
    b"&uArr;" => c"⇑",
    b"&uacute;" => c"ú",
    b"&uarr;" => c"↑",
    b"&ucirc;" => c"û",
    b"&ugrave;" => c"ù",
    b"&uml;" => c"¨",
    b"&upsih;" => c"ϒ",
    b"&upsilon;" => c"υ",
    b"&uuml;" => c"ü",
    b"&weierp;" => c"℘",
    b"&xi;" => c"ξ",
    b"&yacute;" => c"ý",
    b"&yen;" => c"¥",
    b"&yuml;" => c"ÿ",
    b"&zeta;" => c"ζ",
    b"&zwj;" => c"\u{200D}",
    b"&zwnj;" => c"\u{200C}",
};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add() {
        let result = rust_lib_add(2, 2);
        assert_eq!(result, 4);
    }

    // Successful lookup. The input slice does not include NUL.
    #[test]
    fn lookup_html_entity_without_nul() -> Result<(), Box<dyn std::error::Error>> {
        let input = c"&Aacute;";
        let want = "Á";
        let cchar = unsafe { rust_lib_lookup_html_entity(input.as_ptr(), input.count_bytes()) }; // exclude NUL byte
        assert!(!cchar.is_null());
        let cstr = unsafe { CStr::from_ptr(cchar) };
        let got = cstr.to_str()?;
        assert_eq!(got, want);
        Ok(())
    }

    // The input slice does not include NUL. Including NUL results in a failed lookup.
    #[test]
    fn lookup_html_entity_with_nul() {
        let input = c"&Aacute;";
        let cchar = unsafe { rust_lib_lookup_html_entity(input.as_ptr(), input.count_bytes() + 1) }; // include NUL byte
        assert!(cchar.is_null());
    }

    #[test]
    fn lookup_html_entity_partial() -> Result<(), Box<dyn std::error::Error>> {
        let input = c"&Aacute;extra stuff";
        let len = 8; // length of "&Aacute;".
        let want = "Á";
        let cchar = unsafe { rust_lib_lookup_html_entity(input.as_ptr(), len) };
        assert!(!cchar.is_null());
        let cstr = unsafe { CStr::from_ptr(cchar) };
        let got = cstr.to_str()?;
        assert_eq!(got, want);
        Ok(())
    }

    #[test]
    fn lookup_html_entity_mismatch() {
        let input = c"&foo;";
        let cchar = unsafe { rust_lib_lookup_html_entity(input.as_ptr(), input.count_bytes()) };
        assert!(cchar.is_null());
    }

    #[test]
    fn lookup_html_entity_null() {
        let cchar = unsafe { rust_lib_lookup_html_entity(std::ptr::null(), 5) };
        assert!(cchar.is_null());
    }

    #[test]
    fn lookup_html_entity_zero() {
        let input = c"&Aacute;";
        let cchar = unsafe { rust_lib_lookup_html_entity(input.as_ptr(), 0) };
        assert!(cchar.is_null());
    }
}
