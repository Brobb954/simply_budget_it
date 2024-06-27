pub async fn delete_handler(Path(id): Path<String>, State(state): State<Arc<AppState>>, Json(budget): Json<Budget>)
    let route = id.as_str();
    match route {
        da => delete_all_budgets(),
    }
}
