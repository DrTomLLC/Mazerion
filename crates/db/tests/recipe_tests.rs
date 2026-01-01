use rusqlite::Connection;
use rust_decimal::Decimal;

use mazerion_db::models::Recipe;
use mazerion_db::repositories::recipe::RecipeRepository;
use mazerion_db::schemas::create_user_schema;

fn setup_test_db() -> anyhow::Result<Connection> {
    let conn = Connection::open_in_memory()?;
    create_user_schema(&conn)?;
    Ok(conn)
}

#[test]
fn test_create_user_recipe() -> anyhow::Result<()> {
    let conn = setup_test_db()?;
    let repo = RecipeRepository::new(&conn);

    let recipe = Recipe {
        id: 0,
        name: "Traditional Mead".to_string(),
        category: "mead".to_string(),
        subcategory: Some("traditional".to_string()),
        description: Some("Classic honey mead".to_string()),
        author: None,
        source: None,
        batch_size_l: Decimal::new(190, 1),
        target_og: Some(Decimal::new(1100, 3)),
        target_fg: Some(Decimal::new(1010, 3)),
        target_abv: Some(Decimal::new(120, 1)),
        created_at: "2025-01-01T00:00:00Z".to_string(),
        updated_at: "2025-01-01T00:00:00Z".to_string(),
    };

    let id = repo.create_user_recipe(&recipe)?;
    assert!(id > 0);

    let retrieved = repo.get_user_recipe(id)?.unwrap();
    assert_eq!(retrieved.name, recipe.name);
    assert_eq!(retrieved.category, recipe.category);

    Ok(())
}

#[test]
fn test_list_user_recipes() -> anyhow::Result<()> {
    let conn = setup_test_db()?;
    let repo = RecipeRepository::new(&conn);

    let mead = Recipe {
        id: 0,
        name: "Mead Recipe".to_string(),
        category: "mead".to_string(),
        subcategory: None,
        description: None,
        author: None,
        source: None,
        batch_size_l: Decimal::new(190, 1),
        target_og: None,
        target_fg: None,
        target_abv: None,
        created_at: "2025-01-01T00:00:00Z".to_string(),
        updated_at: "2025-01-01T00:00:00Z".to_string(),
    };

    let beer = Recipe {
        id: 0,
        name: "Beer Recipe".to_string(),
        category: "beer".to_string(),
        subcategory: None,
        description: None,
        author: None,
        source: None,
        batch_size_l: Decimal::new(190, 1),
        target_og: None,
        target_fg: None,
        target_abv: None,
        created_at: "2025-01-01T00:00:00Z".to_string(),
        updated_at: "2025-01-01T00:00:00Z".to_string(),
    };

    repo.create_user_recipe(&mead)?;
    repo.create_user_recipe(&beer)?;

    let all = repo.list_user_recipes(None, 100)?;
    assert_eq!(all.len(), 2);

    let meads = repo.list_user_recipes(Some("mead"), 100)?;
    assert_eq!(meads.len(), 1);

    Ok(())
}

#[test]
fn test_update_user_recipe() -> anyhow::Result<()> {
    let conn = setup_test_db()?;
    let repo = RecipeRepository::new(&conn);

    let mut recipe = Recipe {
        id: 0,
        name: "Original Name".to_string(),
        category: "mead".to_string(),
        subcategory: None,
        description: None,
        author: None,
        source: None,
        batch_size_l: Decimal::new(190, 1),
        target_og: None,
        target_fg: None,
        target_abv: None,
        created_at: "2025-01-01T00:00:00Z".to_string(),
        updated_at: "2025-01-01T00:00:00Z".to_string(),
    };

    let id = repo.create_user_recipe(&recipe)?;

    recipe.id = id;
    recipe.name = "Updated Name".to_string();
    recipe.batch_size_l = Decimal::from(20);

    repo.update_user_recipe(&recipe)?;

    let retrieved = repo.get_user_recipe(id)?.unwrap();
    assert_eq!(retrieved.name, "Updated Name");
    assert_eq!(retrieved.batch_size_l, Decimal::from(20));

    Ok(())
}

