//! Industry

use std::str::FromStr;

use serde::{Deserialize, Serialize};

use crate::TGError;

/// Industry
///
/// Represents the industry of a company.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Industry {
    MacroStrategy,
    CyclicalIndustry,
    TechnologyMediaTelecom,
    Internet,
    ConsumerIndustry,
    PublicServiceAndMonopoly,
    FinanceAndRealEstate,
    AdvancedManufacturing,
    AgricultureAndAquaculture,
    HealthAndMedicine,
    PowerSystemAndEnergy,
    Electronics,
}

impl FromStr for Industry {
    type Err = TGError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "MacroStrategy" => Ok(Industry::MacroStrategy),
            "CyclicalIndustry" => Ok(Industry::CyclicalIndustry),
            "TechnologyMediaTelecom" => Ok(Industry::TechnologyMediaTelecom),
            "Internet" => Ok(Industry::Internet),
            "ConsumerIndustry" => Ok(Industry::ConsumerIndustry),
            "PublicServiceAndMonopoly" => Ok(Industry::PublicServiceAndMonopoly),
            "FinanceAndRealEstate" => Ok(Industry::FinanceAndRealEstate),
            "AdvancedManufacturing" => Ok(Industry::AdvancedManufacturing),
            "AgricultureAndAquaculture" => Ok(Industry::AgricultureAndAquaculture),
            "HealthAndMedicine" => Ok(Industry::HealthAndMedicine),
            "PowerSystemAndEnergy" => Ok(Industry::PowerSystemAndEnergy),
            "Electronics" => Ok(Industry::Electronics),
            _ => Err(TGError::Parse(value.to_string())),
        }
    }
}
