// Put here as this code breaks rustfmt
//
// TODO: The system identifier and public identifier strings must be compared to the values given in the lists above in an ASCII case-insensitive manner
pub(super) fn quirks_check(
    name: &str,
    public_id: &str,
    system_id: &str,
    is_force_quirks: bool,
    system_id_present: bool,
) -> bool {
    is_force_quirks
        || name != "html"
        || public_id == "-//W3O//DTD W3 HTML Strict 3.0//EN//"
        || public_id == "-/W3C/DTD HTML 4.0 Transitional/EN"
        || public_id == "HTML"
        || system_id == "http://www.ibm.com/data/dtd/v11/ibmxhtml1-transitional.dtd"
        || public_id.starts_with("+//Silmaril//dtd html Pro v0r11 19970101//")
        || public_id.starts_with("-//AS//DTD HTML 3.0 asWedit + extensions//")
        || public_id.starts_with("-//AdvaSoft Ltd//DTD HTML 3.0 asWedit + extensions//")
        || public_id.starts_with("-//IETF//DTD HTML 2.0 Level 1//")
        || public_id.starts_with("-//IETF//DTD HTML 2.0 Level 2//")
        || public_id.starts_with("-//IETF//DTD HTML 2.0 Strict Level 1//")
        || public_id.starts_with("-//IETF//DTD HTML 2.0 Strict Level 2//")
        || public_id.starts_with("-//IETF//DTD HTML 2.0 Strict//")
        || public_id.starts_with("-//IETF//DTD HTML 2.0//")
        || public_id.starts_with("-//IETF//DTD HTML 2.1E//")
        || public_id.starts_with("-//IETF//DTD HTML 3.0//")
        || public_id.starts_with("-//IETF//DTD HTML 3.2 Final//")
        || public_id.starts_with("-//IETF//DTD HTML 3.2//")
        || public_id.starts_with("-//IETF//DTD HTML 3//")
        || public_id.starts_with("-//IETF//DTD HTML Level 0//")
        || public_id.starts_with("-//IETF//DTD HTML Level 1//")
        || public_id.starts_with("-//IETF//DTD HTML Level 2//")
        || public_id.starts_with("-//IETF//DTD HTML Level 3//")
        || public_id.starts_with("-//IETF//DTD HTML Strict Level 0//")
        || public_id.starts_with("-//IETF//DTD HTML Strict Level 1//")
        || public_id.starts_with("-//IETF//DTD HTML Strict Level 2//")
        || public_id.starts_with("-//IETF//DTD HTML Strict Level 3//")
        || public_id.starts_with("-//IETF//DTD HTML Strict//")
        || public_id.starts_with("-//IETF//DTD HTML//")
        || public_id.starts_with("-//Metrius//DTD Metrius Presentational//")
        || public_id.starts_with("-//Microsoft//DTD Internet Explorer 2.0 HTML Strict//")
        || public_id.starts_with("-//Microsoft//DTD Internet Explorer 2.0 HTML//")
        || public_id.starts_with("-//Microsoft//DTD Internet Explorer 2.0 Tables//")
        || public_id.starts_with("-//Microsoft//DTD Internet Explorer 3.0 HTML Strict//")
        || public_id.starts_with("-//Microsoft//DTD Internet Explorer 3.0 HTML//")
        || public_id.starts_with("-//Microsoft//DTD Internet Explorer 3.0 Tables//")
        || public_id.starts_with("-//Netscape Comm. Corp.//DTD HTML//")
        || public_id.starts_with("-//Netscape Comm. Corp.//DTD Strict HTML//")
        || public_id.starts_with("-//O'Reilly and Associates//DTD HTML 2.0//")
        // This line breaks rustfmt...
        || public_id.starts_with("-//SoftQuad Software//DTD HoTMetaL PRO 6.0::19990601::extensions to HTML 4.0//")
        || public_id.starts_with("-//O'Reilly and Associates//DTD HTML Extended 1.0//")
        || public_id.starts_with("-//O'Reilly and Associates//DTD HTML Extended Relaxed 1.0//")
        || public_id.starts_with("-//SQ//DTD HTML 2.0 HoTMetaL + extensions//")
        || public_id
            .starts_with("-//SoftQuad//DTD HoTMetaL PRO 4.0::19971010::extensions to HTML 4.0//")
        || public_id.starts_with("-//Spyglass//DTD HTML 2.0 Extended//")
        || public_id.starts_with("-//Sun Microsystems Corp.//DTD HotJava HTML//")
        || public_id.starts_with("-//Sun Microsystems Corp.//DTD HotJava Strict HTML//")
        || public_id.starts_with("-//W3C//DTD HTML 3 1995-03-24//")
        || public_id.starts_with("-//W3C//DTD HTML 3.2 Draft//")
        || public_id.starts_with("-//W3C//DTD HTML 3.2 Final//")
        || public_id.starts_with("-//W3C//DTD HTML 3.2//")
        || public_id.starts_with("-//W3C//DTD HTML 3.2S Draft//")
        || public_id.starts_with("-//W3C//DTD HTML 4.0 Frameset//")
        || public_id.starts_with("-//W3C//DTD HTML 4.0 Transitional//")
        || public_id.starts_with("-//W3C//DTD HTML Experimental 19960712//")
        || public_id.starts_with("-//W3C//DTD HTML Experimental 970421//")
        || public_id.starts_with("-//W3C//DTD W3 HTML//")
        || public_id.starts_with("-//W3O//DTD W3 HTML 3.0//")
        || public_id.starts_with("-//WebTechs//DTD Mozilla HTML 2.0//")
        || public_id.starts_with("-//WebTechs//DTD Mozilla HTML//")
        || (!system_id_present && public_id.starts_with("-//W3C//DTD HTML 4.01 Frameset//"))
        || (!system_id_present && public_id.starts_with("-//W3C//DTD HTML 4.01 Transitional//"))
}


pub(super) fn limited_quirks_check(
    public_id: &str,
    system_id_present: bool,
) -> bool {
    public_id.starts_with("-//W3C//DTD XHTML 1.0 Frameset//")
        || public_id.starts_with("-//W3C//DTD XHTML 1.0 Transitional//")
        || (system_id_present && public_id.starts_with("-//W3C//DTD HTML 4.01 Frameset//"))
        || (system_id_present && public_id.starts_with("-//W3C//DTD HTML 4.01 Transitional//"))
}
