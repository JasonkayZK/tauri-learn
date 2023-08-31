// Return a Result<String, ()> to bypass the borrowing issue
#[tauri::command]
pub async fn async_command(value: &str, bound: i32) -> Result<String, ()> {
    // Call another async function and wait for it to finish
    let res = some_async_function(bound).await;

    // Note that the return value must be wrapped in `Ok()` now.
    Ok(format!("{}, res: {}", value, res))
}

async fn some_async_function(bound: i32) -> i32 {
    let mut res = 0;
    for i in 0..=bound {
        res += i;
    }
    res
}
