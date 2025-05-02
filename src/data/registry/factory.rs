use crate::data::registry::traits::{AnyRegisterLoader, RegisterLoader};
use crate::data::schema::traits::RegistrySchema;
use crate::error::{IdsError, Result};
use std::collections::HashMap;
use std::path::Path;

use super::loaders::akm::AkmRegister;
use super::loaders::bef::BefRegister;
use super::loaders::dod::DodRegister;
use super::loaders::dodsaarsag::DodsaarsagRegister;
use super::loaders::idan::IdanRegister;
use super::loaders::ind::IndRegister;
use super::loaders::lpr::{Lpr2Register, Lpr3Register};
use super::loaders::mfr::MfrRegister;
use super::loaders::uddf::UddfRegister;
use super::loaders::vnds::VndsRegister;

/// Factory for creating registry loaders with any schema type
pub struct RegistryFactory;

impl RegistryFactory {
    /// Create a registry loader from a registry name
    pub fn from_name(name: &str) -> Result<AnyRegisterLoader> {
        match name.to_lowercase().as_str() {
            "akm" => Ok(Box::new(AkmRegister)),
            "bef" => Ok(Box::new(BefRegister)),
            "dod" => Ok(Box::new(DodRegister)),
            "dodsaarsag" => Ok(Box::new(DodsaarsagRegister)),
            "idan" => Ok(Box::new(IdanRegister)),
            "ind" => Ok(Box::new(IndRegister)),
            "lpr2" => Ok(Box::new(Lpr2Register)),
            "lpr3" => Ok(Box::new(Lpr3Register)),
            "mfr" => Ok(Box::new(MfrRegister)),
            "uddf" => Ok(Box::new(UddfRegister)),
            "vnds" => Ok(Box::new(VndsRegister)),
            _ => Err(IdsError::Validation(format!("Unknown registry: {name}"))),
        }
    }

    /// Create a registry loader from a path (inferring the registry type from the path)
    pub fn from_path(path: &Path) -> Result<AnyRegisterLoader> {
        let path_str = path.to_string_lossy().to_lowercase();

        // Try to infer registry from directory name
        if path_str.contains("akm") {
            return Ok(Box::new(AkmRegister));
        } else if path_str.contains("bef") {
            return Ok(Box::new(BefRegister));
        } else if path_str.contains("dod") && !path_str.contains("dodsaarsag") {
            return Ok(Box::new(DodRegister));
        } else if path_str.contains("dodsaarsag") {
            return Ok(Box::new(DodsaarsagRegister));
        } else if path_str.contains("idan") {
            return Ok(Box::new(IdanRegister));
        } else if path_str.contains("ind") && !path_str.contains("idan") {
            return Ok(Box::new(IndRegister));
        } else if path_str.contains("lpr_") || path_str.contains("lpr2") {
            return Ok(Box::new(Lpr2Register));
        } else if path_str.contains("lpr3") {
            return Ok(Box::new(Lpr3Register));
        } else if path_str.contains("mfr") {
            return Ok(Box::new(MfrRegister));
        } else if path_str.contains("uddf") {
            return Ok(Box::new(UddfRegister));
        } else if path_str.contains("vnds") {
            return Ok(Box::new(VndsRegister));
        }

        // If we can't infer from the path, return an error
        Err(IdsError::Validation(format!(
            "Could not determine registry type from path: {}",
            path.display()
        )))
    }

    /// Create instances of all available loaders, mapped by name
    #[must_use] pub fn create_all() -> HashMap<&'static str, AnyRegisterLoader> {
        let mut loaders = HashMap::<&'static str, AnyRegisterLoader>::new();
        
        // Add registry loaders
        let akm = AkmRegister;
        loaders.insert(akm.register_name(), Box::new(akm) as AnyRegisterLoader);
        
        let bef = BefRegister;
        loaders.insert(bef.register_name(), Box::new(bef) as AnyRegisterLoader);
        
        let dod = DodRegister;
        loaders.insert(dod.register_name(), Box::new(dod) as AnyRegisterLoader);
        
        let dodsaarsag = DodsaarsagRegister;
        loaders.insert(dodsaarsag.register_name(), Box::new(dodsaarsag) as AnyRegisterLoader);
        
        let idan = IdanRegister;
        loaders.insert(idan.register_name(), Box::new(idan) as AnyRegisterLoader);
        
        let ind = IndRegister;
        loaders.insert(ind.register_name(), Box::new(ind) as AnyRegisterLoader);
        
        let lpr2 = Lpr2Register;
        loaders.insert(lpr2.register_name(), Box::new(lpr2) as AnyRegisterLoader);
        
        let lpr3 = Lpr3Register;
        loaders.insert(lpr3.register_name(), Box::new(lpr3) as AnyRegisterLoader);
        
        let mfr = MfrRegister;
        loaders.insert(mfr.register_name(), Box::new(mfr) as AnyRegisterLoader);
        
        let uddf = UddfRegister;
        loaders.insert(uddf.register_name(), Box::new(uddf) as AnyRegisterLoader);
        
        let vnds = VndsRegister;
        loaders.insert(vnds.register_name(), Box::new(vnds) as AnyRegisterLoader);
        
        loaders
    }
    
    /// Get schema name from the registry loader
    #[must_use] pub fn get_schema_name<S: RegistrySchema>() -> &'static str {
        std::any::type_name::<S>()
    }
}
