use get_full_year::get_full_year;

#[tokio::test]
async fn test_get_full_year() {
    let result = get_full_year(false).await;
    assert!(result.is_ok());
    let data = result.unwrap();
    assert!(data.year > 0);
}

#[tokio::test]
async fn test_get_full_year_enterprise() {
    let result = get_full_year(true).await;
    assert!(result.is_ok());
    let data = result.unwrap();
    assert!(data.year > 0);
}