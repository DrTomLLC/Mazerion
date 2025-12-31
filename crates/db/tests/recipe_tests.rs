// Recipe repository tests

use mazerion_db::*;
use mazerion_core::Result;
use rusqlite::Connection;
use rust_decimal::Decimal;

fn setup_test_db() -> Result<Connection> {
    let conn = Connection::open_in_memory()
        .map_err(|e| mazerion_core::Error::DatabaseError(format!("Failed to open: {}", e)))?;

    create_user_schema(&conn)?;
    create_recipes_master_schema(&conn)?;
    Ok(conn)
}

#[test]
fn test_create_user_recipe() -> Result<()> {
    let conn = setup_test_db()?;
    let repo = RecipeRepository::new(&conn);

    let recipe = Recipe {
        id: None,
        name: "Traditional Mead".to_string(),
        category: "mead".to_string(),
        subcategory: Some("traditional".to_string()),
        description: Some("Classic honey mead".to_string()),
        author: None,
        source: None,
        difficulty: None,
        batch_size_l: Decimal::from_str_exact("19.0").unwrap(),
        target_og: Some(Decimal::from_str_exact("1.120").unwrap()),
        target_fg: Some(Decimal::from_str_exact("1.010").unwrap()),
        target_abv: Some(Decimal::from_str_exact("14.5").unwrap()),
        created_at: String::new(),
        updated_at: String::new(),
    };

    let id = repo.create_user_recipe(&recipe)?;
    assert!(id > 0);

    let fetched = repo.get_user_recipe(id)?;
    assert!(fetched.is_some());
    let fetched = fetched.unwrap();
    assert_eq!(fetched.name, "Traditional Mead");
    assert_eq!(fetched.category, "mead");
    assert_eq!(fetched.batch_size_l, Decimal::from_str_exact("19.0").unwrap());

    Ok(())
}

#[test]
fn test_list_user_recipes() -> Result<()> {
    let conn = setup_test_db()?;
    let repo = RecipeRepository::new(&conn);

    let mead = Recipe {
        id: None,
        name: "Orange Blossom Mead".to_string(),
        category: "mead".to_string(),
        subcategory: None,
        description: None,
        author: None,
        source: None,
        difficulty: None,
        batch_size_l: Decimal::from_str_exact("19.0").unwrap(),
        target_og: None,
        target_fg: None,
        target_abv: None,
        created_at: String::new(),
        updated_at: String::new(),
    };

    let beer = Recipe {
        id: None,
        name: "IPA".to_string(),
        category: "beer".to_string(),
        subcategory: Some("american".to_string()),
        description: None,
        author: None,
        source: None,
        difficulty: None,
        batch_size_l: Decimal::from_str_exact("20.0").unwrap(),
        target_og: None,
        target_fg: None,
        target_abv: None,
        created_at: String::new(),
        updated_at: String::new(),
    };

    repo.create_user_recipe(&mead)?;
    repo.create_user_recipe(&beer)?;

    let all = repo.list_user_recipes(None, 100)?;
    assert_eq!(all.len(), 2);

    let meads = repo.list_user_recipes(Some("mead"), 100)?;
    assert_eq!(meads.len(), 1);
    assert_eq!(meads[0].name, "Orange Blossom Mead");

    Ok(())
}

#[test]
fn test_update_user_recipe() -> Result<()> {
    let conn = setup_test_db()?;
    let repo = RecipeRepository::new(&conn);

    let mut recipe = Recipe {
        id: None,
        name: "Test Recipe".to_string(),
        category: "mead".to_string(),
        subcategory: None,
        description: Some("Original description".to_string()),
        author: None,
        source: None,
        difficulty: None,
        batch_size_l: Decimal::from_str_exact("19.0").unwrap(),
        target_og: None,
        target_fg: None,
        target_abv: None,
        created_at: String::new(),
        updated_at: String::new(),
    };

    let id = repo.create_user_recipe(&recipe)?;
    recipe.id = Some(id);
    recipe.description = Some("Updated description".to_string());
    recipe.batch_size_l = Decimal::from_str_exact("20.0").unwrap();

    repo.update_user_recipe(&recipe)?;

    let fetched = repo.get_user_recipe(id)?.unwrap();
    assert_eq!(fetched.description, Some("Updated description".to_string()));
    assert_eq!(fetched.batch_size_l, Decimal::from_str_exact("20.0").unwrap());

    Ok(())
}

