use std::sync::{Arc, Mutex};
use std::path::Path;
use rusqlite::{Connection, Result};

use crate::repositories::yeast::YeastRepository;
use crate::repositories::honey::HoneyRepository;
use crate::repositories::hop::HopRepository;
use crate::repositories::malt::MaltRepository;
use crate::repositories::fruit::FruitRepository;
use crate::repositories::vegetable::VegetableRepository;
use crate::repositories::spice::SpiceRepository;
use crate::repositories::herb::HerbRepository;
use crate::repositories::extract::ExtractRepository;
use crate::repositories::syrup::SyrupRepository;
use crate::repositories::adjunct::AdjunctRepository;
use crate::repositories::water_profile::WaterProfileRepository;
use crate::repositories::water_salt::WaterSaltRepository;
use crate::repositories::acid::AcidRepository;
use crate::repositories::nutrient::NutrientRepository;
use crate::repositories::enzyme::EnzymeRepository;
use crate::repositories::bacteria::BacteriaRepository;
use crate::repositories::tannin::TanninRepository;
use crate::schemas;

pub struct DatabaseManager {
    user_db: Arc<Mutex<Connection>>,
    encyclopedia_db: Arc<Mutex<Connection>>,
}

impl DatabaseManager {
    pub fn new<P: AsRef<Path>>(user_db_path: P, encyclopedia_db_path: P) -> Result<Self> {
        let user_db = Connection::open(user_db_path)?;
        let encyclopedia_db = Connection::open(encyclopedia_db_path)?;

        user_db.execute_batch("PRAGMA foreign_keys = ON;")?;
        encyclopedia_db.execute_batch("PRAGMA foreign_keys = ON;")?;

        let manager = Self {
            user_db: Arc::new(Mutex::new(user_db)),
            encyclopedia_db: Arc::new(Mutex::new(encyclopedia_db)),
        };

        manager.initialize_schemas()?;
        Ok(manager)
    }

    fn initialize_schemas(&self) -> Result<()> {
        let encyclopedia_db = self.encyclopedia_db.lock().unwrap();
        schemas::init_all_tables(&encyclopedia_db)?;
        Ok(())
    }

    pub fn with_yeast_repo<T, F>(&self, f: F) -> Result<T>
    where
        F: FnOnce(YeastRepository) -> Result<T>,
    {
        let conn = self.encyclopedia_db.lock().unwrap();
        let repo = YeastRepository::new(&conn);
        f(repo)
    }

    pub fn with_honey_repo<T, F>(&self, f: F) -> Result<T>
    where
        F: FnOnce(HoneyRepository) -> Result<T>,
    {
        let conn = self.encyclopedia_db.lock().unwrap();
        let repo = HoneyRepository::new(&conn);
        f(repo)
    }

    pub fn with_hop_repo<T, F>(&self, f: F) -> Result<T>
    where
        F: FnOnce(HopRepository) -> Result<T>,
    {
        let conn = self.encyclopedia_db.lock().unwrap();
        let repo = HopRepository::new(&conn);
        f(repo)
    }

    pub fn with_malt_repo<T, F>(&self, f: F) -> Result<T>
    where
        F: FnOnce(MaltRepository) -> Result<T>,
    {
        let conn = self.encyclopedia_db.lock().unwrap();
        let repo = MaltRepository::new(&conn);
        f(repo)
    }

    pub fn with_fruit_repo<T, F>(&self, f: F) -> Result<T>
    where
        F: FnOnce(FruitRepository) -> Result<T>,
    {
        let conn = self.encyclopedia_db.lock().unwrap();
        let repo = FruitRepository::new(&conn);
        f(repo)
    }

    pub fn with_vegetable_repo<T, F>(&self, f: F) -> Result<T>
    where
        F: FnOnce(VegetableRepository) -> Result<T>,
    {
        let conn = self.encyclopedia_db.lock().unwrap();
        let repo = VegetableRepository::new(&conn);
        f(repo)
    }

    pub fn with_spice_repo<T, F>(&self, f: F) -> Result<T>
    where
        F: FnOnce(SpiceRepository) -> Result<T>,
    {
        let conn = self.encyclopedia_db.lock().unwrap();
        let repo = SpiceRepository::new(&conn);
        f(repo)
    }

    pub fn with_herb_repo<T, F>(&self, f: F) -> Result<T>
    where
        F: FnOnce(HerbRepository) -> Result<T>,
    {
        let conn = self.encyclopedia_db.lock().unwrap();
        let repo = HerbRepository::new(&conn);
        f(repo)
    }

    pub fn with_extract_repo<T, F>(&self, f: F) -> Result<T>
    where
        F: FnOnce(ExtractRepository) -> Result<T>,
    {
        let conn = self.encyclopedia_db.lock().unwrap();
        let repo = ExtractRepository::new(&conn);
        f(repo)
    }

    pub fn with_syrup_repo<T, F>(&self, f: F) -> Result<T>
    where
        F: FnOnce(SyrupRepository) -> Result<T>,
    {
        let conn = self.encyclopedia_db.lock().unwrap();
        let repo = SyrupRepository::new(&conn);
        f(repo)
    }

    pub fn with_adjunct_repo<T, F>(&self, f: F) -> Result<T>
    where
        F: FnOnce(AdjunctRepository) -> Result<T>,
    {
        let conn = self.encyclopedia_db.lock().unwrap();
        let repo = AdjunctRepository::new(&conn);
        f(repo)
    }

    pub fn with_water_profile_repo<T, F>(&self, f: F) -> Result<T>
    where
        F: FnOnce(WaterProfileRepository) -> Result<T>,
    {
        let conn = self.encyclopedia_db.lock().unwrap();
        let repo = WaterProfileRepository::new(&conn);
        f(repo)
    }

    pub fn with_water_salt_repo<T, F>(&self, f: F) -> Result<T>
    where
        F: FnOnce(WaterSaltRepository) -> Result<T>,
    {
        let conn = self.encyclopedia_db.lock().unwrap();
        let repo = WaterSaltRepository::new(&conn);
        f(repo)
    }

    pub fn with_acid_repo<T, F>(&self, f: F) -> Result<T>
    where
        F: FnOnce(AcidRepository) -> Result<T>,
    {
        let conn = self.encyclopedia_db.lock().unwrap();
        let repo = AcidRepository::new(&conn);
        f(repo)
    }

    pub fn with_nutrient_repo<T, F>(&self, f: F) -> Result<T>
    where
        F: FnOnce(NutrientRepository) -> Result<T>,
    {
        let conn = self.encyclopedia_db.lock().unwrap();
        let repo = NutrientRepository::new(&conn);
        f(repo)
    }

    pub fn with_enzyme_repo<T, F>(&self, f: F) -> Result<T>
    where
        F: FnOnce(EnzymeRepository) -> Result<T>,
    {
        let conn = self.encyclopedia_db.lock().unwrap();
        let repo = EnzymeRepository::new(&conn);
        f(repo)
    }

    pub fn with_bacteria_repo<T, F>(&self, f: F) -> Result<T>
    where
        F: FnOnce(BacteriaRepository) -> Result<T>,
    {
        let conn = self.encyclopedia_db.lock().unwrap();
        let repo = BacteriaRepository::new(&conn);
        f(repo)
    }

    pub fn with_tannin_repo<T, F>(&self, f: F) -> Result<T>
    where
        F: FnOnce(TanninRepository) -> Result<T>,
    {
        let conn = self.encyclopedia_db.lock().unwrap();
        let repo = TanninRepository::new(&conn);
        f(repo)
    }
}