#[test]
fn test_delete_user_recipe() -> anyhow::Result<()> {
    let conn = setup_test_db()?;
    let repo = RecipeRepository::new(&conn);

    let recipe = Recipe {
        id: 0,
        name: "To Delete".to_string(),
        category: "mead".to_string(),
        subcategory: None,
        description: None,
        author: None,
        source: None,
        batch_size_l: Decimal::new(190, 1),
        target_og: None,
        target_fg: None,
        target_abv: None,
        created_at: "2025-01-01T00:00:00Z".to_string(),
        updated_at: "2025-01-01T00:00:00Z".to_string(),
    };

    let id = repo.create_user_recipe(&recipe)?;
    repo.delete_user_recipe(id)?;

    let result = repo.get_user_recipe(id)?;
    assert!(result.is_none());

    Ok(())
}

#[test]
fn test_search_user_recipes() -> anyhow::Result<()> {
    let conn = setup_test_db()?;
    let repo = RecipeRepository::new(&conn);

    let recipe1 = Recipe {
        id: 0,
        name: "Honey Mead".to_string(),
        category: "mead".to_string(),
        subcategory: None,
        description: Some("Sweet honey flavor".to_string()),
        author: None,
        source: None,
        batch_size_l: Decimal::new(190, 1),
        target_og: None,
        target_fg: None,
        target_abv: None,
        created_at: "2025-01-01T00:00:00Z".to_string(),
        updated_at: "2025-01-01T00:00:00Z".to_string(),
    };

    let recipe2 = Recipe {
        id: 0,
        name: "Berry Mead".to_string(),
        category: "mead".to_string(),
        subcategory: None,
        description: Some("Fruity berry notes".to_string()),
        author: None,
        source: None,
        batch_size_l: Decimal::new(190, 1),
        target_og: None,
        target_fg: None,
        target_abv: None,
        created_at: "2025-01-01T00:00:00Z".to_string(),
        updated_at: "2025-01-01T00:00:00Z".to_string(),
    };

    repo.create_user_recipe(&recipe1)?;
    repo.create_user_recipe(&recipe2)?;

    let results = repo.search_user_recipes("honey", 100)?;
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].name, "Honey Mead");

    let results = repo.search_user_recipes("mead", 100)?;
    assert_eq!(results.len(), 2);

    Ok(())
}

#[test]
fn test_recipe_validation() -> anyhow::Result<()> {
    let conn = setup_test_db()?;
    let repo = RecipeRepository::new(&conn);

    let invalid_name = Recipe {
        id: 0,
        name: "".to_string(),
        category: "mead".to_string(),
        subcategory: None,
        description: None,
        author: None,
        source: None,
        batch_size_l: Decimal::new(190, 1),
        target_og: None,
        target_fg: None,
        target_abv: None,
        created_at: "2025-01-01T00:00:00Z".to_string(),
        updated_at: "2025-01-01T00:00:00Z".to_string(),
    };
    assert!(repo.create_user_recipe(&invalid_name).is_err());

    let invalid_size = Recipe {
        id: 0,
        name: "Valid Name".to_string(),
        category: "mead".to_string(),
        subcategory: None,
        description: None,
        author: None,
        source: None,
        batch_size_l: Decimal::ZERO,
        target_og: None,
        target_fg: None,
        target_abv: None,
        created_at: "2025-01-01T00:00:00Z".to_string(),
        updated_at: "2025-01-01T00:00:00Z".to_string(),
    };
    assert!(repo.create_user_recipe(&invalid_size).is_err());

    Ok(())
}

#[test]
fn test_master_recipe_operations() -> anyhow::Result<()> {
    let conn = setup_test_db()?;
    let repo = RecipeRepository::new(&conn);

    let recipe = Recipe {
        id: 0,
        name: "Test Recipe".to_string(),
        category: "mead".to_string(),
        subcategory: None,
        description: None,
        author: None,
        source: None,
        batch_size_l: Decimal::new(190, 1),
        target_og: None,
        target_fg: None,
        target_abv: None,
        created_at: "2025-01-01T00:00:00Z".to_string(),
        updated_at: "2025-01-01T00:00:00Z".to_string(),
    };

    repo.create_user_recipe(&recipe)?;

    let fetched = repo.get_user_recipe(1)?;
    assert!(fetched.is_some());

    let all = repo.list_user_recipes(None, 100)?;
    assert_eq!(all.len(), 1);

    let meads = repo.list_user_recipes(Some("mead"), 100)?;
    assert_eq!(meads.len(), 1);

    Ok(())
}