#[test]
fn test_delete_user_recipe() -> Result<()> {
    let conn = setup_test_db()?;
    let repo = RecipeRepository::new(&conn);

    let recipe = Recipe {
        id: None,
        name: "To Delete".to_string(),
        category: "mead".to_string(),
        subcategory: None,
        description: None,
        author: None,
        source: None,
        difficulty: None,
        batch_size_l: Decimal::from_str_exact("19.0").unwrap(),
        target_og: None,
        target_fg: None,
        target_abv: None,
        created_at: String::new(),
        updated_at: String::new(),
    };

    let id = repo.create_user_recipe(&recipe)?;
    assert!(repo.get_user_recipe(id)?.is_some());

    repo.delete_user_recipe(id)?;
    assert!(repo.get_user_recipe(id)?.is_none());

    Ok(())
}

#[test]
fn test_search_user_recipes() -> Result<()> {
    let conn = setup_test_db()?;
    let repo = RecipeRepository::new(&conn);

    let recipe1 = Recipe {
        id: None,
        name: "Orange Blossom Mead".to_string(),
        category: "mead".to_string(),
        subcategory: None,
        description: Some("Made with orange blossom honey".to_string()),
        author: None,
        source: None,
        difficulty: None,
        batch_size_l: Decimal::from_str_exact("19.0").unwrap(),
        target_og: None,
        target_fg: None,
        target_abv: None,
        created_at: String::new(),
        updated_at: String::new(),
    };

    let recipe2 = Recipe {
        id: None,
        name: "Blackberry Melomel".to_string(),
        category: "mead".to_string(),
        subcategory: None,
        description: Some("Mead with blackberries".to_string()),
        author: None,
        source: None,
        difficulty: None,
        batch_size_l: Decimal::from_str_exact("19.0").unwrap(),
        target_og: None,
        target_fg: None,
        target_abv: None,
        created_at: String::new(),
        updated_at: String::new(),
    };

    repo.create_user_recipe(&recipe1)?;
    repo.create_user_recipe(&recipe2)?;

    let results = repo.search_user_recipes("orange", 100)?;
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].name, "Orange Blossom Mead");

    let results = repo.search_user_recipes("mead", 100)?;
    assert_eq!(results.len(), 2);

    Ok(())
}

#[test]
fn test_recipe_validation() -> Result<()> {
    let conn = setup_test_db()?;
    let repo = RecipeRepository::new(&conn);

    let invalid_name = Recipe {
        id: None,
        name: "".to_string(),
        category: "mead".to_string(),
        subcategory: None,
        description: None,
        author: None,
        source: None,
        difficulty: None,
        batch_size_l: Decimal::from_str_exact("19.0").unwrap(),
        target_og: None,
        target_fg: None,
        target_abv: None,
        created_at: String::new(),
        updated_at: String::new(),
    };

    assert!(repo.create_user_recipe(&invalid_name).is_err());

    let invalid_size = Recipe {
        id: None,
        name: "Test".to_string(),
        category: "mead".to_string(),
        subcategory: None,
        description: None,
        author: None,
        source: None,
        difficulty: None,
        batch_size_l: Decimal::ZERO,
        target_og: None,
        target_fg: None,
        target_abv: None,
        created_at: String::new(),
        updated_at: String::new(),
    };

    assert!(repo.create_user_recipe(&invalid_size).is_err());

    Ok(())
}

#[test]
fn test_master_recipe_operations() -> Result<()> {
    let conn = setup_test_db()?;

    conn.execute(
        "INSERT INTO recipes (id, name, category, subcategory, description, author, source, difficulty,
         batch_size_l, target_og, target_fg, target_abv)
         VALUES (1, 'Master Recipe', 'beer', 'ipa', 'Test description', 'Test Author', 'Test Source', 'intermediate',
         '20.0', '1.060', '1.012', '6.3')",
        [],
    ).map_err(|e| mazerion_core::Error::DatabaseError(format!("{}", e)))?;

    let repo = RecipeRepository::new(&conn);

    let fetched = repo.get_master_recipe(1)?;
    assert!(fetched.is_some());
    let fetched = fetched.unwrap();
    assert_eq!(fetched.name, "Master Recipe");
    assert_eq!(fetched.author, Some("Test Author".to_string()));

    let all = repo.list_master_recipes(None, 100)?;
    assert_eq!(all.len(), 1);

    let beers = repo.list_master_recipes(Some("beer"), 100)?;
    assert_eq!(beers.len(), 1);

    Ok(())
}