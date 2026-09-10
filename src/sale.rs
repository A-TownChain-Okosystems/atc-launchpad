// Copyright (c) 2026 Michael Wroblewski — Apache-2.0
//! Sale-Accounting mit Hard-Cap und Mindest-Beitrag (LAUNCHPAD-Spezifikationen, MVP).

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SaleError {
    HardCapExceeded,
    BelowMinimum,
}

#[derive(Debug, Clone)]
pub struct SaleConfig {
    pub hard_cap: u128,
    pub min_contribution: u128,
}

pub struct Sale {
    config: SaleConfig,
    total: u128,
    contributions: Vec<(u64, u128)>,
}

impl Sale {
    pub fn new(config: SaleConfig) -> Self {
        Sale { config, total: 0, contributions: Vec::new() }
    }

    pub fn contribute(&mut self, contributor: u64, amount: u128) -> Result<(), SaleError> {
        if amount < self.config.min_contribution {
            return Err(SaleError::BelowMinimum);
        }
        if self.total + amount > self.config.hard_cap {
            return Err(SaleError::HardCapExceeded);
        }
        self.total += amount;
        self.contributions.push((contributor, amount));
        Ok(())
    }

    pub fn total(&self) -> u128 {
        self.total
    }

    pub fn contributors(&self) -> usize {
        self.contributions.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cap_und_minimum() {
        let mut s = Sale::new(SaleConfig { hard_cap: 100, min_contribution: 10 });
        assert_eq!(s.contribute(1, 10), Ok(()));
        assert_eq!(s.contribute(2, 90), Ok(()));
        assert_eq!(s.contribute(3, 1), Err(SaleError::BelowMinimum));
        assert_eq!(s.contribute(3, 50), Err(SaleError::HardCapExceeded));
        assert_eq!(s.total(), 100);
        assert_eq!(s.contributors(), 2);
    }

    #[test]
    fn exakt_bis_cap() {
        let mut s = Sale::new(SaleConfig { hard_cap: 100, min_contribution: 1 });
        for i in 0..100 {
            assert_eq!(s.contribute(i, 1), Ok(()));
        }
        assert_eq!(s.contribute(999, 1), Err(SaleError::HardCapExceeded));
        assert_eq!(s.total(), 100);
    }
}
