#[test]
fn test_join() {
    let base: &crate::Path<crate::Unknown> = crate::Path::new("/home").unwrap();
    let argument: &crate::Path<crate::Unknown> = crate::Path::new("jakub").unwrap();
    let _result: crate::PathBuf<crate::Unknown> = base.join(argument);
}

#[test]
fn test_parent() {
    let input: &crate::Path<crate::Absolute> = crate::Path::new("/home/jakub").unwrap();
    let _result: &crate::Path<crate::Absolute> = input.parent().unwrap();
    // assert_eq!(result.0.to_str().unwrap(), "/home");
}
